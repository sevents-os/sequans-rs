use atat::atat_derive::AtatEnum;

#[derive(Clone, PartialEq, AtatEnum, Default)]
#[at_enum(u8)]
pub enum SslTlsVersion {
    Tls10 = 0,
    Tls11 = 1,
    #[default]
    Tls12 = 2,
    Tls13 = 3,
    Reset = 255,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum CipherSuite {
    SslRsaWith3DesEdeCbcSha = 0x000A,
    TlsAes128Ccm8Sha256 = 0x1305,
    TlsAes128CcmSha256 = 0x1304,
    TlsAes128GcmSha256 = 0x1301,
    TlsAes256GcmSha384 = 0x1302,
    TlsChacha20Poly1305Sha256 = 0x1303,
    TlsDhePskWithAes128CbcSha256 = 0x00B2,
    TlsDhePskWithAes128Ccm = 0xC0A6,
    TlsDhePskWithAes128GcmSha256 = 0x00AA,
    TlsDhePskWithAes256CbcSha384 = 0x00B3,
    TlsDhePskWithAes256Ccm = 0xC0A7,
    TlsDhePskWithAes256GcmSha384 = 0x00AB,
    TlsDheRsaWithAes128CbcSha = 0x0033,
    TlsDheRsaWithAes128CbcSha256 = 0x0067,
    TlsDheRsaWithAes256CbcSha = 0x0039,
    TlsDheRsaWithAes256CbcSha256 = 0x006B,
    TlsEcdheEcdsaWith3desEdeCbcSha = 0xC008,
    TlsEcdheEcdsaWithAes128CbcSha = 0xC009,
    TlsEcdheEcdsaWithAes128Ccm = 0xC0AC,
    TlsEcdheEcdsaWithAes128Ccm8 = 0xC0AE,
    TlsEcdheEcdsaWithAes256CbcSha = 0xC00A,
    TlsEcdheEcdsaWithAes256Ccm8 = 0xC0AF,
    TlsEcdheRsaWith3desEdeCbcSha = 0xC012,
    TlsEcdheRsaWithAes128CbcSha = 0xC013,
    TlsEcdheRsaWithAes256CbcSha = 0xC014,
    TlsPskWithAes128CbcSha = 0x008C,
    TlsPskWithAes128CbcSha256 = 0x00AE,
    TlsPskWithAes128Ccm = 0xC0A4,
    TlsPskWithAes128Ccm8 = 0xC0A8,
    TlsPskWithAes128GcmSha256 = 0x00A8,
    TlsPskWithAes256CbcSha = 0x008D,
    TlsPskWithAes256CbcSha384 = 0x00AF,
    TlsPskWithAes256Ccm = 0xC0A5,
    TlsPskWithAes256Ccm8 = 0xC0A9,
    TlsPskWithAes256GcmSha384 = 0x00A9,
    TlsRsaWithAes128CbcSha = 0x002F,
    TlsRsaWithAes128CbcSha256 = 0x003C,
    TlsRsaWithAes128Ccm8 = 0xC0A0,
    TlsRsaWithAes128GcmSha256 = 0x009C,
    TlsRsaWithAes256CbcSha = 0x0035,
    TlsRsaWithAes256CbcSha256 = 0x003D,
    TlsRsaWithAes256Ccm8 = 0xC0A1,
}

/// Private key storage id used to identify whether key stored on NVM or HCE.
#[derive(Clone, PartialEq, AtatEnum, Default)]
#[at_enum(u8)]
pub enum StorageId {
    /// Embedded non-volatile memory (see AT+SQNSNVW)
    #[default]
    NVM = 0,
    /// Hosted Crypto Engine (host MCU acting as storage proxy)]
    HostedCryptoEngine = 1,
    /// For future use
    Reserved = 2,
}

/// Session resumption feature enable.
#[derive(Clone, PartialEq, AtatEnum, Default)]
#[at_enum(u8)]
pub enum Resume {
    /// Session resumption feature disabled (default)
    #[default]
    Disabled = 0,
    /// Session resumption feature enabled
    Enabled = 1,
}
