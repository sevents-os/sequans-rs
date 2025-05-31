use core::str::FromStr;

use atat::{AtatResp, atat_derive::AtatResp, serde_at::serde::Deserialize};
use jiff::{
    Timestamp, Zoned,
    civil::DateTime,
    tz::{Offset, TimeZone},
};
use serde::Deserializer;

/// Any modem time below 1 Jan 2023 00:00:00 UTC is considered an invalid time.
const MODEM_MIN_VALID_TIMESTAMP: i64 = 1_672_531_200;

#[derive(Clone, Debug)]
pub struct Clock {
    /// The current timestamp.
    pub time: Zoned,
}

impl AtatResp for Clock {}

impl<'de> Deserialize<'de> for Clock {
    /// Deserializes current time from the modem clock response.
    ///
    /// Format is "yy/MM/dd, hh:mm: ss+zz", where characters indicate year (two last digits), month, day, hour, minutes, seconds and the 'GMT offset', computed as the difference in quarters of an hour, between the local legal time and GMT; range is -96... +96). E.g. 6th of May 1994, 10:10:00 PM GMT+2 hours equals to "94/05/06,22:10:00+08"
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = heapless::String::<64>::deserialize(deserializer)?;
        Clock::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl FromStr for Clock {
    type Err = ClockParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example: "24/05/30,13:22:45+08"
        if s.len() < 20 {
            return Err(ClockParseError::InvalidFormat);
        }

        let date_time_str = &s[0..17]; // "yy/MM/dd,HH:mm:ss"
        let tz_sign = s.chars().nth(17).ok_or(ClockParseError::InvalidFormat)?;
        let tz_offset_q: i32 = s[18..]
            .parse()
            .map_err(|_| ClockParseError::InvalidFormat)?;

        let offset_secs = match tz_sign {
            '-' => -tz_offset_q * 15 * 60,
            _ => tz_offset_q * 15 * 60,
        };

        let offset = Offset::from_seconds(offset_secs).unwrap().to_time_zone();

        let time = DateTime::strptime("%y/%m/%d,%H:%M:%S", date_time_str)
            .map_err(|_| ClockParseError::InvalidFormat)?
            .to_zoned(offset)
            .unwrap();

        if time.timestamp().as_second() < MODEM_MIN_VALID_TIMESTAMP {
            Ok(Self {
                time: Zoned::new(Timestamp::UNIX_EPOCH, TimeZone::UTC),
            })
        } else {
            Ok(Self { time })
        }
    }
}

#[derive(Debug)]
pub enum ClockParseError {
    InvalidFormat,
}

impl core::fmt::Display for ClockParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

use super::types::RAT;

#[derive(AtatResp)]
pub struct ActiveRAT {
    #[at_arg(position = 0)]
    pub rat: RAT,
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::Timestamp;

    #[test]
    fn test_valid_clock_with_valid_timestamp() {
        let input = "24/05/30,13:22:45+08";
        let clock = Clock::from_str(input).unwrap();
        assert!(clock.time.timestamp().as_second() >= super::MODEM_MIN_VALID_TIMESTAMP);
        assert_eq!(clock.time.offset().seconds(), 8 * 15 * 60);
    }

    #[test]
    fn test_valid_clock_with_old_timestamp() {
        let input = "20/01/01,00:00:00+00";
        let clock = Clock::from_str(input).unwrap();
        assert_eq!(clock.time.timestamp(), Timestamp::UNIX_EPOCH);
        assert_eq!(clock.time.offset(), Offset::UTC);
    }

    #[test]
    fn test_valid_clock_negative_offset() {
        let input = "24/05/30,13:22:45-04";
        let clock = Clock::from_str(input).unwrap();
        assert_eq!(clock.time.offset().seconds(), -4 * 15 * 60);
    }

    #[test]
    fn test_invalid_format_too_short() {
        let input = "24/05/30,13:22";
        let err = Clock::from_str(input).unwrap_err();
        matches!(err, ClockParseError::InvalidFormat);
    }

    #[test]
    fn test_invalid_offset_parse() {
        let input = "24/05/30,13:22:45+XX";
        let err = Clock::from_str(input).unwrap_err();
        matches!(err, ClockParseError::InvalidFormat);
    }

    #[test]
    fn test_invalid_datetime_format() {
        let input = "24-05-30,13:22:45+08"; // bad separator
        let err = Clock::from_str(input).unwrap_err();
        matches!(err, ClockParseError::InvalidFormat);
    }
}
