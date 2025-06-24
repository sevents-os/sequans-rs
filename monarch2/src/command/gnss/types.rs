use atat::atat_derive::{AtatEnum, AtatLen};
use serde::{Deserialize, Serialize, Serializer};

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
    /// Almanac data details, this is not needed when real-time ephemeris data is available.
    Almanac = 0,
    /// Real-time ephemeris data details. Use this kind of assistance data for the fastest and
    /// most power efficient GNSS fix.
    RealTimeEphemeris = 1,
    /// Predicted ephemeris data details.
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

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProgramGnssAction {
    /// Programs a fix.
    #[default]
    Single,
    /// Cancels a previously programmed fix.
    Stop,
}

impl Serialize for ProgramGnssAction {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::Single => Serializer::serialize_bytes(serializer, b"\"single\""),
            Self::Stop => Serializer::serialize_bytes(serializer, b"\"stop\""),
        }
    }
}

#[derive(Clone, Debug, PartialEq, AtatLen, Serialize, Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Meters(pub f32);

#[cfg(test)]
mod tests {
    use super::*;
    use atat::serde_at::ser::to_slice;

    #[test]
    fn test_pdp_type() {
        let options = atat::serde_at::SerializeOptions {
            value_sep: false,
            ..atat::serde_at::SerializeOptions::default()
        };

        let mut buf = heapless::Vec::<_, 8>::new();
        buf.resize_default(8).unwrap();
        let written = to_slice(&ProgramGnssAction::Single, "", &mut buf, options).unwrap();
        buf.resize_default(written).unwrap();

        assert_eq!(
            heapless::String::<8>::from_utf8(buf).unwrap(),
            heapless::String::<8>::try_from("\"single\"").unwrap()
        );
    }
}
