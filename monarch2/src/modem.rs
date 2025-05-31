use core::cell::RefCell;

use atat::{UrcChannel, UrcSubscription, asynch::AtatClient};
use embassy_sync::{
    blocking_mutex::{
        Mutex,
        raw::{CriticalSectionRawMutex, NoopRawMutex},
    },
    signal::Signal,
};
use heapless::String;
use static_cell::StaticCell;

use crate::{
    Bool,
    command::{
        self, Urc,
        device::{GetOperatingMode, SetOperatingMode, types::RAT},
        mobile_equipment::{GetSignalQuality, SetFunctionality, types::FunctionalMode},
        mqtt::{self, types::MQTTStatusCode},
        network::{PLMNSelection, types::NetworkRegistrationState},
        pdp::DefinePDPContext,
        system_features::{ConfigureCEREGReports, ConfigureCMEErrorReports},
    },
    error::Error,
};
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
            debug!("received message: {:?}", msg);
            match msg {
                #[cfg(feature = "gm02sp")]
                command::Urc::GnssFixReady(fix_ready) => {
                    info!("GNSS fix ready: {:?}", fix_ready);
                    self.state.fix_subscriber.signal(fix_ready);
                }
                command::Urc::MqttConnected(connected) => {
                    info!("MQTT connected: {:?}", connected);
                    self.state.mqtt_connected.signal(connected);
                }
                command::Urc::MqttDisconnected(disconnected) => {
                    info!("MQTT disconnected: {:?}", disconnected);
                    // self.state.mqtt_connected.signal(connected);
                }
                command::Urc::MqttMessagePublished(published) => {
                    info!("MQTT message published: {:?}", published);
                }
                command::Urc::MqttMessageReceived(received) => {
                    info!("MQTT message received: {:?}", received);
                }
                command::Urc::MqttSubscribed(subscribed) => {
                    info!("MQTT subscribed: {:?}", subscribed);
                }
                command::Urc::Shutdown(shutdown) => {
                    info!("Shutdown: {:?}", shutdown);
                }
                command::Urc::NetworkRegistrationStatus(status) => {
                    info!("Network registration status: {:?}", status);
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

        self.client
            .send(&ConfigureCMEErrorReports {
                typ: crate::command::system_features::types::CMEErrorReports::Numeric,
            })
            .await?;

        self.client
            .send(&ConfigureCEREGReports {
                typ: crate::command::system_features::types::CEREGReports::Enabled,
            })
            .await?;

        self.initialized = true;

        Ok(())
    }

    pub async fn get_operation_mode(&mut self) -> Result<RAT, Error> {
        let res = self.client.send(&GetOperatingMode).await?;
        Ok(res.rat)
    }

    pub async fn set_opeartion_mode(&mut self, mode: RAT) -> Result<(), Error> {
        self.client.send(&SetOperatingMode { mode }).await?;
        Ok(())
    }

    pub async fn ping(&mut self) -> Result<(), Error> {
        self.client.send(&command::AT).await?;
        Ok(())
    }

    pub async fn define_pdp_context(&mut self) -> Result<(), Error> {
        self.client
            .send(&DefinePDPContext {
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

    pub async fn set_op_state(&mut self, mode: FunctionalMode) -> Result<(), Error> {
        self.client
            .send(&SetFunctionality {
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
        self.set_op_state(FunctionalMode::Full).await?;

        //  Set the network operator selection to automatic
        self.client
            .send(&PLMNSelection {
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

                    let signal = self.client.send(&GetSignalQuality).await?;
                    info!("rssi: {:?}", signal);
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

#[cfg(feature = "gm02sp")]
impl<'sub, AtCl, const N: usize, const L: usize> Modem<'sub, AtCl, N, L>
where
    AtCl: AtatClient,
{
    pub async fn set_gnss_config(&mut self, sensitivity: FixSensitivity) -> Result<(), Error> {
        self.client
            .send(&SetGnssConfig {
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
    pub async fn check_assistance_data(&mut self) -> Result<(), Error> {
        let data = self.client.send(&GetGnssAssitance).await?;

        self.update_almanac = false;
        self.update_ephemeris = false;

        match data.almanac.available {
            Bool::True => {
                info!(
                    "Almanace data is available and should be updated within {}",
                    data.almanac.time_to_update
                );
                self.update_almanac = data.almanac.time_to_update <= 0;
            }
            Bool::False => {
                info!("Almanace data is not available",);
                self.update_almanac = true;
            }
        }

        match data.realtime_ephemeris.available {
            Bool::True => {
                info!(
                    "Real-time ephemeris data is available and should be updated within {}",
                    data.realtime_ephemeris.time_to_update
                );
                self.update_ephemeris = data.realtime_ephemeris.time_to_update <= 0;
            }
            Bool::False => {
                info!("ephemerise data is not available",);
                self.update_ephemeris = true;
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

        // Even with valid assistance data the system clock could be invalid
        let mut clock = self.client.send(&GetClock).await?;

        if clock.time.timestamp().is_zero() {
            info!("Clock time out of sync, synchronizing");

            // The system clock is invalid, connect to LTE network to sync time
            self.lte_connect().await?;

            // Wait for the modem to synchronize time with the LTE network, try 5 times
            // with a delay of 500ms.
            for _ in 0..5 {
                Timer::after(Duration::from_millis(500)).await;
                clock = self.client.send(&GetClock).await?;
                if !clock.time.timestamp().is_zero() {
                    break;
                }
            }

            if clock.time.timestamp().is_zero() {
                return Err(Error::ClockSynchronization);
            }
        };

        // Check the availability of assistance data
        self.check_assistance_data().await?;

        if !self.update_almanac && !self.update_ephemeris {
            //     if (lteConnected) {
            //         if (!lteDisconnect()) {
            //             ESP_LOGE(TAG, "Could not disconnect from the LTE network");
            //             return false;
            //         }
            //     }

            return Ok(());
        }

        // if (!lteConnected) {
        //     if (!lteConnect()) {
        //         ESP_LOGE(TAG, "Could not connect to LTE network");
        //         return false;
        //     }
        // }

        if self.update_almanac {
            self.client
                .send(&UpdateGnssAssitance {
                    typ: command::gnss::types::GnssAssitanceType::Almanac,
                })
                .await?;
        }

        if self.update_ephemeris {
            self.client
                .send(&UpdateGnssAssitance {
                    typ: command::gnss::types::GnssAssitanceType::RealTimeEphemeris,
                })
                .await?;
        }

        // if (!modem.gnssGetAssistanceStatus(&rsp) ||
        //     rsp.type != WALTER_MODEM_RSP_DATA_TYPE_GNSS_ASSISTANCE_DATA) {
        //     ESP_LOGE(TAG, "Could not request GNSS assistance status");
        //     return false;
        // }

        // if (!lteDisconnect()) {
        //     ESP_LOGE(TAG, "Could not disconnect from the LTE network");
        //     return false;
        // }

        // return true;
        Ok(())
    }

    pub async fn get_gnss_fix(&mut self) -> Result<GnssFixReady, Error> {
        // Fail if fix in progress?
        self.state.fix_subscriber.reset();

        self.client
            .send(&ProgramGnss {
                action: command::gnss::types::ProgramGnssAction::Single,
            })
            .await?;

        let fix = with_timeout(Duration::from_secs(300), self.state.fix_subscriber.wait()).await?;

        info!("GNSS fix received: {:?}", fix);

        Ok(fix)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UsernamePassword {
    /// Username for broker authentication.
    pub username: String<256>,

    /// Password for broker authentication.
    pub password: String<256>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SecurityProfile {
    /// The index of the secure profile previously set with the SSL / TLS Security Profile Configuration.
    pub id: u8,
}

// TODO: replace enum with dedicated methods.
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum MqttAuth {
    UsernamePassword(UsernamePassword),
    SecurityProfile(SecurityProfile),
}

impl<'sub, AtCl, const N: usize, const L: usize> Modem<'sub, AtCl, N, L>
where
    AtCl: AtatClient,
{
    pub async fn mqtt_configure(&mut self, client_id: &str, auth: MqttAuth) -> Result<(), Error> {
        let msg = match auth {
            MqttAuth::UsernamePassword(UsernamePassword { username, password }) => {
                &mqtt::Configure {
                    id: 0,
                    client_id,
                    username: Some(username),
                    password: Some(password),
                    sp_id: None,
                }
            }
            MqttAuth::SecurityProfile(SecurityProfile { id }) => &mqtt::Configure {
                id: 0,
                client_id,
                username: None,
                password: None,
                sp_id: Some(id),
            },
        };

        self.client.send(msg).await?;

        Ok(())
    }

    pub async fn mqtt_connect(&mut self, host: &str) -> Result<(), Error> {
        self.client
            .send(&mqtt::Connect {
                id: 0,
                host,
                port: None,
                keepalive: None,
            })
            .await?;

        let connected =
            with_timeout(Duration::from_secs(30), self.state.mqtt_connected.wait()).await?;

        if connected.rc != MQTTStatusCode::Success {
            error!("MQTT connect error: {:?}", connected.rc);
        }

        Ok(())
    }

    pub async fn mqtt_send(
        &mut self,
        topic: &str,
        qos: mqtt::types::Qos,
        data: &[u8],
    ) -> Result<(), Error> {
        self.client
            .send(&mqtt::PreparePublish {
                id: 0,
                topic,
                qos: Some(qos),
                length: data.len(),
            })
            .await?;

        self.client
            .send(&mqtt::Publish {
                payload: atat::serde_bytes::Bytes::new(data),
            })
            .await?;

        // TODO: wait for [`command::Urc::MqttMessagePublished`] URC.

        Ok(())
    }

    pub async fn mqtt_disconnect(&mut self) -> Result<(), Error> {
        self.client.send(&mqtt::Disconnect { id: 0 }).await?;
        Ok(())
    }
}
