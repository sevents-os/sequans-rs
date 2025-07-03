use core::cell::RefCell;

use atat::{AtatCmd, UrcChannel, UrcSubscription, asynch::AtatClient};
use embassy_sync::{
    blocking_mutex::{
        Mutex,
        raw::{CriticalSectionRawMutex, NoopRawMutex},
    },
    signal::Signal,
};
use heapless::String;
use static_cell::StaticCell;

#[cfg(feature = "gm02sp")]
use crate::{
    Reserved,
    command::{
        device::GetClock,
        gnss::{
            GetGnssAssitance, ProgramGnss, SetGnssConfig, UpdateGnssAssitance,
            types::FixSensitivity, urc::GnssFixReady,
        },
    },
};
use crate::{
    command::{
        self, Urc, device, mobile_equipment, mqtt,
        network::{self, types::NetworkRegistrationState},
        nvm, pdp, ssl_tls,
        system_features::{ConfigureCEREGReports, ConfigureCMEErrorReports},
    },
    error::Error,
    types::Bool,
};
use embassy_time::{Duration, Timer, with_timeout};

/// Represents the state of the modem.
///
/// The state is designed to be shared across multiple components of the modem stack,
/// such as the URC (unsolicited result code) handler and any control interface.
struct ModemState {
    reg_state: Mutex<CriticalSectionRawMutex, RefCell<NetworkRegistrationState>>,
    mqtt_connected: Signal<NoopRawMutex, mqtt::urc::Connected>,

    #[cfg(feature = "gm02sp")]
    fix_subscriber: Signal<NoopRawMutex, GnssFixReady>,
}

impl ModemState {
    /// Creates a new `ModemState`.
    const fn new() -> Self {
        Self {
            reg_state: Mutex::new(RefCell::new(NetworkRegistrationState::NotSearching)),
            mqtt_connected: Signal::new(),
            #[cfg(feature = "gm02sp")]
            fix_subscriber: Signal::new(),
        }
    }
}

/// A handle to the modem, providing access to AT command operations and URC subscription handling.
pub struct Modem<'a, AtCl, const N: usize, const L: usize> {
    client: AtCl,
    state: &'a ModemState,
    urc_chan: &'a UrcChannel<Urc, N, L>,
    initialized: bool,
    #[cfg(feature = "gm02sp")]
    update_almanac: bool,
    #[cfg(feature = "gm02sp")]
    update_ephemeris: bool,
}

/// Handles unsolicited result codes (URCs) received from the modem.
///
/// This handler is intended to run as a long-lived task that continuously polls for URC messages
/// and processes them. It is typically launched by calling [`Modem::urc_handler`] followed by
/// `.run().await`.
pub struct UrcHandler<'a, const N: usize, const L: usize> {
    urc_subscription: UrcSubscription<'a, Urc, N, L>,
    state: &'a ModemState,
}

impl<'a, const N: usize, const L: usize> UrcHandler<'a, N, L> {
    /// Runs the URC handler task indefinitely.
    ///
    /// This method should be spawned as a background task alongside other modem activities.
    pub async fn run(&mut self) -> ! {
        loop {
            let msg = self.urc_subscription.next_message_pure().await;
            match msg {
                #[cfg(feature = "gm02sp")]
                command::Urc::GnssFixReady(fix_ready) => {
                    debug!("GNSS fix ready: {:?}", fix_ready);
                    self.state.fix_subscriber.signal(fix_ready);
                }
                command::Urc::MqttConnected(connected) => {
                    debug!("MQTT connected: {:?}", connected);
                    self.state.mqtt_connected.signal(connected);
                }
                command::Urc::MqttDisconnected(disconnected) => {
                    debug!("MQTT disconnected: {:?}", disconnected);
                    // self.state.mqtt_connected.signal(connected);
                }
                command::Urc::MqttMessagePublished(published) => {
                    debug!("MQTT message published: {:?}", published);
                }
                command::Urc::MqttMessageReceived(received) => {
                    debug!("MQTT message received: {:?}", received);
                }
                command::Urc::MqttSubscribed(subscribed) => {
                    debug!("MQTT subscribed: {:?}", subscribed);
                }
                command::Urc::MqttPromptToPublish(prompt) => {
                    debug!("MQTT prompt to publish: {:?}", prompt);
                }
                command::Urc::Shutdown => {
                    debug!("Device shutdown");
                }
                command::Urc::Start => {
                    debug!("Device started");
                }
                command::Urc::CoapConnected(conn) => {
                    debug!("COAP connected: {:?}", conn);
                }
                command::Urc::NetworkRegistrationStatus(status) => {
                    debug!("Network registration status: {:?}", status);
                    self.state.reg_state.lock(|v| {
                        v.replace(status.stat);
                    });
                }
            };
        }
    }
}

