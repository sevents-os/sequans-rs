use atat::atat_derive::AtatResp;
use jiff::civil;
use serde::{Deserialize, Deserializer, de};

use crate::gnss::types::QuotedF32;

/// The maximum number of tracked GNSS satellites.
static GNSS_MAX_SATS: usize = 32;

/// This notification is received when a GNSS fix is available. The notification information depends on <urc_settings> and <metrics> configuration set by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
#[derive(Debug, Clone, PartialEq, AtatResp)]
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
    pub confidence: QuotedF32,

    /// Latitude in degrees from -90 to 90. Only available when <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
    #[at_arg(position = 4)]
    pub lat: QuotedF32,

    /// Longitude in degrees from -180 to 180. Only available when <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
    #[at_arg(position = 5)]
    pub long: QuotedF32,

    /// Elevation in metres. Only available when <loc_mode> is set to "on-device location' mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command. Since this figure is computed using the GRS 80 ellipsoid as reference, it is likely to depart drastically from the true (geodesic) value in some areas.
    #[at_arg(position = 6)]
    pub elev: QuotedF32,

    /// Northing speed in m/s. Only available when <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
    #[at_arg(position = 7)]
    pub north_speed: QuotedF32,

    /// Easting speed in m/s. Only available when <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
    #[at_arg(position = 8)]
    pub east_speed: QuotedF32,

    /// Down speed in m/s. Only available when <loc_mode> is set to "on-device location" mode by the [`SetGnssConfig` (AT+LPGNSSCFG)](super::SetGnssConfig) command.
    #[at_arg(position = 9)]
    pub down_speed: QuotedF32,

    // Base64 encoding of the GNSS raw data to be used with AT+LPGNSSSENDRAW. Maximum 256 chars.
    // This field is ignored.
    #[at_arg(position = 10)]
    pub raw_data: heapless::String<1024>,

    #[at_arg(position = 11)]
    pub sats: Option<SateliteInfos>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SateliteInfo {
    // Sattelite number.
    pub sat_no: heapless::String<2>,
    // CN0 figure for the satellite, in dB / Hz.
    // The minimum required signal strength is 30dB/Hz.
    pub signal_strength: u32,
}

/// List of satellite information.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SateliteInfos(pub heapless::Vec<SateliteInfo, GNSS_MAX_SATS>);

impl<'de> Deserialize<'de> for SateliteInfos {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // This is a very unfortunate 🍝 code. atat splits on `,` by default
        // but in our case we have data such as `("XX",100),("YY",200)`
        // (notice the comma `,` inside the parentheses).
        // What we do here is that we take all of these pairs at the end of the response
        // as one long string and then manually parse it into the sattelit info.
        let s: heapless::String<256> = heapless::String::deserialize(deserializer)?;
        let mut infos = heapless::Vec::new();

        for part in s.split_terminator("),(") {
            let cleaned = part.trim_start_matches('(').trim_end_matches(')');

            let mut fields = cleaned.splitn(2, ',');
            let num_raw = fields
                .next()
                .ok_or_else(|| de::Error::custom("Missing num"))?;
            let hx_raw = fields
                .next()
                .ok_or_else(|| de::Error::custom("Missing hx"))?;

            let num_trimmed = num_raw.trim_matches('"');
            let mut num = heapless::String::<2>::new();
            num.push_str(num_trimmed)
                .map_err(|_| de::Error::custom("num too long"))?;

            let hx: u32 = hx_raw
                .parse()
                .map_err(|_| de::Error::custom("Invalid number"))?;

            infos
                .push(SateliteInfo {
                    sat_no: num,
                    signal_strength: hx,
                })
                .map_err(|_| de::Error::custom("Too many satellites"))?;
        }

        Ok(SateliteInfos(infos))
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
        let input = b"0,\"2025-06-24T15:55:20.000000\",66563,\"20000000.000000\",\"0.000000\",\"0.000000\",\"0.000000\",\"0.000000\",\"0.000000\",\"0.000000\",\"+oyFVQ4AAADeYQAAAAAAAIADTG5IQAAAALCAxgJAAAAAAAAALkDoAwAAAwQBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADQEnNBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAaMpaaAAAAAA=\",(\"XX\",21)\r\n";

        let got = atat::serde_at::from_slice::<GnssFixReady>(input).ok();
        let expected = Some(GnssFixReady {
            fix_id: 0,
            timestamp: civil::DateTime::from_parts(
                civil::date(2025, 6, 24),
                civil::time(15, 55, 20, 00)
            ),
            ttf: 66563,
            confidence: QuotedF32(20000000.000000),
            lat: QuotedF32(0.),
            long: QuotedF32(0.),
            elev: QuotedF32(0.),
            north_speed: QuotedF32(0.),
            east_speed: QuotedF32(0.),
            down_speed: QuotedF32(0.),
            raw_data: heapless::String::try_from(
                "+oyFVQ4AAADeYQAAAAAAAIADTG5IQAAAALCAxgJAAAAAAAAALkDoAwAAAwQBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADQEnNBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAaMpaaAAAAAA="
            ).unwrap(),
            sats: Some(SateliteInfos(heapless::Vec::from_slice(&[
                SateliteInfo{
                    sat_no: heapless::String::try_from("XX").unwrap(),
                    signal_strength: 21,
                 }
            ]).unwrap())),
        });
        assert_eq!(got, expected);
    }
}
