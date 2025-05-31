use atat::atat_derive::AtatEnum;

/// The supported packet data protocol types.
#[derive(Clone, PartialEq, AtatEnum)]
#[at_enum(u8)]
pub enum PDPType {
    #[at_enum("IP")]
    IP,
    #[at_enum("IPV4V6")]
    IPv4V6,
    #[at_enum("IPV6")]
    IPv6,
    #[at_enum("Non-IP")]
    NonIP,
    #[at_enum("OSPIH")]
    OSPIH,
    #[at_enum("PPP")]
    PPP,
    #[at_enum("X.25")]
    X25,
}

/// The supported packet data protocol header compression mechanisms.
#[derive(Clone, PartialEq, AtatEnum, Default)]
#[at_enum(u8)]
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
pub enum PDPIPv4Alloc {
    #[default]
    NAS = 0,
    /// DP context is for emergency bearer services.
    DHCP = 1,
}

#[derive(Clone, PartialEq, AtatEnum, Default)]
#[at_enum(u8)]
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
pub enum PDPPCSCF {
    #[default]
    Auto = 0,
    NAS = 1,
}
