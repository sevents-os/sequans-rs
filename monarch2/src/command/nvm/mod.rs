use atat::atat_derive::AtatCmd;

pub mod types;

use crate::nvm::types::DataType;

use super::NoResponse;

/// This command writes / deletes data (certificates, etc.) to / from the non-volatile (NV) memory. Data stored in nonvolatile memory is not affected by device reboots and software upgrades.
///
/// Attention: A factory reset (see Device Reset to Factory State: AT+SQNSFACTORYRESET (on page 484)) deletes all data written in the NV memory.
///
/// Usage and syntax changes according to the type of data to store.
///
/// ## Certificates
///
/// The form with "certificate" writes a single certificate, or several concatenated certificates, in the non volatile memory. Once the operation is completed, public certificates are immediately available for all client secured IP connection (Device Initiated Upgrade: AT+SQNSUPGRADE (on page 74), Secured socket).
/// For secured sockets in server mode, the certificate <index> must be used to assign private certificate to the secure server.
///
/// An <index> must be provided for the system to identify the certificate (or bundle thereof) in future operations (delete, etc.)
///
/// The ‹size> parameter gives the size in bytes of the certificate to upload: after the command is issued, the user must provide the certificate size in bytes using the PEM (Privacy-enhanced Electronic Mail) format. Once ‹size> bytes have been received, the operation is automatically completed. If the certificate is successfully uploaded and verified, the response is OK. If the upload fails for some reason, then an error code is reported.
///
/// Maximum <size> for certificates is 8 kB.
///
/// Writing a zero byte certificate at ID <index> deletes the certificate stored at that index.
///
/// ## Private key
///
/// This form of the command writes a private key in PEM format to the non-volatile memory. Maximum <size> for private keys is 2 kB.
///
/// Note: Password encrypted private RSA keys are not supported.
///
/// Note: The MQTT broker can provide certificates and private keys files with < CR> < LF> (Carriage Return and Line Feed) endings. The parameter ‹size>, however, must not take the < CR› characters into account.
#[derive(Clone, AtatCmd)]
#[at_cmd("+SQNSNVW", NoResponse)]
pub struct PrepareWrite {
    #[at_arg(position = 0)]
    pub data_type: DataType,

    /// Indexes O to 4 and 7 to 10 are reserved for Sequans's internal use. Do not change their contents.
    #[at_arg(position = 1)]
    pub index: u8,

    ///  Size in bytes of the certificate to upload. A 'O' value removes the corresponding entry. See above for individual limits.
    ///
    /// Important: The NVRAM has a maximum user capacity of 200 kB. Any attempt to store new data beyond that limit fails with ERROR.
    #[at_arg(position = 2)]
    pub size: usize,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("", NoResponse, cmd_prefix = "", termination = "", value_sep = false)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Write<'a> {
    /// The actual multi-line message to send.
    #[at_arg(position = 0, len = 8192)]
    pub data: &'a atat::serde_bytes::Bytes,
}
