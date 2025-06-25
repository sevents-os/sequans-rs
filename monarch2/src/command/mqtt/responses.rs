use atat::atat_derive::AtatResp;

#[derive(Clone, AtatResp)]
pub struct PromptToPayload {
    #[at_arg(position = 0)]
    pub pmid: u16,
}
