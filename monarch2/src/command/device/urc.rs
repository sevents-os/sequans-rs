use atat::atat_derive::AtatResp;

#[derive(Debug, Clone, AtatResp)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Shutdown;
