use atat::atat_derive::AtatEnum;

/// Public key type.
#[derive(Clone, Debug, PartialEq, AtatEnum)]
pub enum KeyType {
    /// ECDSA public key, 256 bits..
    #[at_enum("ECDSA 256")]
    Ecdsa256,
    /// RSA public key, 2048 bits.
    #[at_enum("RSA 2048")]
    Rsa2048,
}
