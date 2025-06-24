use atat::atat_derive::AtatEnum;

/// Modem's radio technology.
#[derive(Clone, PartialEq, AtatEnum)]
#[at_enum(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAT {
    /// LTE-M
    LteM = 1,
    /// NB-IoT
    NBIoT = 2,
    /// Reserved for future user
    Reserved = 3,
}
