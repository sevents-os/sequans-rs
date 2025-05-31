use atat::atat_derive::AtatCmd;
use types::KeyType;

pub mod types;

use super::NoResponse;

/// This command allows to set the public key used to check the integrity of the upgrade packages.
///
/// Once this command is sent, the device expects the input of a <size>-byte PEM encoded public key.
///
/// The current public key can be obtained by using AT+SMNPK?.
///
/// # WARNING
///
/// This is a manufacturing mode command. You need to enter manufacturing mode with AT +CFUN=5 before using it.
///
/// # Prerequisite
///
/// AT+CFUN=5, OTP unlocked and pubkey not already set.
#[derive(Clone, AtatCmd)]
#[at_cmd("+SMNPK", NoResponse, timeout = 300)]
pub struct BurnPublicKey {
    /// Size in bytes of PEM encoded public key.
    #[at_arg(position = 0)]
    pub size: i32,

    /// Public key type.
    #[at_arg(position = 1)]
    pub typ: KeyType,
}
