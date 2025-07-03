use atat::atat_derive::AtatCmd;
use heapless::String;
use types::Qos;

use super::NoResponse;

pub mod responses;
pub mod types;
pub mod urc;

/// This command disconnects from a broker. Connection must have been previously initiated with the Initiate MQTT.
///
/// Type: `asynchronous`
#[derive(Clone, AtatCmd)]
#[at_cmd("+SQNSMQTTDISCONNECT", NoResponse)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Disconnect {
    /// Client ID. The only supported value is 0 - 1 client.
    #[at_arg(position = 0)]
    pub id: u8,
}

/// This command configure the MQTT stack with the client id, user name and password
/// (if required) for the remote broker, and the CA certificate name to use for server authentication.
///
/// Type: `synchronoous`
#[derive(Clone, AtatCmd)]
#[at_cmd("+SQNSMQTTCFG", NoResponse, timeout = 300)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Configure<'a> {
    /// Client ID. The only supported value is 0 - 1 client.
    #[at_arg(position = 0)]
    pub id: u8,

    /// The unique client ID string used when connecting to the broker. Must not be empty.
    #[at_arg(position = 1, len = 128)]
    pub client_id: &'a str,

    /// Username for broker authentication.
    #[at_arg(position = 2)]
    pub username: String<256>,

    /// Password for broker authentication.
    #[at_arg(position = 3)]
    pub password: String<256>,

    /// The index of the secure profile previously set with the SSL / TLS Security Profile Configuration.
    #[at_arg(position = 4)]
    pub sp_id: Option<u8>,
}

/// This command is used to create new client connection to an external bridge or a broker.
///
/// Note: This command only initiates a new connection to the MQTT broker.
///
/// Note: As DNS queries can take up to 120 seconds, and some might already be pending, it can take up to several DNS max. resolve time for the URC to be sent.
///
/// Note: If the MQTT connection was dropped by the server and automatically resumed by the modem, the latter sends a + SQNSMQTTONCONNECT: 0, 0 URC to the host CPU. If the MCU had subscribed to the reception of MQTT messages from the server, the MCU must re-subscribe to carry on receiving MQTT messages.
///
/// # Prerequisite
///
/// Prior call to Initiate a Client Configuration: AT+SQNSMQTTCFG ([`Configure`]).
///
/// Type: `asynchronous`
#[derive(Clone, AtatCmd)]
#[at_cmd("+SQNSMQTTCONNECT", NoResponse, timeout = 300)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Connect<'a> {
    /// Client ID. The only supported value is 0 - 1 client.
    #[at_arg(position = 0)]
    pub id: u8,

    /// Bridge or broker host name or IP address.
    #[at_arg(position = 1, len = 256)]
    pub host: &'a str,

    /// Port for LS connection. Port 8883 is used by default if a TIS certificate is provided, otherwise port 1883 is used for non-TLS connection.
    #[at_arg(position = 2)]
    pub port: Option<u32>,

    /// Maximum period (in seconds) allowed between communications with the broker.
    ///
    /// If no other messages are being exchanged, this parameter controls the rate
    /// at which the client sends ping messages to the broker. Default value is 60 seconds.
    #[at_arg(position = 3)]
    pub keepalive: Option<u32>,
}

/// This command is used to publish a payload into a topic on to a broker host. It starts the publishing operation.
///
/// The <payload> is provided as binary data of <length> bytes. The behaviour is similar to the Write Data in NVM: AT+SQNSNVW command.
///
/// Important: The connection must have been established with the +SQNSMQTTCONNECT command. The command must be used after reception of the +SQNSMQTTONCONNECT URC with <rc>=0.
///
/// The +SQNSMQTTONPUBLISH: <id>, <pmid>, <rc> URC notifies that the publishing operation asked by client <id> is done.
///
/// ‹pmid> provides the publishing message id. <c> provides the publishing result code: O if success, otherwise an error code, in which case the message is not published.
#[derive(Clone, AtatCmd)]
#[at_cmd("+SQNSMQTTPUBLISH", NoResponse, termination = "\r")]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PreparePublish<'a> {
    /// Client ID. The only supported value is 0 - 1 client.
    #[at_arg(position = 0)]
    pub id: u8,

    /// The topic the client wants to publish to.
    #[at_arg(position = 1, len = 64)]
    pub topic: &'a str,

    /// The quality of service level to request for the subscription.
    #[at_arg(position = 2)]
    pub qos: Option<Qos>,

    /// Indicates the amount of bytes to publish.
    #[at_arg(position = 3)]
    pub length: usize,
}

