use std::{collections::HashMap, sync::Arc};

use futures_util::{lock::Mutex, stream::SplitSink};
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

pub(crate) struct PeerMap(
    pub(crate) HashMap<String, Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>>,
);

impl Default for PeerMap {
    fn default() -> Self {
        Self(HashMap::new())
    }
}
