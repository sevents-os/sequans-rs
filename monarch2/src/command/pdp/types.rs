use atat::{AtatLen, atat_derive::AtatEnum};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

/// The supported packet data protocol header compression mechanisms.
#[derive(Clone, PartialEq, AtatEnum, Default)]
#[at_enum(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDPHComp {
    #[default]
    Off = 0,
    On = 1,
    RFC1144 = 2,
    RFC2507 = 3,
    RFC3095 = 4,
    Unspec = 99,
}

/// The supported packet data protocol data compression mechanisms.
#[derive(Clone, PartialEq, AtatEnum, Default)]
#[at_enum(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDPDComp {
    #[default]
    Off = 0,
    On = 1,
    V42BIS = 2,
    V44 = 3,
    Unspec = 99,
}

#[derive(Clone, PartialEq, AtatEnum, Default)]
#[at_enum(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDPIPv4Alloc {
    #[default]
    NAS = 0,
    /// DP context is for emergency bearer services.
    DHCP = 1,
}

#[derive(Clone, PartialEq, AtatEnum, Default)]
#[at_enum(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDPRequestType {
    /// PDP context is for new PDP context establishment or for handover from a non-3GPP access network (how the MT decides whether the PDP context is for new PDP context establishment or for handover is implementation specific)
    #[default]
    NewOrHandover = 0,
    /// DP context is for emergency bearer services.
    Emergency = 1,
    /// PDP context is for new PDP context establishment.
    New = 2,
    /// PDP context is for handover from a non-3GPP access network.
    Handover = 3,
    /// PDP context is for handover of emergency bearer services from a non-3GPP access network.
    EmergencyHandover = 4,
}

/// The supported types of P-CSCF discovery in a packet data context.
#[derive(Clone, PartialEq, AtatEnum, Default)]
#[at_enum(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDPPCSCF {
    #[default]
    Auto = 0,
    NAS = 1,
}

/// The supported packet data protocol types.
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDPType {
    IP,
    IPv4V6,
    IPv6,
    NonIP,
    OSPIH,
    PPP,
    X25,
}

impl AtatLen for PDPType {
    const LEN: usize = 8;
}

impl Serialize for PDPType {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::IP => Serializer::serialize_bytes(serializer, b"\"IP\""),
            Self::IPv4V6 => Serializer::serialize_bytes(serializer, b"\"IPV4V6\""),
            Self::IPv6 => Serializer::serialize_bytes(serializer, b"\"IPV6\""),
            Self::NonIP => Serializer::serialize_bytes(serializer, b"\"Non-IP\""),
            Self::OSPIH => Serializer::serialize_bytes(serializer, b"\"OSPIH\""),
            Self::PPP => Serializer::serialize_bytes(serializer, b"\"PPP\""),
            Self::X25 => Serializer::serialize_bytes(serializer, b"\"X.25\""),
        }
    }
}

impl<'de> Deserialize<'de> for PDPType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PDPTypeVisitor;

        const VARIANTS: &[&str] = &["IP", "IPV4V6", "IPV6", "Non-IP", "OSPIH", "PPP", "X.25"];

        impl<'de> de::Visitor<'de> for PDPTypeVisitor {
            type Value = PDPType;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a valid PDP type string")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<PDPType, E>
            where
                E: de::Error,
            {
                match v {
                    b"IP" => Ok(PDPType::IP),
                    b"IPV4V6" => Ok(PDPType::IPv4V6),
                    b"IPV6" => Ok(PDPType::IPv6),
                    b"Non-IP" => Ok(PDPType::NonIP),
                    b"OSPIH" => Ok(PDPType::OSPIH),
                    b"PPP" => Ok(PDPType::PPP),
                    b"X.25" => Ok(PDPType::X25),
                    _ => {
                        let value = core::str::from_utf8(v).unwrap_or("\u{fffd}\u{fffd}\u{fffd}");
                        Err(de::Error::unknown_variant(value, VARIANTS))
                    }
                }
            }

            fn visit_str<E>(self, v: &str) -> Result<PDPType, E>
            where
                E: de::Error,
            {
                self.visit_bytes(v.as_bytes())
            }
        }

        deserializer.deserialize_bytes(PDPTypeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atat::serde_at::ser::to_slice;

    #[test]
    fn pdp_type_parsing() {
        let options = atat::serde_at::SerializeOptions {
            value_sep: false,
            ..atat::serde_at::SerializeOptions::default()
        };

        let mut buf = heapless::Vec::<_, 8>::new();
        buf.resize_default(8).unwrap();
        let written = to_slice(&PDPType::IP, "", &mut buf, options).unwrap();
        buf.resize_default(written).unwrap();

        assert_eq!(
            heapless::String::<8>::from_utf8(buf).unwrap(),
            heapless::String::<8>::try_from("\"IP\"").unwrap()
        );
    }
}
