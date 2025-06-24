use atat::{
    AtatLen,
    atat_derive::{AtatCmd, AtatEnum, AtatResp, AtatUrc},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct NoResponse;

#[derive(Clone, AtatCmd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[at_cmd("", NoResponse)]
pub struct AT;

#[derive(Debug, Clone, AtatUrc)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

/// Custom boolean needed for communication with the Sequans Monarch 2 chips.
/// The ATAT commands use 0 and 1 to represent booleans which isn't compatible
/// with atat and thus require custom implementation.
#[derive(Clone, PartialEq, AtatEnum, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[at_enum(u8)]
pub enum Bool {
    #[default]
    False = 0,
    True = 1,
}

impl From<bool> for Bool {
    fn from(b: bool) -> Self {
        if b { Bool::True } else { Bool::False }
    }
}

impl From<Bool> for bool {
    fn from(b: Bool) -> Self {
        b == Bool::True
    }
}

/// Used for reserved fields that are currently ignored but can't be skipped
/// during serialization.
#[derive(Clone, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Reserved;

impl Serialize for Reserved {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&[])
    }
}

impl AtatLen for Reserved {
    // 0 would result in the field being completely omitted which is not what we want.
    const LEN: usize = 1;
}

impl<'de> Deserialize<'de> for Reserved {
    fn deserialize<D>(deserializer: D) -> Result<Reserved, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReservedVisitor;

        impl<'de> de::Visitor<'de> for ReservedVisitor {
            type Value = Reserved;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a reserved field (ignored content)")
            }

            fn visit_bytes<E>(self, _v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Reserved)
            }
        }

        deserializer.deserialize_any(ReservedVisitor)
    }
}
