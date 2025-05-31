use atat::atat_derive::AtatCmd;
use heapless::String;

use super::NoResponse;

/// AT+SQNSNVW="certificate", ‹index>, ‹size>
///
/// This command writes / deletes data (certificates, etc.) to / from the non-volatile (NV) memory. Data stored in nonvolatile memory is not affected by device reboots and software upgrades.
///
/// Attention: A factory reset (see Device Reset to Factory State: AT+SQNSFACTORYRESET (on page 484)) deletes all data written in the NV memory.
///
/// Usage and syntax changes according to the type of data to store.
///
/// The form with "certificate" writes a single certificate, or several concatenated certificates, in the non volatile memory. Once the operation is completed, public certificates are immediately available for all client secured IP connection (Device Initiated Upgrade: AT+SQNSUPGRADE (on page 74), Secured socket).
/// For secured sockets in server mode, the certificate < index> must be used to assign private certificate to the secure server.
///
/// An sindex> must be provided for the system to identify the certificate (or bundle thereof) in future operations (delete, etc.)
///
/// The ‹size > parameter gives the size in bytes of the certificate to upload: after the command is issued, the user must provide the certificate size in bytes using the PEM (Privacy-enhanced Electronic Mail) format. Once ‹size> bytes have been received, the operation is automatically completed. If the certificate is successfully uploaded and verified, the response is OK. If the upload fails for some reason, then an error code is reported.
///
/// Maximum <size > for certificates is 8 kB.
///
/// Writing a zero byte certificate at ID <index> deletes the certificate stored at that index.
#[derive(Clone, AtatCmd)]
#[at_cmd("+SQNSNVW=\"certificate\",", NoResponse, value_sep = false)]
pub struct WriteCertificate {
    /// Certificate index.
    ///
    /// Indexes O to 4 and 7 to 10 are reserved for Sequans's internal use. Do not change their contents.
    #[at_arg(position = 0)]
    pub index: u8,

    ///  Size in bytes of the certificate to upload. A 'O' value removes the corresponding entry. See above for individual limits.
    ///
    /// Important: The NVRAM has a maximum user capacity of 200 kB. Any attempt to store new data beyond that limit fails with ERROR.
    #[at_arg(position = 1)]
    pub size: usize,

    #[at_arg(position = 2)]
    pub data: String<1024>,
}

/// AT+SQNSNVW="certificate", ‹index>, ‹size>
///
/// This command writes / deletes data (certificates, etc.) to / from the non-volatile (NV) memory. Data stored in nonvolatile memory is not affected by device reboots and software upgrades.
///
/// Attention: A factory reset (see Device Reset to Factory State: AT+SQNSFACTORYRESET (on page 484)) deletes all data written in the NV memory.
///
/// Usage and syntax changes according to the type of data to store.
///
/// This form of the command writes a private key in PEM format to the non-volatile memory. Maximum < size> for private keys is 2 kB.
/// Note: Password encrypted private RSA keys are not supported.
/// i Note: The MQTT broker can provide certificates and private keys files with ‹ CR><LF> (Carriage Return and Line Feed) endings. The parameter < size>, however, must not take the ‹ CR› characters into account. To remove the < CR>s, use the following command on UNIX: cat file_with_cr
/// I tr-d \015 > no_cr_file
#[derive(Clone, AtatCmd)]
#[at_cmd("+SQNSNVW=\"privatekey\",", NoResponse, value_sep = false)]
pub struct WritePrivateKey {
    /// Private key index.
    ///
    /// Indexes O to 4 and 7 to 10 are reserved for Sequans's internal use. Do not change their contents.
    #[at_arg(position = 0)]
    pub index: u8,

    ///  Size in bytes of the private key to upload. A 'O' value removes the corresponding entry. See above for individual limits.
    ///
    /// Important: The NVRAM has a maximum user capacity of 200 kB. Any attempt to store new data beyond that limit fails with ERROR.
    #[at_arg(position = 1)]
    pub size: usize,

    #[at_arg(position = 2)]
    pub data: String<1024>,
}
