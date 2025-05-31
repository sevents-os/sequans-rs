use atat::atat_derive::AtatEnum;

/// The possible states that the SIM card can be in.
#[derive(Clone, PartialEq, AtatEnum)]
#[at_enum(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SIMState {
    /// MT is not waiting for any password.
    Ready = 1,
    /// MT is waiting for the SIM PIN to be given.
    PinRequired = 2,
    /// MT is waiting for the SIM PUK to be given.
    PukRequired = 3,
    /// MT is waiting for the phone to SIM card password to be given.
    PhoneToSimPinRequired = 4,
    /// MT is waiting for the phone-to-very first SIM card password to be given.
    PhoneToFirstSimPinRequired = 5,
    /// MT is waiting for the phone-to-very first SIM card unblocking password to be given.
    PhoneToFirstSimPukRequired = 6,
    /// MT is waiting for theSIM PIN2 to be given (this <code> is recommended to be returned only when the last executed command resulted in PIN2 authentication failure (i.e. +CME ERROR: 17); if PIN2 is not entered right after the failure, it is recommended that MT does not block its operation).
    Pin2Required = 7,
    /// MT is waiting for the SIM PUK2 to be given (this < code> is recommended to be returned only when the last executed command resulted in PUK2 authentication failure (i.e. +CME ERROR: 18); if PUK2 and new PIN2 are not entered right after the failure, it is recommended that MT does not block its operation).
    Puk2Required = 8,
    /// MT is waiting for the network personalisation password to be given.
    NetworkPinRequired = 9,
    /// MT is waiting for the network personalisation unblocking password to be given.
    NetworkPukRequired = 10,
    /// MT is waiting for the network subset personalization password to be given.
    NetworkSubsetPinRequired = 11,
    /// MT is waiting for the network subset personalization unblocking password to be given.
    NetworkSubsetPukRequired = 12,
    /// MT is waiting for the service provider personalization password to be given.
    ServiceProviderPinRequired = 13,
    /// MT is waiting for service provider personalisation unblocking password to be given.
    ServiceProviderPukRequired = 14,
    /// MT is waiting for the corporate personalisation password to be given.
    CorporateSimRequired = 15,
    /// MT is waiting for the corporate personalisation unblocking password to be given.
    CorporatePukRequired = 16,
}
