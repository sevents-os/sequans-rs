use atat::atat_derive::AtatResp;

/// The + SHUTDOWN URC indicates that the ME has completed the shutdown procedure and is about to restart.
#[derive(Debug, Clone, AtatResp)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Shutdown;

/// The +SYSSTART URC indicates that the ME has started (or restarted after a AT^ RESET) and is ready to operate.
#[derive(Debug, Clone, AtatResp)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Start;
