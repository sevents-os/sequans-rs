use atat::atat_derive::AtatCmd;
use heapless::String;
use types::{PDPDComp, PDPHComp, PDPIPv4Alloc, PDPPCSCF, PDPRequestType, PDPType};

pub mod types;

use super::{Bool, NoResponse};

/// Defines a PDP context.
///
/// This command configures the parameters of a Packet Data Protocol (PDP) context.
/// It is used to define the context ID, PDP type (e.g., "IP", "IPV6", or "IPV4V6"),
/// the Access Point Name (APN), and optionally the PDP address and other parameters.
///
/// Reboot persistent, module must not be attached (+CEREG != 1 or 5).
#[derive(Clone, AtatCmd)]
#[at_cmd("+CGDCONT", NoResponse)]
pub struct DefinePDPContext {
    /// Context Identifier (CID): integer between 1–16.
    #[at_arg(position = 0)]
    pub cid: u8,

    /// PDP Type: typically "IP", "IPV6", or "IPV4V6".
    #[at_arg(position = 1, len = 12)]
    pub pdp_type: PDPType,

    /// Cellular APN for SIM card. Leave empty to autodetect APN.
    #[at_arg(position = 2)]
    pub apn: String<64>,

    /// Optional PDP address. Usually left empty for dynamic assignment.
    #[at_arg(position = 3)]
    pub pdp_addr: String<64>,

    /// Data compression.
    #[at_arg(position = 4)]
    pub d_comp: PDPDComp,

    /// The supported packet data protocol header compression mechanisms..
    #[at_arg(position = 5)]
    pub h_comp: PDPHComp,

    /// IPv4 address allocation method (0, 1, or 2).
    #[at_arg(position = 6)]
    pub ipv4_alloc: PDPIPv4Alloc,

    /// Integer: 0..4. Indicates the type of PDP context activation request for the PDP context, see 3GPP TS 24.301 [83] (sub-clause 6.5.1.2) and 3GPP TS 24.008 [8] (sub-clause 10.5.6.17). If the initial PDP context is supported (see sub-clause 10.1.0) it is not allowed to assign <cid>=0 for emergency bearer services.
    ///
    /// According to 3GPP TS 24.008 [8] (sub-clause 4.2.4.2.2 and sub-clause 4.2.5.1.4) and 3GPP TS 24.301 [83] (sub-clause 5.2.2.3.3 and sub-clause 5.2.3.2.2), a separate PDP context must be established for emergency bearer services.
    /// eng ale tot me a a series to el adisated come, only
    ///
    /// NOTE: A PDP context established for handover of emergency bearer services from a non-3GPP access network has the same status as a PDP context for emergency bearer services.    ///
    #[at_arg(position = 7)]
    pub request_type: PDPRequestType,

    /// The supported types of P-CSCF discovery in a packet data context.
    #[at_arg(position = 8)]
    pub pdp_pcscf_discovery_method: PDPPCSCF,

    /// Indicates to the network whether the PDP context is for IM CN subsystem-related signalling only or not.
    #[at_arg(position = 9)]
    pub for_imcn: Bool,

    /// Indicates the NAS signalling priority requested for this PDP context.
    #[at_arg(position = 10)]
    pub nslpi: Bool,

    /// Specifies if security protected transmission of PCO is requested or not.
    #[at_arg(position = 11)]
    pub secure_pco: Bool,

    /// Influences how the MT/TA requests to get the IPv4 MTU size.
    /// - false: Preference of IPv4 MTU size discovery not influenced by + CGDCONT
    /// - true: Preference of IP v4 MTU size discovery through NAS signalling
    #[at_arg(position = 12)]
    pub ipv4_mtu_discovery: Bool,

    /// Indicates to the network whether or not the MS supports local IP address in TFTs
    /// - false: Indicates that the MS does not support local IP address in TFTs
    /// - true: Indicates that the MS supports local IP address in TFTs
    #[at_arg(position = 13)]
    pub local_addr_ind: Bool,

    /// Influences how the MT/ TA requests to get the Non-IP MTU size.
    /// - false: Preference of Non-IP MTU size discovery not influenced by + CDCONT
    /// - true: Preference of Non-IP MTU size discovery through NAS signalling
    #[at_arg(position = 14)]
    pub non_ip_mtu_discovery: Bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use atat::serde_at::ser::to_slice;
    use heapless::String;

    #[test]
    fn test_pdp_type() {
        let options = atat::serde_at::SerializeOptions {
            value_sep: false,
            ..atat::serde_at::SerializeOptions::default()
        };

        let mut buf = heapless::Vec::<_, 8>::new();
        buf.resize_default(8).unwrap();
        let written = to_slice(&PDPType::IP, "", &mut buf, options).unwrap();
        buf.resize_default(written).unwrap();

        assert_eq!(
            String::<8>::from_utf8(buf).unwrap(),
            String::<8>::try_from("\"IP\"").unwrap()
        );
    }
}