impl<'a, AtCl, const N: usize, const L: usize> Modem<'a, AtCl, N, L>
where
    AtCl: AtatClient,
{
    /// Constructs a new `Modem` instance with a client, URC channel, and shared state.
    ///
    /// # Arguments
    ///
    /// - `client`: An AT command client for communicating with the modem.
    /// - `urc_chan`: A reference to the URC channel used to receive asynchronous modem messages.
    ///
    /// This method does not initialize the modem; call [`begin`](Self::begin) to do so.
    pub fn new(client: AtCl, urc_chan: &'a UrcChannel<Urc, N, L>) -> Self {
        static MODEM_STATE_CELL: StaticCell<ModemState> = StaticCell::new();
        let modem_state: &'static ModemState = MODEM_STATE_CELL.init(ModemState::new());
        Self {
            client,
            urc_chan,
            state: modem_state,
            initialized: false,
            #[cfg(feature = "gm02sp")]
            update_almanac: false,
            #[cfg(feature = "gm02sp")]
            update_ephemeris: false,
        }
    }

    /// Creates a new URC handler associated with this modem.
    ///
    /// The URC handler will subscribe to unsolicited messages from the modem and process them,
    /// updating shared state where necessary. The user must run the [`UrcHandler`](UrcHandler) to begin handling messages.
    ///
    /// # Panics
    ///
    /// Panics if the subscription to the URC channel fails (e.g., buffer full or uninitialized).
    pub fn urc_handler(&self) -> UrcHandler<'a, N, L> {
        UrcHandler {
            urc_subscription: self.urc_chan.subscribe().unwrap(),
            state: self.state,
        }
    }

    pub async fn send<Cmd: AtatCmd>(&mut self, cmd: &Cmd) -> Result<Cmd::Response, Error> {
        self.client.send(cmd).await.map_err(|e| e.into())
    }

    /// Initializes the modem by sending basic configuration commands.
    ///
    /// This method must be called once before other modem operations are invoked.
    /// It is safe to call multiple times; subsequent calls will be no-ops.
    ///
    /// - Enables numeric CME error reporting.
    /// - Enables network registration URC reporting.
    pub async fn begin(&mut self) -> Result<(), Error> {
        if self.initialized {
            return Ok(());
        }

        self.send(&ConfigureCMEErrorReports {
            typ: crate::command::system_features::types::CMEErrorReports::Numeric,
        })
        .await?;

        self.send(&ConfigureCEREGReports {
            typ: crate::command::system_features::types::CEREGReports::Enabled,
        })
        .await?;

        self.initialized = true;

        Ok(())
    }

    pub async fn get_operation_mode(&mut self) -> Result<device::types::RAT, Error> {
        let res = self.send(&device::GetOperatingMode).await?;
        Ok(res.rat)
    }

    pub async fn set_opeartion_mode(&mut self, mode: device::types::RAT) -> Result<(), Error> {
        self.send(&device::SetOperatingMode { mode }).await?;
        Ok(())
    }

    pub async fn ping(&mut self) -> Result<(), Error> {
        self.send(&command::AT).await?;
        Ok(())
    }

    pub async fn define_pdp_context(&mut self) -> Result<(), Error> {
        self.send(&pdp::DefinePDPContext {
            cid: 1,
            pdp_type: command::pdp::types::PDPType::IP,
            apn: String::try_from("").unwrap(),
            pdp_addr: String::try_from("").unwrap(),
            d_comp: command::pdp::types::PDPDComp::default(),
            h_comp: command::pdp::types::PDPHComp::default(),
            ipv4_alloc: command::pdp::types::PDPIPv4Alloc::NAS,
            request_type: command::pdp::types::PDPRequestType::NewOrHandover,
            pdp_pcscf_discovery_method: command::pdp::types::PDPPCSCF::Auto,
            for_imcn: Bool::False,
            nslpi: Bool::False,
            secure_pco: Bool::False,
            ipv4_mtu_discovery: Bool::False,
            local_addr_ind: Bool::False,
            non_ip_mtu_discovery: Bool::False,
        })
        .await?;
        Ok(())
    }

    pub async fn set_op_state(
        &mut self,
        mode: mobile_equipment::types::FunctionalMode,
    ) -> Result<(), Error> {
        self.send(&mobile_equipment::SetFunctionality {
            fun: mode,
            rst: None,
        })
        .await?;
        Ok(())
    }

    pub fn get_network_registration_state(&self) -> NetworkRegistrationState {
        self.state.reg_state.lock(|v| v.borrow().clone())
    }
}