// NOTE: this can be nicer, we shouldn't need to have 2 separate commands but instead implement
// [`atat::AtatCmd`] for  [`PreparePublish`] and handle the customization for payload there.
#[derive(Clone, AtatCmd)]
#[at_cmd(
    "",
    NoResponse,
    cmd_prefix = "",
    termination = "",
    value_sep = false,
    timeout = 300
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Publish<'a> {
    /// The actual multi-line message to send.
    #[at_arg(position = 0, len = 2048)]
    pub payload: &'a atat::serde_bytes::Bytes,
}

/// This command delivers a message selected by its id or the last received message if <qos>=0. The device must have been connected using the Initiate MQTT Connection to a Broker: AT+SQNSMQTTCONNECT (on page 148) command.
///
/// Note: This command should be used after +SQNSMQTTONMESSAGE: <id>, ‹topic>, ‹msg_length>, ‹qos>, ‹mid> reception of the URC.
///
/// The +SQNSMQTTONMESSAGE: <id>, ‹topic>, ‹msg_length>, ‹qos>, <mid> URC notifies about a newly received message stored into the internal message cache of the client < id›.
///
/// Type: `synchronous`
#[derive(Clone, AtatCmd)]
#[at_cmd("+SQNSMQTTRCVMESSAGE", NoResponse, timeout = 300)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Receive {
    /// Client ID. The only supported value is 0 - 1 client.
    #[at_arg(position = 0)]
    pub id: u8,

    /// The topic the client wants to publish to.
    #[at_arg(position = 1)]
    pub topic: String<256>,

    /// Id of the message to read. <mid> is generated by the broker.
    ///
    /// A maximum of 100 messages are saved in the FIFO after +SQNSMQTTONMESSAGE is emitted. If the queue overflows, the URC +SQNSMQTTMEMORYFULL is sent and the oldest messages are lost.
    ///
    /// A message with <qos>=0 doesn't have a <mid›, as this type of message is overwritten every time a new message arrives. No <mid> value is to be given to read a message with <qos>=0.
    #[at_arg(position = 2)]
    pub mid: Option<u16>,

    /// Maximum length to read from the message. Currently only messages with payloads up to 4096 characters are supported.
    #[at_arg(position = 3)]
    pub max_length: Option<u16>,
}

/// This command subscribes to a topic on a broker host previously contacted with Initiate MQTT Connection to a Broker: AT+SQNSMQTTCONNECT (on page 148). This command performs the actual subscription.
///
/// The +SQNSMQTTONSUBSCRIBE: <id>, ‹topic>, ‹rc› URC notifies that the subscription has completed for the client <id>.
///
/// <topic> provides the topic name. <c> provides the subscription result code: 0 if success, otherwise an error occurred and the client's request has been rejected.
///
/// Note: This command must be used after the reception of the Initiate MQIT Connection to a Broker: AT +SQNSMQTTCONNECT URC with <rc>=0, confirming that the connection is established.
#[derive(Clone, AtatCmd)]
#[at_cmd("+SQNSMQTTSUBSCRIBE", NoResponse, timeout = 300)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Subscribe {
    /// Client ID. The only supported value is 0 - 1 client.
    #[at_arg(position = 0)]
    pub id: u8,

    /// The topic the client wants to subscribe to.
    #[at_arg(position = 1)]
    pub topic: String<256>,

    /// The quality of service level to request for the subscription.
    #[at_arg(position = 2)]
    pub qos: Option<Qos>,
}
