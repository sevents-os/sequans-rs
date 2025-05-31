use atat::atat_derive::AtatResp;
use heapless::{String, Vec};
use jiff::civil;

use super::types::Meters;

/// The maximum number of tracked GNSS satellites.
static GNSS_MAX_SATS: usize = 32;

/// This notification is received when a GNSS fix is available. The notification information depends on <urc_settings> and <metrics> configuration set by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
#[derive(Debug, Clone, AtatResp)]
pub struct GnssFixReady {
    /// Fix identifier. The memory can store ten fixes. If no free slot remains, the oldest fix is overwritten.
    #[at_arg(position = 0)]
    pub fix_id: u8,
    /// UTC time, in ISO 8601 format, of the GNSS fix. When <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command, the time stamp is computed using GNSS.
    #[at_arg(position = 1)]
    pub timestamp: civil::DateTime,
    /// Duration (in milliseconds) of the fix. When <loc_mode> is set to "on-device location' mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command, the duration runs from the start of the capture to the completion of the computation.
    #[at_arg(position = 2)]
    pub ttf: u32,
    /// Estimated error of the fix in metres. When <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command, the confidence is estimated at 1 a (68 %).
    #[at_arg(position = 3)]
    pub confidence: Meters,
    /// Latitude in degrees from -90 to 90. Only available when <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
    #[at_arg(position = 4)]
    pub lat: f32,
    /// Longitude in degrees from -180 to 180. Only available when <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
    #[at_arg(position = 5)]
    pub long: f32,
    /// Elevation in metres. Only available when <loc_mode> is set to "on-device location' mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command. Since this figure is computed using the GRS 80 ellipsoid as reference, it is likely to depart drastically from the true (geodesic) value in some areas.
    #[at_arg(position = 6)]
    pub elev: Meters,
    /// Northing speed in m/s. Only available when <loc _mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
    #[at_arg(position = 7)]
    pub north_speed: f32,
    /// Easting speed in m/s. Only available when <loc _mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
    #[at_arg(position = 8)]
    pub east_speed: f32,
    /// Down speed in m/s. Only available when <loc _mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
    #[at_arg(position = 9)]
    pub down_speed: f32,
    /// Base64 encoding of the GNSS raw data to be used with AT+LPGNSSSENDRAW. Maximum 256 chars.
    #[at_arg(position = 10)]
    pub raw_meas: String<256>,
    /// Satellite number (2 chars).
    #[at_arg(position = 11)]
    pub sat_n_num: String<2>,
    /// [CNO] (Carrier-to-Noise Density Ratio) figure for the `sat_n_num`th satellite, in dB/Hz.
    ///
    /// [CNO]: https://ensatellite.com/cn0/
    #[at_arg(position = 12)]
    pub sat_cn0: Vec<f32, GNSS_MAX_SATS>,
}

#[cfg(feature = "defmt")]
impl defmt::Format for GnssFixReady {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GnssFixReady",);
    }
}
