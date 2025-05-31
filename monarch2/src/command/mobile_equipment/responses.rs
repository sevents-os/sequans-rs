use atat::atat_derive::AtatResp;

#[derive(Clone, Debug, AtatResp)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SignalQuality {
    /// The RSSI of the signal in dBm.
    #[at_arg(position = 0)]
    pub rssi: i32,

    /// Channel bit error rate (in percent). Always 99 ('unknown').
    #[at_arg(position = 1)]
    pub ber: u8,
}
