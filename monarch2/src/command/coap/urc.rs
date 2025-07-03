use atat::atat_derive::AtatResp;

use crate::types::Bool;

#[derive(Debug, Clone, AtatResp)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Connected {
    /// Profile id.
    #[at_arg(position = 0)]
    pub id: u8,

    /// Connection return code.
    #[at_arg(position = 1)]
    pub server_address: heapless::String<64>,

    #[at_arg(position = 2)]
    pub port: u16,

    #[at_arg(position = 3)]
    pub local_port: u16,

    #[at_arg(position = 4)]
    pub dtls_enabled: Bool,
}
