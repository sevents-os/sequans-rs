use atat::atat_derive::AtatEnum;

/// Functional mode of the modem.
#[derive(Clone, Debug, PartialEq, AtatEnum)]
#[at_enum(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FunctionalMode {
    /// Minimum
    Minimum = 0,
    /// Full
    Full = 1,
    /// Aurplane mode
    AirplaneMode = 4,
}

/// Reset flag
#[derive(Clone, Debug, PartialEq, AtatEnum)]
#[at_enum(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResetFlag {
    /// Do not reset
    Off = 0,
    /// Reset after setting
    On = 1,
}
