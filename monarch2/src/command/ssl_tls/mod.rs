use atat::atat_derive::AtatCmd;
use heapless::String;
use responses::Configuration;
use types::{Resume, SslTlsVersion, StorageId};

pub mod responses;
pub mod types;

/// This command sets the security profile parameters required to configure subsequent SSL/TLS connections.
///
/// A security profile is identified by a unique ID <spld>. Up to 6 security profiles can be configured. Each security profile cover the following SSL/LS connections properties:
#[derive(Clone, AtatCmd)]
#[at_cmd("+SQNSPCFG", Configuration)]
pub struct Configure {
    /// Security profile identifier.
    #[at_arg(position = 0)]
    pub sp_id: u8,

    /// SSL / TLS version.
    #[at_arg(position = 1)]
    pub version: SslTlsVersion,

    /// Example: <cipherSpecs>="0x8C;0x8D;0XAE;0xAF"
    ///
    /// Warning: If the remote server supports none of the cipher suites configured in the ‹cipherSpecs> list, the handshake fails.
    ///
    /// TODO: use CipherSuite enum
    #[at_arg(position = 2)]
    pub cipher_specs: String<256>,

    /// Bit field: 8 bits wide (00. . FF): Server certificate validation.
    ///
    /// Configuration bits:
    ///
    /// • All 0 (default): certificate not validated
    /// • Bit 0 set to 1: certificate validation done against a specific or a list of imported trusted root certificates and against validity period
    /// • Bit 1: unused
    /// • Bit 2 set to 1: server URL verified against certificate common name field (on top of bit 0)
    /// • Bit 3-7 are reserved for future use
    ///
    /// For instance, to activate certification activate certification verification including validity period check, <certValidLevel>=0x01.
    #[at_arg(position = 3)]
    pub cert_valid_level: u8,

    /// Integer: 0.19: Client certificate ID,
    ///
    /// The client certificate serves to authenticate the client when mutual authentication is requested. The client certificate must be imported with Write Data in NVM: AT+SQNSNVW (on page 71) command.
    ///
    /// When this parameter is omitted (default), no certificate is referenced.
    #[at_arg(position = 4)]
    pub ca_cert_id: u8,

    /// Integer: 0..19: Client certificate ID,
    ///
    /// The client certificate serves to authenticate the client when mutual authentication is requested. The client certificate must be imported with Write Data in NVM: AT+SQNSNVW (on page 71) command.
    ///
    /// When this parameter is omitted (default), no certificate is referenced.
    #[at_arg(position = 5)]
    pub client_cert_id: u8,

    /// Integer: 0..19: Client private key ID.
    ///
    /// The client's private key is used to authenticate when mutual authentication is requested. The Client's private key should be imported with the Write Data in NVM: AT+SQNSNVW (on page 71) command.
    ///
    /// When the parameter is omitted (default), no key is referenced.
    ///
    /// Note: Password protected keys are not supported.
    #[at_arg(position = 6)]
    pub client_private_key_id: u8,

    /// String. Pre-shared key used for connection (when a TLS_PSK_* cipher suite is used). The value must be specified as a string of hexadecimal numbers (e.g. "734c61425224655f...")
    ///
    /// The factory default value is an empty string, meaning no pre-shared key defined.
    #[at_arg(position = 7)]
    pub psk: String<64>,

    /// Pre-shared key identity used for connection (when a TLS_PSK_* cipher suite is used).
    ///
    /// The factory default value is an empty string, meaning empty key identity defined
    #[at_arg(position = 8)]
    pub psk_identity: Option<String<64>>,

    /// Private key storage id used to identify whether key stored on NVM or HCE.
    #[at_arg(position = 9)]
    pub storage_id: StorageId,

    /// Session resumption feature enable.
    #[at_arg(position = 10)]
    pub resume: Resume,

    /// Maximum TLS client session duration in seconds.
    ///
    /// 0 - No limit. The server can set its own expiration value, advertised in the session ticket lifetime expiration mechanism
    /// >0 - Maximum duration of a given TLS session. This parameter takes precedence over the server own value
    #[at_arg(position = 11)]
    pub lifetime: u32,
}
