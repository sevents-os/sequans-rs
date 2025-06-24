use atat::atat_derive::AtatResp;
use heapless::String;
use serde::{
    Deserialize, Deserializer,
    de::{self, SeqAccess, Visitor},
};

use super::{
    Bool, Reserved,
    types::{FixSensitivity, LocationMode, UrcNotificationSetting},
};

#[derive(Clone, AtatResp)]
pub struct GnssConfig {
    /// The GNSS location mode.
    #[at_arg(position = 0)]
    pub loc_mode: LocationMode,

    /// The sensitivity mode.
    #[at_arg(position = 1)]
    pub fix_sensi: FixSensitivity,

    #[at_arg(position = 2)]
    pub urc_settings: UrcNotificationSetting,

    #[at_arg(position = 3)]
    pub reserved: Reserved,

    #[at_arg(position = 4)]
    pub metrics: Bool,
}

/// This structure represents the details of a certain GNSS assistance type.
#[derive(Clone, Default, AtatResp)]
pub struct GnssAssistanceTypeDetails {
    // /// Whether the GNSS assitance is available.
    // #[at_arg(position = 0)]
    // pub typ: bool,
    /// Whether the GNSS assitance is available.
    #[at_arg(position = 0)]
    pub available: Bool,

    /// Time in seconds since the last download of assitance data.
    #[at_arg(position = 1)]
    pub last_update: i32,

    /// Time (in seconds) before the current assistance data become stale (still usable but with degraded accuracy).
    #[at_arg(position = 2)]
    pub time_to_update: i32,

    /// Time (in seconds) before the current assistance data become invalid (not usable for fix computation any more).
    #[at_arg(position = 3)]
    pub time_to_expiration: i32,
}

#[derive(Clone, Default)]
pub struct GnssAsssitance {
    /// Almanac data details, this is not needed when real-time ephemeris data is available.
    pub almanac: GnssAssistanceTypeDetails,
    /// Real-time ephemeris data details. Use this kind of assistance data for the fastest and
    /// most power efficient GNSS fix.
    pub realtime_ephemeris: GnssAssistanceTypeDetails,
    /// Predicted ephemeris data details.
    pub predicted_ephemeris: GnssAssistanceTypeDetails,
}

impl atat::AtatResp for GnssAsssitance {}

impl<'de> Deserialize<'de> for GnssAsssitance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RespVisitor;

        impl<'de> Visitor<'de> for RespVisitor {
            type Value = GnssAsssitance;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("GNSS assistance response")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut assistance = GnssAsssitance::default();

                while let Some(line) = seq.next_element::<&str>()? {
                    let prefix = "+LPGNSSASSISTANCE: ";
                    let line = line.strip_prefix(prefix).unwrap();

                    let mut parts = line.split(',');

                    let part_id = parts
                        .next()
                        .ok_or_else(|| de::Error::custom("missing type"))?
                        .parse::<u8>()
                        .map_err(|_| de::Error::custom("invalid type"))?;

                    let availability = (parts
                        .next()
                        .ok_or_else(|| de::Error::custom("missing availability"))?
                        .parse::<u8>()
                        .map_err(|_| de::Error::custom("invalid availability"))?
                        != 0)
                        .into();

                    let last_update = parts
                        .next()
                        .ok_or_else(|| de::Error::custom("missing last_update"))?
                        .parse::<i32>()
                        .map_err(|_| de::Error::custom("invalid last_update"))?;

                    let time_to_update = parts
                        .next()
                        .ok_or_else(|| de::Error::custom("missing time_to_update"))?
                        .parse::<i32>()
                        .map_err(|_| de::Error::custom("invalid time_to_update"))?;

                    let time_to_expiration = parts
                        .next()
                        .ok_or_else(|| de::Error::custom("missing time_to_expiration"))?
                        .parse::<i32>()
                        .map_err(|_| de::Error::custom("invalid time_to_expiration"))?;

                    let info = GnssAssistanceTypeDetails {
                        available: availability,
                        last_update,
                        time_to_update,
                        time_to_expiration,
                    };

                    match part_id {
                        0 => assistance.almanac = info,
                        1 => assistance.realtime_ephemeris = info,
                        2 => assistance.predicted_ephemeris = info,
                        _ => return Err(de::Error::custom("unknown GNSS assistance type")),
                    }
                }

                Ok(assistance)
            }
        }

        deserializer.deserialize_seq(RespVisitor)
    }
}

#[derive(Clone, AtatResp)]
pub struct GnssCloudServerName {
    /// Server's hostname.
    #[at_arg(position = 0)]
    pub hostname: String<256>,

    /// Version of the API the server runs.
    #[at_arg(position = 1)]
    pub api_version: String<16>,
}

#[derive(Clone, Default, AtatResp)]
pub struct GnssTimeout {
    /// Time-out in seconds (0..999). 0 means no limit (default).
    #[at_arg(position = 0)]
    pub timeout: u32,
}
