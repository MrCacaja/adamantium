use std::net::SocketAddr;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub(crate) enum Action {
    SyncEntity,
    SyncId,
    Disconnect,
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match &self {
            Action::SyncEntity => "0",
            Action::SyncId => "1",
            Action::Disconnect => "2",
        }
        .parse()
        .unwrap()
    }
}

pub(crate) struct ActionMessage {
    pub action: Action,
    pub arg: String,
}

impl ToString for ActionMessage {
    fn to_string(&self) -> String {
        self.action.to_string() + self.arg.as_str()
    }
}

pub(crate) struct PeerInput {
    pub peer_socket: SocketAddr,
    pub input: String,
}

pub(crate) struct PeerConnected {
    pub peer_socket: SocketAddr,
}

pub(crate) struct PeerDisconnected {
    pub peer_socket: SocketAddr,
}

impl PeerInput {
    pub(crate) fn new(peer_socket: SocketAddr, input: String) -> Self {
        Self { peer_socket, input }
    }
}

impl PeerConnected {
    pub(crate) fn new(peer_socket: SocketAddr) -> Self {
        Self { peer_socket }
    }
}

impl PeerDisconnected {
    pub(crate) fn new(peer_socket: SocketAddr) -> Self {
        Self { peer_socket }
    }
}

pub(crate) enum PeerEvent {
    Input(PeerInput),
    Connected(PeerConnected),
    Disconnected(PeerDisconnected),
}

pub(crate) type InputEventSender = UnboundedSender<Box<PeerEvent>>;
pub(crate) type InputEventReceiver = UnboundedReceiver<Box<PeerEvent>>;

pub(crate) fn create_input_event_channel() -> (InputEventSender, InputEventReceiver) {
    unbounded_channel()
}

pub(crate) enum PeerType {
    Ip(String),
    Ips(Vec<String>),
    Global,
}

pub(crate) struct ServerOutputEvent {
    pub peer_ip: PeerType,
    pub message: ActionMessage,
}

pub(crate) type OutputEventSender = UnboundedSender<Box<ServerOutputEvent>>;
pub(crate) type OutputEventReceiver = UnboundedReceiver<Box<ServerOutputEvent>>;

pub(crate) fn create_output_event_channel() -> (OutputEventSender, OutputEventReceiver) {
    unbounded_channel()
}
