use atat::atat_derive::AtatCmd;
use responses::SignalQuality;
use types::{FunctionalMode, ResetFlag};

use super::NoResponse;

pub mod responses;
pub mod types;

/// Sets the functionality level of the device.
///
/// This command is used to control the functional mode of the modem.
/// Common values include:
/// - `0`: Minimum functionality (e.g., low-power mode)
/// - `1`: Full functionality (normal operation)
/// - `4`: Disable RF (airplane mode)
///
/// The second parameter can optionally trigger a device reset after applying the new mode.
///
/// ### AT Command Format
///
/// ```text
/// AT+CFUN=<fun>[,<rst>]
/// ```
///
/// - `<fun>`: functionality level (0, 1, 4)
/// - `<rst>`: optional reset flag (0 = no reset, 1 = reset)
///
/// ### Examples
///
/// - `AT+CFUN=1`: Set full functionality without reset.
/// - `AT+CFUN=4,1`: Airplane mode with reset.
///
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

/// This command returns received signal strength indication <rssi>. The parameter <ber> is kept for compatibility reasons but is always set to 99.
///
/// See also Mobile Termination Error Result Code: +CME ERROR (on page 282) for <err › values.
///
/// The test command returns values supported as compound values.
#[derive(Clone, Debug, AtatCmd)]
#[at_cmd("+CSQ", SignalQuality)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GetSignalQuality;
