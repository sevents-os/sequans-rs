use atat::atat_derive::AtatCmd;
use heapless::String;

use super::NoResponse;

pub mod types;

/// This command sends to the MT a password which is necessary before it can be operated
/// (SIM PIN, SIM PUK, PH SIM PIN, etc.). If the PIN is to be entered twice,
/// the TA shall automatically repeat the PIN.
/// If no PIN request is pending, no action is taken towards MT and an error message,
/// +CME ERROR, is returned to TE.
///
/// Note: SIM PIN, SIM PUK, PH-SIM PIN, PH-FSIM PIN, PH-FSIM PUK, SIM PIN2 and SIM PUK2 refer to the PIN of the selected application on the UICC. For example, in an UTRAN context, the selected application on the currently selected UICC should be a USIM and the SIM PIN then represents the PIN of the selected USIM. See 3GPP TS 31.101 [65] for further details on application selection on the UICC.
///
/// If the PIN required is SIM PUK or SIM PUK2, the second PIN is required. This second PIN, ‹newpin>, is used to replace the old PIN in the SIM.
///
/// The read command returns an alphanumeric string indicating whether some password is required or not.
///
/// See also Mobile Termination Error Result Code: +CME ERROR (on page 282) for <err > values.///
#[derive(Clone, AtatCmd)]
#[at_cmd("+CPIN", NoResponse, timeout = 300)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct EnterPin {
    /// PIN code.
    #[at_arg(position = 0)]
    pub pin: String<6>,

    /// New PIN code.
    #[at_arg(position = 1)]
    pub new_pin: Option<String<6>>,
}
