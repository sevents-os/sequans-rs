use atat::atat_derive::AtatResp;
use heapless::String;

use crate::gnss::types::GnssAssitanceType;

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
#[derive(Clone, AtatResp)]
pub struct GnssAsssitance {
    #[at_arg(position = 0)]
    pub typ: GnssAssitanceType,

    // /// Whether the GNSS assitance is available.
    // #[at_arg(position = 0)]
    // pub typ: bool,
    /// Whether the GNSS assitance is available.
    #[at_arg(position = 1)]
    pub available: Bool,

    /// Time in seconds since the last download of assitance data.
    #[at_arg(position = 2)]
    pub last_update: i32,

    /// Time (in seconds) before the current assistance data become stale (still usable but with degraded accuracy).
    #[at_arg(position = 3)]
    pub time_to_update: i32,

    /// Time (in seconds) before the current assistance data become invalid (not usable for fix computation any more).
    #[at_arg(position = 4)]
    pub time_to_expiration: i32,
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

#[cfg(test)]
mod tests {
    use super::*;
    use atat::serde_at::from_str;

    #[test]
    fn test_gnss_assistance_parsing() {
        let input = r#"+LPGNSSASSISTANCE: 0,1,81390742,0,0"#;
        //         let input = r#"+LPGNSSASSISTANCE: 0,1,81390742,0,0\r
        // +LPGNSSASSISTANCE: 1,0,0,0,0\r
        // +LPGNSSASSISTANCE: 2,0,0,0,0"#;
        let assistance: GnssAsssitance = from_str(input).unwrap();

        assert_eq!(assistance.available, true.into());
        assert_eq!(assistance.last_update, 81390742);
        assert_eq!(assistance.time_to_update, 0);
        assert_eq!(assistance.time_to_expiration, 0);
    }

    #[test]
    fn test_full_gnss_assistance_response_parsing() {
        let input = "+LPGNSSASSISTANCE: 0,1,81390742,0,0\r\n+LPGNSSASSISTANCE: 1,0,0,0,0\r\n+LPGNSSASSISTANCE: 2,0,0,0,0";
        let assistance: heapless::Vec<GnssAsssitance, 3> = from_str(input).unwrap();

        assert!(assistance.is_full());
    }
}
