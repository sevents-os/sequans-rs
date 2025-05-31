use atat::atat_derive::AtatCmd;
use responses::{GnssAsssitance, GnssCloudServerName, GnssConfig, GnssTimeout};
use types::{
    AcquisitionMode, FixSensitivity, GnssAssitanceType, LocationMode, ProgramGnssAction,
    UrcNotificationSetting,
};

use super::{Bool, NoResponse, Reserved};

pub mod responses;
pub mod types;
pub mod urc;

/// Configures the GNSS (Global Navigation Satellite System) module.
#[derive(Clone, AtatCmd)]
#[at_cmd("+LPGNSSCFG?", GnssConfig)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GetGnssConfig;

/// Configures the GNSS (Global Navigation Satellite System) module.
#[derive(Clone, AtatCmd)]
#[at_cmd("+LPGNSSCFG", GnssConfig)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SetGnssConfig {
    /// The GNSS location mode.
    #[at_arg(position = 0)]
    pub location_mode: LocationMode,

    /// The sensitivity mode.
    #[at_arg(position = 1)]
    pub fix_sensitivity: FixSensitivity,

    #[at_arg(position = 2)]
    pub urc_settings: UrcNotificationSetting,

    #[at_arg(position = 3)]
    pub reserved: Reserved,

    // Position 3 is reserved
    #[at_arg(position = 4)]
    pub metrics: Bool,

    /// The acquisition mode.
    #[at_arg(position = 5)]
    pub acquisition_mode: AcquisitionMode,

    /// Enables fast error report if satellite reception is too poor. 0: No early abort (default).
    #[at_arg(position = 6)]
    pub early_abort: Bool,
}

/// Triggers a connection to the GNSS cloud, downloads the almanac or the ephemeris files and stores them in persistent memory. This AT command only works with an available LTE connection.
#[derive(Clone, AtatCmd)]
#[at_cmd("+LPGNSSASSISTANCE", GnssAsssitance)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UpdateGnssAssitance {
    /// The GNSS location mode.
    #[at_arg(position = 0)]
    pub typ: GnssAssitanceType,
}

/// Verify the status of the assistance, or check if an update is required. If both the real-time and predicted ephemeris are valid when a fix is requested, the real-time ephemeris takes precedence.
#[derive(Clone, AtatCmd)]
#[at_cmd("+LPGNSSASSISTANCE?", GnssAsssitance)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GetGnssAssitance;

/// This AT command programs or cancels a GNSS fix.
///
/// The command may return an extended error message <err>, with the following meaning:
///
/// • NO_RTC: There is no RTC available (no LTE connection). Attach to the LTE network to synchronise the clock and try again.
/// • LTE
/// _CONCURRENCY: The GNSS fix cannot be performed because the device is currently connected to the
/// LTE network. Disconnect using AT+CFUN=0 (on page 299).
/// • FIX
/// _IN_PROGRESS: Another fix is already being processed.
/// • NO_VALID_EPHEMERIS_FOR_ON-DEVICE_NAVIGATION: No ephemeris is available and <loc _mode> has been set to "on-device location" by AT+LPGNSSCFG (on page 231).
#[derive(Clone, AtatCmd)]
#[at_cmd("+LPGNSSFIXPROG", GnssAsssitance)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ProgramGnss {
    /// The GNSS location mode.
    #[at_arg(position = 0)]
    pub action: ProgramGnssAction,
}

/// This AT command sets the name of the server the assistance data is downloaded from. The name is saved and preserved at reboot / reset.
#[derive(Clone, AtatCmd)]
#[at_cmd("+LPGNSSCLOUDSEL", NoResponse)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SetGnssCloudServerName<'a> {
    /// Server's hostname.
    #[at_arg(position = 0, len = 256)]
    pub hostname: &'a str,
}

/// This AT command sets the name of the server the assistance data is downloaded from. The name is saved and preserved at reboot / reset.
#[derive(Clone, AtatCmd)]
#[at_cmd("+LPGNSSCLOUDSEL?", GnssCloudServerName)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GetGnssCloudServerName;

/// This AT command sets a time-out for GNSS processing. If the time-out is reached, a +LPGNSSFIXSTOP URC is sent with "TIMEOUT" as the <reason> parameter.
#[derive(Clone, AtatCmd)]
#[at_cmd("+LPGNSSTIMEOUT", NoResponse)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SetGnssTimeout {
    /// Time-out in seconds (0..999). 0 means no limit (default).
    #[at_arg(position = 0)]
    pub timeout: u32,
}

/// This AT command gets the currently configured time-out for GNSS processing.
#[derive(Clone, AtatCmd)]
#[at_cmd("+LPGNSSTIMEOUT?", GnssTimeout)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GetGnssTimeout;
