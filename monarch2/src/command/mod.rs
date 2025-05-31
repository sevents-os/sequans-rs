use atat::atat_derive::{AtatCmd, AtatResp, AtatUrc};

pub mod device;
#[cfg(feature = "gm02sp")]
pub mod gnss;
pub mod manufacturing;
pub mod mobile_equipment;
pub mod mqtt;
pub mod network;
pub mod nvm;
pub mod pdp;
pub mod sim;
pub mod sms;
pub mod ssl_tls;
pub mod system_features;

#[derive(Clone, AtatResp)]
pub struct NoResponse;

#[derive(Clone, AtatCmd)]
#[at_cmd("", NoResponse)]
pub struct AT;

#[derive(Debug, Clone, AtatUrc)]
#[allow(clippy::large_enum_variant)]
pub enum Urc {
    #[cfg(feature = "gm02sp")]
    #[at_urc("+LPGNSSFIXREADY")]
    GnssFixReady(gnss::urc::GnssFixReady),

    #[at_urc("+SQNSMQTTONCONNECT")]
    MqttConnected(mqtt::urc::Connected),
    #[at_urc("+SQNSMQTTONDISCONNECT")]
    MqttDisconnected(mqtt::urc::Disconnected),
    #[at_urc("+SQNSMQTTONPUBLISH")]
    MqttMessagePublished(mqtt::urc::PublishResponse),
    #[at_urc("+SQNSMQTTONMESSAGE")]
    MqttMessageReceived(mqtt::urc::Received),
    #[at_urc("+SQNSMQTTONSUBSCRIBE")]
    MqttSubscribed(mqtt::urc::Subscribed),

    #[at_urc("+SHUTDOWN")]
    Shutdown(device::urc::Shutdown),

    #[at_urc("+CEREG")]
    NetworkRegistrationStatus(network::urc::NetworkRegistrationStatus),
}
