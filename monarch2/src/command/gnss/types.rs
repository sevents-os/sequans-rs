use atat::atat_derive::{AtatEnum, AtatLen};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, AtatEnum, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[at_enum(u8)]
pub enum LocationMode {
    /// The GNSS location modus. When set to 'on-device location' the GNSS subsystem will compute
    /// position and speed and estimate the error on these parameters.
    #[default]
    OnDeviceLocation = 0,
    Disabled = 1,
}

/// Type of GNSS assistance.
#[derive(Clone, PartialEq, AtatEnum)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[at_enum(u8)]
pub enum GnssAssitanceType {
    Almanac = 0,
    RealTimeEphemeris = 1,
    PredictedEphemeris = 2,
}

/// The possible sensitivity settings use by Walter's GNSS receiver. This sets the amount of
/// time that the receiver is actually on. More sensitivity requires more power.
#[derive(Clone, PartialEq, AtatEnum, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[at_enum(u8)]
pub enum FixSensitivity {
    Low = 1,
    #[default]
    Medium = 2,
    High = 3,
}

#[derive(Clone, PartialEq, AtatEnum, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[at_enum(u8)]
pub enum UrcNotificationSetting {
    Disabled = 0,
    #[default]
    Short = 1,
    Full = 2,
}

/// The possible GNSS acquistion modes. In a cold or warm start situation Walter has no clue
/// where he is on earth. In hot start mode Walter must know where he is within 100km. When no
/// ephemerides are available and/or the time is not known cold start will be used automatically.
#[derive(Clone, PartialEq, AtatEnum)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[at_enum(u8)]
pub enum AcquisitionMode {
    ColdWarmStart = 0,
    HotStart = 1,
}

#[derive(Clone, Debug, Default, PartialEq, AtatEnum)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProgramGnssAction {
    /// Programs a fix.
    #[at_enum("single")]
    #[default]
    Single,
    /// Cancels a previously programmed fix.
    #[at_enum("stop")]
    Stop,
}

#[derive(Clone, Debug, PartialEq, AtatLen, Serialize, Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Meters(pub f32);
