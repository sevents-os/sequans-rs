use atat::atat_derive::AtatEnum;

/// The possible sensitivity settings use by Walter's GNSS receiver. This sets the amount of
/// time that the receiver is actually on. More sensitivity requires more power.
#[derive(Clone, Debug, PartialEq, AtatEnum, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[at_enum(u8)]
pub enum Qos {
    #[default]
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

/// Publishing return code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, AtatEnum)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(i8)]
pub enum MQTTStatusCode {
    Success = 0,
    NoMem = -1,
    Protocol = -2,
    Inval = -3,
    NoConn = -4,
    ConnRefused = -5,
    NotFound = -6,
    ConnLost = -7,
    Tls = -8,
    PayloadSize = -9,
    NotSupported = -10,
    Auth = -11,
    AclDenied = -12,
    Unknown = -13,
    Errno = -14,
    Eai = -15,
    Proxy = -16,
    Unavailable = -17,
}
