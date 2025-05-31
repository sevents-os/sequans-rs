use atat::atat_derive::AtatResp;

use super::types::NetworkRegistrationState;

// 7.14 Network registration status +CEREG
#[derive(Debug, Clone, AtatResp)]
pub struct NetworkRegistrationStatus {
    #[at_arg(position = 0)]
    pub stat: NetworkRegistrationState,
}