impl<'sub, AtCl, const N: usize, const L: usize> Modem<'sub, AtCl, N, L>
where
    AtCl: AtatClient,
{
    /// Connect to the LTE network.
    ///
    /// This function will connect the modem to the LTE network. This function will
    /// block until the modem is attached.
    pub async fn lte_connect(&mut self) -> Result<(), Error> {
        self.set_op_state(mobile_equipment::types::FunctionalMode::Full)
            .await?;

        //  Set the network operator selection to automatic
        self.send(&network::PLMNSelection {
            mode: command::network::types::NetworkSelectionMode::Automatic,
            ..Default::default()
        })
        .await?;

        loop {
            match self.get_network_registration_state() {
                NetworkRegistrationState::RegisteredHome => break,
                NetworkRegistrationState::RegisteredRoaming => break,
                _ => {
                    Timer::after(Duration::from_millis(1000)).await;
                    // let signal = self.send(&GetSignalQuality).await?;
                    // debug!("rssi: {:?}", signal);
                }
            }
        }

        Ok(())
    }

    /// Disconnect from the LTE network.
    ///
    /// This function will disconnect the modem from the LTE network and block until
    /// the network is actually disconnected. After the network is disconnected the
    /// GNSS subsystem can be used.
    pub async fn lte_disconnect(&mut self) -> Result<(), Error> {
        self.set_op_state(command::mobile_equipment::types::FunctionalMode::Minimum)
            .await?;

        while self.get_network_registration_state() != NetworkRegistrationState::NotSearching {
            Timer::after(Duration::from_millis(100)).await;
        }

        Ok(())
    }
}

impl<'sub, AtCl, const N: usize, const L: usize> Modem<'sub, AtCl, N, L>
where
    AtCl: AtatClient,
{
    pub async fn get_time(&mut self) -> Result<device::responses::Clock, Error> {
        // Even with valid assistance data the system clock could be invalid
        let mut clock = self.send(&GetClock).await?;

        if clock.time.0.timestamp().is_zero() {
            debug!("Clock time out of sync, synchronizing");

            // The system clock is invalid, connect to LTE network to sync time
            self.lte_connect().await?;

            // Wait for the modem to synchronize time with the LTE network, try 5 times
            // with a delay of 500ms.
            for _ in 0..5 {
                Timer::after(Duration::from_millis(500)).await;
                clock = self.send(&GetClock).await?;
                if !clock.time.0.timestamp().is_zero() {
                    break;
                }
            }

            self.lte_disconnect().await?;

            if clock.time.0.timestamp().is_zero() {
                return Err(Error::ClockSynchronization);
            }
        };

        Ok(clock)
    }
}

