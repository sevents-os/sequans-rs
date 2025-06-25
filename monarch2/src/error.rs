use crate::mqtt::types::MQTTStatusCode;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    AT(atat::Error),
    Timeout(embassy_time::TimeoutError),
    ClockSynchronization,
    MQTT(MQTTStatusCode),
}

impl From<atat::Error> for Error {
    fn from(err: atat::Error) -> Self {
        Error::AT(err)
    }
}

impl From<embassy_time::TimeoutError> for Error {
    fn from(err: embassy_time::TimeoutError) -> Self {
        Error::Timeout(err)
    }
}
