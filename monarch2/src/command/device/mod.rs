use atat::atat_derive::AtatCmd;
use responses::{ActiveRAT, Clock};
use types::RAT;

use super::NoResponse;

pub mod responses;
pub mod types;
pub mod urc;

/// This command causes device to revert to a previously saved state.
///
/// This factory reset rewinds all non-volatile parameters of the module back to the last restoration point set by Save Module Configuration: AT+SQNFACTORYSAVE (on page 267). The detail of the restoration point please refer to Save Module Configuration: AT+SQNFACTORYSAVE (on page 267). If no restoration point has been created, the parameters are overwritten with their factory defaults.
///
/// Note that this AT command also flushes any data cached by the LTE modem, such as last used cell, eDRX/PSM settings, autoconnect setting, RING config, CEREG, CMEE and the user certificates/ the private keys.
///
/// A reboot is needed to commit the command.
///
// Attention: The manufacturing command AT+SQNFACTORYSAVE must be used during the manufacturing process to define a restoration point for the AT+SQNSFACTORYRESET. Failing to create a restoration point can result in undefined behaviour.
//
// See also Mobile Termination Error Result Code: +CME ERROR (on page 282) for <err› values.
#[derive(Clone, AtatCmd)]
#[at_cmd("+SQNSFACTORYRESET", NoResponse)]
pub struct FactoryReset;

/// This command causes the device to detach from the network and shut down. Before turning off, it returns a final acknowledgement. This command proceeds despite any active or pending activity. The device does not accept any further command.
///
/// Attention: On restart, the module MUST be reset using the RESETN line. Powering the power up is not enough to reboot the module.
///
/// See also Mobile Termination Error Result Code: +CME ERROR (on page 282) for <err > values.
#[derive(Clone, AtatCmd)]
#[at_cmd("+SQNSSHDN", NoResponse, timeout = 1000)]
pub struct Shutdown;

/// This command causes device to revert to a previously saved state.
///
/// This factory reset rewinds all non-volatile parameters of the module back to the last restoration point set by Save Module Configuration: AT+SQNFACTORYSAVE. The detail of the restoration point please refer to Save Module Configuration: AT+SQNFACTORYSAVE. If no restoration point has been created, the parameters are overwritten with their factory defaults.
///
/// Note that this AT command also flushes any data cached by the LTE modem, such as last used cell, eDRX/PSM settings, autoconnect setting, RING config, CEREG, CMEE and the user certificates/ the private keys.
///
/// A reboot is needed to commit the command.
///
/// Attention: The manufacturing command AT+SQNFACTORYSAVE must be used during the manufacturing process to define a restoration point for the AT+SQNSFACTORYRESET. Failing to create a restoration point can result in undefined behaviour.
#[derive(Clone, AtatCmd)]
#[at_cmd("+SQNSFACTORYRESET", NoResponse, timeout = 10000)]
pub struct ResetToFactoryState;

/// Returns the current time.
#[derive(Clone, AtatCmd)]
#[at_cmd("+CCLK?", Clock)]
pub struct GetClock;

#[derive(Clone, AtatCmd)]
#[at_cmd("+SQNMODEACTIVE?", ActiveRAT)]
pub struct GetOperatingMode;

/// This command chooses the operating mode between LTE-M and NB-loT
/// on a device when both LTE-M and NB-IoT are allowed.
/// This command can be run only if the device is in CFUN=0 state.
///
/// The setting persists at reboot and upgrade.
///
/// If the device is not dual mode capable, the active mode cannot be changed:
/// AT+SQNMODEACTIVE (or AT+SQNMODEACTIVE?) returns the only allowed mode of operation
/// and trying to set a value with AT + SQNMODEACTIVE fails and returns +CME ERROR 589
/// (Dual mode not configured).
///
/// For devices dual mode capable, trying to set the mode of operation to the current value
/// returns OK and does nothing.
///
/// Trying to switch the mode of operation when in CFUN=1 state returns +CME ERROR 591
/// (Device is in active state).
#[derive(Clone, AtatCmd)]
#[at_cmd("+SQNMODEACTIVE", NoResponse)]
pub struct SetOperatingMode {
    #[at_arg(position = 0)]
    pub mode: RAT,
}
