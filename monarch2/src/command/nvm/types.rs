use atat::AtatLen;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

/// Type of NVM data.
#[derive(Clone, PartialEq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DataType {
    #[default]
    Certificate,
    Privatekey,
}

impl AtatLen for DataType {
    const LEN: usize = 12;
}

impl Serialize for DataType {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::Certificate => Serializer::serialize_bytes(serializer, b"\"certificate\""),
            Self::Privatekey => Serializer::serialize_bytes(serializer, b"\"privatekey\""),
        }
    }
}

impl<'de> Deserialize<'de> for DataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PDPTypeVisitor;

        const VARIANTS: &[&str] = &["IP", "IPV4V6", "IPV6", "Non-IP", "OSPIH", "PPP", "X.25"];

        impl<'de> de::Visitor<'de> for PDPTypeVisitor {
            type Value = DataType;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a valid PDP type string")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<DataType, E>
            where
                E: de::Error,
            {
                match v {
                    b"certificate" => Ok(DataType::Certificate),
                    b"privatekey" => Ok(DataType::Privatekey),
                    _ => {
                        let value = core::str::from_utf8(v).unwrap_or("\u{fffd}\u{fffd}\u{fffd}");
                        Err(de::Error::unknown_variant(value, VARIANTS))
                    }
                }
            }

            fn visit_str<E>(self, v: &str) -> Result<DataType, E>
            where
                E: de::Error,
            {
                self.visit_bytes(v.as_bytes())
            }
        }

        deserializer.deserialize_bytes(PDPTypeVisitor)
    }
}