#[cfg(feature = "gm02sp")]
impl<'sub, AtCl, const N: usize, const L: usize> Modem<'sub, AtCl, N, L>
where
    AtCl: AtatClient,
{
    pub async fn set_gnss_config(&mut self, sensitivity: FixSensitivity) -> Result<(), Error> {
        self.send(&SetGnssConfig {
            location_mode: command::gnss::types::LocationMode::OnDeviceLocation,
            fix_sensitivity: sensitivity,
            urc_settings: command::gnss::types::UrcNotificationSetting::Full,
            reserved: Reserved,
            metrics: false.into(),
            acquisition_mode: command::gnss::types::AcquisitionMode::ColdWarmStart,
            early_abort: false.into(),
        })
        .await?;

        Ok(())
    }

    // Check the assistance data in the modem response.
    //
    // This function checks the availability of assistance data in the modem's
    // response. This function also sets a flag if any of the assistance databases
    // should be updated.
    async fn check_assistance_data(&mut self) -> Result<(), Error> {
        use crate::gnss::responses::GnssAsssitance;

        let data = self.send(&GetGnssAssitance).await?;

        self.update_almanac = false;
        self.update_ephemeris = false;

        for GnssAsssitance {
            typ,
            available,
            time_to_update,
            ..
        } in data
        {
            match typ {
                crate::gnss::types::GnssAssitanceType::Almanac => match available {
                    Bool::True => {
                        debug!(
                            "almanace data is available and should be updated within {}",
                            time_to_update
                        );
                        self.update_almanac = time_to_update <= 0;
                    }
                    Bool::False => {
                        debug!("almanace data is not available",);
                        self.update_almanac = true;
                    }
                },
                crate::gnss::types::GnssAssitanceType::RealTimeEphemeris => match available {
                    Bool::True => {
                        debug!(
                            "real-time ephemeris data is available and should be updated within {}",
                            time_to_update
                        );
                        self.update_ephemeris = time_to_update <= 0;
                    }
                    Bool::False => {
                        debug!("real-time ephemerise data is not available",);
                        self.update_ephemeris = true;
                    }
                },
                crate::gnss::types::GnssAssitanceType::PredictedEphemeris => {}
            }
        }

        Ok(())
    }

    /// Update GNSS assistance data when needed.
    ///
    /// This funtion will check if the current real-time ephemeris data is good
    /// enough to get a fast GNSS fix. If not the function will attach to the LTE
    /// network to download newer assistance data.
    pub async fn update_gnss_asistance(&mut self) -> Result<(), Error> {
        self.lte_disconnect().await?;

        // Even with valid assistance data the system clock could be invalid,
        // get_time ensures the device synchronizes the clock first.
        self.get_time().await?;

        // Check the availability of assistance data
        self.check_assistance_data().await?;

        if !self.update_almanac && !self.update_ephemeris {
            return Ok(());
        }

        self.lte_connect().await?;

        if self.update_almanac {
            self.send(&UpdateGnssAssitance {
                typ: command::gnss::types::GnssAssitanceType::Almanac,
            })
            .await?;
        }

        if self.update_ephemeris {
            self.send(&UpdateGnssAssitance {
                typ: command::gnss::types::GnssAssitanceType::RealTimeEphemeris,
            })
            .await?;
        }

        for _ in 0..10 {
            Timer::after(Duration::from_secs(10)).await;
            self.check_assistance_data().await?;
            if !self.update_almanac && !self.update_ephemeris {
                break;
            }
        }

        self.lte_disconnect().await?;

        Ok(())
    }

    pub async fn get_gnss_fix(&mut self) -> Result<GnssFixReady, Error> {
        use embassy_time::TimeoutError;

        self.state.fix_subscriber.reset();

        self.send(&ProgramGnss {
            action: command::gnss::types::ProgramGnssAction::Single,
        })
        .await?;

        match with_timeout(Duration::from_secs(180), self.state.fix_subscriber.wait()).await {
            Ok(fix) => {
                debug!("GNSS fix received: {:?}", fix);
                Ok(fix)
            }
            Err(TimeoutError) => {
                debug!("GNSS fix timed out");

                self.send(&ProgramGnss {
                    action: command::gnss::types::ProgramGnssAction::Stop,
                })
                .await?;

                Err(TimeoutError.into())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UsernamePassword {
    /// Username for broker authentication.
    pub username: String<256>,

    /// Password for broker authentication.
    pub password: String<256>,
}

// TODO: replace enum with dedicated methods.
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum MqttAuth {
    UsernamePassword(UsernamePassword),
    /// The index of the secure profile previously set with the SSL / TLS Security Profile Configuration.
    SecurityProfile(u8),
}

impl<'sub, AtCl, const N: usize, const L: usize> Modem<'sub, AtCl, N, L>
where
    AtCl: AtatClient,
{
    pub async fn mqtt_configure(
        &mut self,
        client_id: &str,
        auth: Option<MqttAuth>,
    ) -> Result<(), Error> {
        let msg = match auth {
            Some(MqttAuth::UsernamePassword(UsernamePassword { username, password })) => {
                &mqtt::Configure {
                    id: 0,
                    client_id,
                    username,
                    password,
                    sp_id: None,
                }
            }
            Some(MqttAuth::SecurityProfile(id)) => &mqtt::Configure {
                id: 0,
                client_id,
                username: String::new(),
                password: String::new(),
                sp_id: Some(id),
            },
            None => &mqtt::Configure {
                id: 0,
                client_id,
                username: String::new(),
                password: String::new(),
                sp_id: None,
            },
        };

        self.send(msg).await?;

        Ok(())
    }

    pub async fn mqtt_connect(&mut self, host: &str, port: Option<u32>) -> Result<(), Error> {
        self.lte_connect().await?;

        self.send(&mqtt::Connect {
            id: 0,
            host,
            port,
            keepalive: None,
        })
        .await?;

        let connected =
            with_timeout(Duration::from_secs(30), self.state.mqtt_connected.wait()).await?;

        match connected.rc {
            mqtt::types::MQTTStatusCode::Success => Ok(()),
            status => {
                error!("MQTT connect error: {:?}", connected.rc);
                Err(Error::MQTT(status))
            }
        }
    }

    pub async fn mqtt_send(
        &mut self,
        topic: &str,
        qos: mqtt::types::Qos,
        data: &[u8],
    ) -> Result<(), Error> {
        debug!("Sending MQTT message");

        self.send(&mqtt::PreparePublish {
            id: 0,
            topic,
            qos: Some(qos),
            length: data.len(),
        })
        .await?;

        debug!("MQTT publish prepared");

        self.send(&mqtt::Publish {
            payload: atat::serde_bytes::Bytes::new(data),
        })
        .await?;

        debug!("MQTT publish Sent");

        Ok(())
    }

    pub async fn mqtt_disconnect(&mut self) -> Result<(), Error> {
        self.send(&mqtt::Disconnect { id: 0 }).await?;
        self.lte_disconnect().await?;
        Ok(())
    }
}

impl<'sub, AtCl, const N: usize, const L: usize> Modem<'sub, AtCl, N, L>
where
    AtCl: AtatClient,
{
    pub async fn nvm_write(
        &mut self,
        data_type: nvm::types::DataType,
        index: u8,
        data: &[u8],
    ) -> Result<(), Error> {
        debug!("Writing to nvm");

        assert!(
            !(0..=4).contains(&index) && !(7..=10).contains(&index),
            "Indexes O to 4 and 7 to 10 are reserved for Sequans's internal use."
        );

        self.send(&nvm::PrepareWrite {
            data_type,
            index,
            size: data.len(),
        })
        .await?;

        debug!("NVM write ready");

        self.send(&nvm::Write {
            data: atat::serde_bytes::Bytes::new(data),
        })
        .await?;

        debug!("NVM written");

        Ok(())
    }
}

impl<'sub, AtCl, const N: usize, const L: usize> Modem<'sub, AtCl, N, L>
where
    AtCl: AtatClient,
{
    /// Configures TLS/SSL security profile for use with e.g. MQTT.
    ///
    /// Certificates first need to be written to NVM (boot persistent).
    pub async fn configure_tls_profile(
        &mut self,
        sp_id: u8,
        ca_cert_id: Option<u8>,
        client_cert_id: Option<u8>,
        client_private_key_id: Option<u8>,
    ) -> Result<(), Error> {
        assert!(
            (1..=6).contains(&sp_id),
            "Security profile index must be between in the range of 1 to 6"
        );

        self.send(&ssl_tls::Configure {
            sp_id,
            version: ssl_tls::types::SslTlsVersion::Tls13,
            cipher_specs: String::new(),
            cert_valid_level: 0b111,
            ca_cert_id: ca_cert_id.into(),
            client_cert_id: client_cert_id.into(),
            client_private_key_id: client_private_key_id.into(),
            psk: String::new(),
            psk_identity: String::new(),
            storage_id: ssl_tls::types::StorageId::NVM,
            resume: ssl_tls::types::Resume::Disabled,
            lifetime: 0,
        })
        .await?;

        Ok(())
    }
}
