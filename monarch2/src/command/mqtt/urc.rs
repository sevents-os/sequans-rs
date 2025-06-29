use atat::atat_derive::AtatResp;
use heapless::String;

use super::types::{MQTTStatusCode, Qos};

#[derive(Debug, Clone, AtatResp)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Connected {
    /// Client ID. The only supported value is 0 - 1 client.
    #[at_arg(position = 0)]
    pub id: u8,

    /// Connection return code.
    #[at_arg(position = 1)]
    pub rc: MQTTStatusCode,
}

#[derive(Debug, Clone, AtatResp)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Disconnected {
    /// Client ID. The only supported value is 0 - 1 client.
    #[at_arg(position = 0)]
    pub id: u8,

    /// Disconnection return code.
    #[at_arg(position = 1)]
    pub rc: MQTTStatusCode,
}

#[derive(Debug, Clone, AtatResp)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PublishResponse {
    /// Client ID. The only supported value is 0 - 1 client.
    #[at_arg(position = 0)]
    pub id: u8,

    /// Publishing message ID. The message ID after 65535 winds back to 0. This ID is local to the modem.
    #[at_arg(position = 1)]
    pub pmid: u16,

    /// Publishing return code.
    #[at_arg(position = 2)]
    pub rc: MQTTStatusCode,
}

#[derive(Debug, Clone, AtatResp)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Received {
    /// Client ID. The only supported value is 0 - 1 client.
    #[at_arg(position = 0)]
    pub id: u8,

    /// The topic the client wants to publish to.
    #[at_arg(position = 1)]
    pub topic: String<256>,

    /// Size of the message payload.
    #[at_arg(position = 2)]
    pub msg_length: u16,

    /// The quality of service level to request for the subscription.
    #[at_arg(position = 3)]
    pub qos: Qos,

    /// Id of the message to read. <mid> is generated by the broker.
    ///
    /// A maximum of 100 messages are saved in the FIFO after +SQNSMQTTONMESSAGE is emitted. If the queue overflows, the URC +SQNSMQTTMEMORYFULL is sent and the oldest messages are lost.
    ///
    /// A message with <qos>=0 doesn't have a <mid›, as this type of message is overwritten every time a new message arrives. No <mid> value is to be given to read a message with <qos>=0.
    #[at_arg(position = 2)]
    pub mid: Option<u16>,
}

#[derive(Debug, Clone, AtatResp)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Subscribed {
    /// Client ID. The only supported value is 0 - 1 client.
    #[at_arg(position = 0)]
    pub id: u8,

    /// The topic the client wants to publish to.
    #[at_arg(position = 1)]
    pub topic: String<256>,

    /// Subscription return code.
    #[at_arg(position = 2)]
    pub rc: MQTTStatusCode,
}

#[derive(Debug, Clone, AtatResp)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PromptToPublish {
    #[at_arg(position = 0)]
    pub pmid: u8,
}
