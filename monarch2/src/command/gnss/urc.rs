use atat::{AtatUrc, atat_derive::AtatResp};
use heapless::{String, Vec};
use jiff::civil;

/// The maximum number of tracked GNSS satellites.
static GNSS_MAX_SATS: usize = 32;

/// This notification is received when a GNSS fix is available. The notification information depends on <urc_settings> and <metrics> configuration set by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
#[derive(Debug, Clone, PartialEq, AtatResp)]
pub struct GnssFixReady {
    /// Fix identifier. The memory can store ten fixes. If no free slot remains, the oldest fix is overwritten.
    pub fix_id: u8,

    /// UTC time, in ISO 8601 format, of the GNSS fix. When <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command, the time stamp is computed using GNSS.
    pub timestamp: civil::DateTime,

    /// Duration (in milliseconds) of the fix. When <loc_mode> is set to "on-device location' mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command, the duration runs from the start of the capture to the completion of the computation.
    pub ttf: u32,

    /// Estimated error of the fix in metres. When <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command, the confidence is estimated at 1 a (68 %).
    pub confidence: f32,

    /// Latitude in degrees from -90 to 90. Only available when <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
    pub lat: f32,

    /// Longitude in degrees from -180 to 180. Only available when <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
    pub long: f32,

    /// Elevation in metres. Only available when <loc_mode> is set to "on-device location' mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command. Since this figure is computed using the GRS 80 ellipsoid as reference, it is likely to depart drastically from the true (geodesic) value in some areas.
    pub elev: f32,

    /// Northing speed in m/s. Only available when <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
    pub north_speed: f32,

    /// Easting speed in m/s. Only available when <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
    pub east_speed: f32,

    /// Down speed in m/s. Only available when <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
    pub down_speed: f32,

    /// Base64 encoding of the GNSS raw data to be used with AT+LPGNSSSENDRAW. Maximum 256 chars.
    /// This field is ignored.

    /// Satellite number (2 chars).
    pub sat_n_num: String<2>,

    /// [CNO] (Carrier-to-Noise Density Ratio) figure for the `sat_n_num`th satellite, in dB/Hz.
    ///
    /// [CNO]: https://ensatellite.com/cn0/
    pub sat_cn0: Vec<f32, GNSS_MAX_SATS>,
}

impl AtatUrc for GnssFixReady {
    type Response = Self;

    fn parse(resp: &[u8]) -> Option<Self::Response> {
        const PREFIX: &str = "+LPGNSSFIXREADY: ";

        let resp_str = str::from_utf8(resp).ok()?;
        let rest = resp_str.strip_prefix(PREFIX)?.trim();

        let mut parts = rest.split(',').map(str::trim);

        let fix_id: u8 = parts.next()?.parse().ok()?;
        let timestamp: civil::DateTime = parts.next()?.trim_matches('"').parse().ok()?;
        let ttf: u32 = parts.next()?.parse().ok()?;
        let confidence: f32 = parts.next()?.trim_matches('"').parse().ok()?;
        let lat: f32 = parts.next()?.trim_matches('"').parse().ok()?;
        let long: f32 = parts.next()?.trim_matches('"').parse().ok()?;
        let elev: f32 = parts.next()?.trim_matches('"').parse().ok()?;
        let north_speed: f32 = parts.next()?.trim_matches('"').parse().ok()?;
        let east_speed: f32 = parts.next()?.trim_matches('"').parse().ok()?;
        let down_speed: f32 = parts.next()?.trim_matches('"').parse().ok()?;

        // NOTE: we skip the raw measurment.
        parts.next()?;

        // let mut sat_n_num = String::new();
        // sat_n_num.push_str(parts.next()?).ok()?;

        // // Parse remaining CN0 float values
        // let mut sat_cn0 = Vec::new();
        // for part in parts {
        //     let cn0: f32 = part.trim_matches('"').parse().ok()?;
        //     sat_cn0.push(cn0).ok()?;
        // }

        Some(GnssFixReady {
            fix_id,
            timestamp,
            ttf,
            confidence,
            lat,
            long,
            elev,
            north_speed,
            east_speed,
            down_speed,
            sat_n_num: String::new(),
            sat_cn0: Vec::new(),
        })
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for GnssFixReady {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GnssFixReady",);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gnss_fix_ready_parsing() {
        let input = b"+LPGNSSFIXREADY: 0,\"2025-06-24T15:55:20.000000\",66563,\"20000000.000000\",\"0.000000\",\"0.000000\",\"0.000000\",\"0.000000\",\"0.000000\",\"0.000000\",\"+oyFVQ4AAADeYQAAAAAAAIADTG5IQAAAALCAxgJAAAAAAAAALkDoAwAAAwQBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADQEnNBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAaMpaaAAAAAA=\"";

        let fix = GnssFixReady::parse(input);

        assert_eq!(
            fix,
            Some(GnssFixReady {
                fix_id: 0,
                timestamp: civil::DateTime::from_parts(
                    civil::date(2025, 6, 24),
                    civil::time(15, 55, 20, 00)
                ),
                ttf: 66563,
                confidence: 20000000.000000,
                lat: 0.,
                long: 0.,
                elev: 0.,
                north_speed: 0.,
                east_speed: 0.,
                down_speed: 0.,
                sat_n_num: String::new(),
                sat_cn0: Vec::new()
            })
        );
    }
}
