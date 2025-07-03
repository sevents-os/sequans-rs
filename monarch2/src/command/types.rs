use atat::{AtatLen, atat_derive::AtatEnum};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Custom boolean needed for communication with the Sequans Monarch 2 chips.
/// The ATAT commands use 0 and 1 to represent booleans which isn't compatible
/// with atat and thus require custom implementation.
#[derive(Clone, Debug, PartialEq, AtatEnum, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[at_enum(u8)]
pub enum Bool {
    #[default]
    False = 0,
    True = 1,
}

impl Bool {
    pub fn as_bool(&self) -> bool {
        matches!(self, Bool::True)
    }
}

impl From<bool> for Bool {
    fn from(b: bool) -> Self {
        if b { Bool::True } else { Bool::False }
    }
}

impl From<Bool> for bool {
    fn from(b: Bool) -> Self {
        b == Bool::True
    }
}

// #[derive(Debug, Clone, Copy, PartialEq)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
// pub struct Quoted<T: AtatLen>(pub T);

// impl<T: AtatLen> AtatLen for Quoted<T> {
//     const LEN: usize = T::LEN + 2;
// }

// impl<'de, T> Deserialize<'de> for Quoted<T>
// where
//     T: AtatLen + Deserialize<'de>,
// {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s: &str = Deserialize::deserialize(deserializer)?;
//         let v = T::deserialize(s.trim_matches('"').into_deserializer())?;
//         Ok(Quoted(v))
//     }
// }

// impl<T> Serialize for Quoted<T>
// where
//     T: AtatLen + core::fmt::Display,
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut buf: heapless::String<{ T::LEN + 2 }> = heapless::String::new();
//         use core::fmt::Write;
//         buf.push('"').unwrap();
//         write!(buf, "{}", self.0).unwrap();
//         buf.push('"').unwrap();
//         serializer.serialize_str(&buf)
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nullable<T: AtatLen> {
    /// No value.
    None,
    /// Some value of type `T`.
    Some(T),
}

impl<T: AtatLen> AtatLen for Nullable<T> {
    const LEN: usize = T::LEN;
}

impl<'de, T> Deserialize<'de> for Nullable<T>
where
    T: AtatLen + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match T::deserialize(deserializer) {
            Ok(v) => Ok(Nullable::Some(v)),
            Err(_) => Ok(Nullable::None),
        }
    }
}

impl<T> Serialize for Nullable<T>
where
    T: AtatLen + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Nullable::None => serializer.serialize_bytes(&[]),
            Nullable::Some(t) => t.serialize(serializer),
        }
    }
}

impl<T: AtatLen> Nullable<T> {
    pub fn from_option(opt: Option<T>) -> Self {
        match opt {
            Some(v) => Nullable::Some(v),
            None => Nullable::None,
        }
    }

    pub fn into_option(self) -> Option<T> {
        match self {
            Nullable::Some(v) => Some(v),
            Nullable::None => None,
        }
    }

    pub fn as_option(&self) -> Option<&T> {
        match self {
            Nullable::Some(v) => Some(v),
            Nullable::None => None,
        }
    }

    pub fn as_option_mut(&mut self) -> Option<&mut T> {
        match self {
            Nullable::Some(v) => Some(v),
            Nullable::None => None,
        }
    }
}

impl<T: AtatLen> From<Option<T>> for Nullable<T> {
    fn from(opt: Option<T>) -> Self {
        Nullable::from_option(opt)
    }
}

impl<T: AtatLen> From<Nullable<T>> for Option<T> {
    fn from(n: Nullable<T>) -> Self {
        n.into_option()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atat::{
        atat_derive::AtatResp,
        serde_at::{SerializeOptions, ser::to_slice},
    };

    #[test]
    fn ser_nullable() {
        #[derive(Clone, PartialEq, Serialize)]
        pub struct WithOption {
            a: u8,
            b: Nullable<u8>,
            c: Nullable<i32>,
            d: u8,
        }

        let value = WithOption {
            a: 0,
            b: Nullable::Some(2),
            c: Nullable::None,
            d: 4,
        };

        let mut buf = heapless::Vec::<_, 32>::new();
        buf.resize_default(32).unwrap();
        let written = to_slice(&value, "+CMD", &mut buf, SerializeOptions::default()).unwrap();
        buf.resize_default(written).unwrap();

        assert_eq!(
            heapless::String::<32>::from_utf8(buf).unwrap(),
            heapless::String::<32>::try_from("AT+CMD=0,2,,4\r\n").unwrap()
        );
    }

    #[test]
    fn de_nullable() {
        #[derive(Debug, PartialEq, Serialize, AtatResp)]
        pub struct WithOption {
            a: u8,
            b: Nullable<u8>,
            c: Nullable<i32>,
            d: Nullable<heapless::String<12>>,
            e: u8,
        }

        let input = b"0,1,,\"foo\",2";

        let got = atat::serde_at::from_slice::<WithOption>(input).ok();
        assert_eq!(
            got,
            Some(WithOption {
                a: 0,
                b: Nullable::Some(1),
                c: Nullable::None,
                d: Nullable::Some(heapless::String::try_from("foo").unwrap()),
                e: 2
            })
        );
    }
}
