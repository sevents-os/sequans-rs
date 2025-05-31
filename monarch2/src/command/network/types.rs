use atat::atat_derive::AtatEnum;

/// The supported network selection modes.
#[derive(Clone, PartialEq, AtatEnum, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[at_enum(u8)]
pub enum NetworkSelectionMode {
    /// Automatic (oper> field is ignored).
    #[default]
    Automatic = 0,
    /// Manual (<oper> field shall be present, and <AcT> optionally).
    Manual = 1,
    /// Unregister from network
    Unregister = 2,
    /// Set only <format> (for read command + COPS?), do not attempt registration / deregistration (<oper > and <AcT> fields are ignored); this value is not applicable in read command response
    SetFormat = 3,
    /// Manual / automatic (<oper > field shall be present); if manual selection fails, automatic mode (<mode>=0) is entered
    ManualAutoFallback = 4,
}

/// The supported network operator name formats.
#[derive(Clone, PartialEq, AtatEnum, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[at_enum(u8)]
pub enum OperatorNameFormat {
    /// Long format alphanumeric <oper>.
    #[default]
    LongAlphanumeric = 0,
    /// Short format alphanumeric <oper>.
    ShortAlphanumeric = 1,
    /// Numeric <oper>
    Numeric = 2,
}

/// The different network registration states that the modem can be in.
#[derive(Clone, Debug, PartialEq, AtatEnum)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[at_enum(u8)]
pub enum NetworkRegistrationState {
    NotSearching = 0,
    RegisteredHome = 1,
    Searching = 2,
    Denied = 3,
    Unknown = 4,
    RegisteredRoaming = 5,
    RegisteredSmsOnlyHome = 6,
    RegisteredSmsOnlyRoaming = 7,
    AttachedEmergencyOnly = 8,
    RegisteredCsfbNotPreferredHome = 9,
    RegisteredCsfbNotPreferredRoaming = 10,
    RegisteredTempConnLoss = 80,
}
