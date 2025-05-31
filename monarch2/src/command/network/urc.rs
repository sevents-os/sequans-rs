use atat::atat_derive::AtatResp;

use super::types::NetworkRegistrationState;

// 7.14 Network registration status +CEREG
#[derive(Debug, Clone, AtatResp)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct NetworkRegistrationStatus {
    #[at_arg(position = 0)]
    pub stat: NetworkRegistrationState,
}
