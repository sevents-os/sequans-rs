use atat::atat_derive::AtatCmd;
use heapless::String;
use types::{NetworkSelectionMode, OperatorNameFormat};

use super::NoResponse;

pub mod types;
pub mod urc;

/// PLMN selection command.
///
/// This command attempts to select and register the MT on the operator network
/// using the SIM/USIM card installed in the currently selected card slot.
/// `mode` indicates whether the selection is done automatically by the MT
/// or is forced to operator `oper` (whose id is given in format `format`)
/// using a certain access technology, indicated in `AcT`. If the selected operator is not available, no other operator is selected (except if < mode>=4). If the selected access technology is not available, then the same operator is selected using an other access technology. The selected operator name format applies to further read commands (AT+COPS?) also. <mode>=2 forces an attempt to unregister from the network. The selected mode affects to all further network registration (e.g. after <mode>=2, MT is unregistered until <mode>=0 or 1 is selected). This command should be abortable when registration/ de-registration attempt is made.
///
/// CEREG URCs will be received as the module registers/ deregisters from the network.
/// See also Mobile Termination Error Result Code: +CME ERROR (on page 282) for <err > values.
///
/// The read command returns the current mode, the currently selected operator and the current Access Technology.
///
/// If no operator is selected, <formats, <oper> and <AcT> are omitted.
///
/// The test command returns a set of five parameters, each representing an operator present in the network. A set consists of an integer indicating the availability of the operator < stat>, long and short alphanumeric format of the name of the operator, numeric format representation of the operator and access technology. If any of the formats are unavailable, the field is empty. The list of operators is in order: home network, networks referenced in SIM or active application in the UICC in the following order: HPLMN selector, User controlled PLMN selector, Operator controlled PLMN selector and PLMN selector (in the SIM or GSM application), and other networks.
///
/// It is recommended (although optional) that after the operator list TA returns lists of supported <mode>s and ‹format>s. These lists shall be delimited from the operator list by two commas.
///
/// The access technology selected parameters, <AcT>, should only be used in terminals capable to register to more than one access technology. Selection of <AcT> does not limit the capability to cell reselections, even though an attempt is made to select an access technology, the phone may still re-select a cell in another access technology.
///
/// Note: This command is only available in operational mode (CFUN=1).
#[derive(Clone, AtatCmd, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[at_cmd("+COPS", NoResponse)]
pub struct PLMNSelection {
    /// Network selection mode.
    #[at_arg(position = 0)]
    pub mode: NetworkSelectionMode,

    /// Network operator name format.
    #[at_arg(position = 1)]
    pub format: Option<OperatorNameFormat>,

    /// Network operator name.
    /// `format` indicates if the format is alphanumeric or numeric; long alphanumeric format can be upto 16 characters long
    /// and short format up to 8 characters (refer GSM MoU SE.13); numeric format is the Location Area Identification number
    /// (refer 3GPP TS 24.008, sub-clause 10.5.1.3) which consists of a three BCD digit country code coded as in ITU T Recommendation E.212 Annex A,
    /// plus a two BCD digit network code, which is administration specific; returned `oper` shall not be in BCD format,
    /// but in IRA characters converted from BCD; hence the number has structure:
    /// `(country code digit 3)(country code digit 2)(country code digit 1)(network code digit 3)(network code digit 2)(network code digit 1)`.
    #[at_arg(position = 2)]
    pub oper: Option<String<16>>,
}
