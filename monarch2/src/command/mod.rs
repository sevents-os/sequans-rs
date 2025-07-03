use atat::{
    AtatLen,
    atat_derive::{AtatCmd, AtatResp, AtatUrc},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

pub mod types;

pub mod coap;
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
    #[at_urc("+SQNSMQTTPUBLISH")]
    MqttPromptToPublish(mqtt::urc::PromptToPublish),

    /// The + SHUTDOWN URC indicates that the ME has completed the shutdown procedure and is about to restart.
    #[at_urc("+SHUTDOWN")]
    Shutdown,

    /// The +SYSSTART URC indicates that the ME has started (or restarted after a AT^ RESET) and is ready to operate.
    #[at_urc("+SYSSTART")]
    Start,

    #[at_urc("+CEREG")]
    NetworkRegistrationStatus(network::urc::NetworkRegistrationStatus),

    #[at_urc("+SQNCOAPCONNECTED")]
    CoapConnected(coap::urc::Connected),
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

#[cfg(test)]
mod tests {
    use super::*;
    use atat::Parser;

    #[test]
    fn test_urc_parse() {
        let input = b"\r\n+LPGNSSFIXREADY: 0,\"2025-06-24T15:55:20.000000\",66563,\"20000000.000000\",\"0.000000\",\"0.000000\",\"0.000000\",\"0.000000\",\"0.000000\",\"0.000000\",\"+oyFVQ4AAADeYQAAAAAAAIADTG5IQAAAALCAxgJAAAAAAAAALkDoAwAAAwQBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADQEnNBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAaMpaaAAAAAA=\"\r\n";
        let x = Urc::parse(input);
        assert_eq!(708, x.unwrap().1);
    }
}
