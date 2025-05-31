/// https://quickspot.io/docs/file/gm02s_at_commands.pdf
use atat::atat_derive::AtatCmd;
use types::{CEREGReports, CMEErrorReports};

use super::NoResponse;

pub mod types;

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMEE", NoResponse, timeout = 300)]
pub struct ConfigureCMEErrorReports {
    #[at_arg(position = 0)]
    pub typ: CMEErrorReports,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CEREG", NoResponse)]
pub struct ConfigureCEREGReports {
    #[at_arg(position = 0)]
    pub typ: CEREGReports,
}
