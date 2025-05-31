use atat::atat_derive::AtatCmd;
use responses::SignalQuality;
use types::{FunctionalMode, ResetFlag};

use super::NoResponse;

pub mod responses;
pub mod types;

/// Sets the functionality level of the device.
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+CFUN", NoResponse)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SetFunctionality {
    /// Functionality level
    #[at_arg(position = 0)]
    pub fun: FunctionalMode,

    /// Optional reset flag
    #[at_arg(position = 1)]
    pub rst: Option<ResetFlag>,
}

/// This command returns received signal strength indication (rssi).
///
/// See also Mobile Termination Error Result Code: +CME ERROR for error values.
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+CSQ", SignalQuality)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GetSignalQuality;
