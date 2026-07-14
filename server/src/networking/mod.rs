use crate::common::events::{
    InputEventSender, PeerConnected, PeerDisconnected, PeerEvent, PeerInput,
};
use crate::ecs::resources::networking::PeerMap;
use crate::log::{Log, LogLevel};
use futures_util::lock::Mutex;
use futures_util::StreamExt;
use std::string::ToString;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, tungstenite::Error};

use tungstenite::Result;

pub(crate) async fn accept_connection(
    stream: TcpStream,
    event_sender: InputEventSender,
    peer_map: Arc<Mutex<PeerMap>>,
) {
    let fallback_sender = event_sender.clone();
    let address = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    if let Err(e) = handle_connection(stream, event_sender, peer_map).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8(_) => {
                Log::new(LogLevel::Info, address.to_string() + " disconnected");
                // remove_player(peer).await;
                fallback_sender
                    .send(Box::new(PeerEvent::Disconnected(PeerDisconnected::new(
                        address,
                    ))))
                    .unwrap();
            }
            err => {
                Log::new_error(err);
            }
        };
    }
}

async fn handle_connection(
    stream: TcpStream,
    event_sender: InputEventSender,
    peer_map: Arc<Mutex<PeerMap>>,
) -> Result<(), Error> {
    let address = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    let (write, mut read) = ws_stream.split();

    {
        let mut peer_map = peer_map.lock().await;
        peer_map
            .0
            .insert(address.to_string(), Arc::new(Mutex::new(write)));
    }

    event_sender
        .send(Box::new(PeerEvent::Connected(PeerConnected::new(address))))
        .unwrap_or_else(|e| eprintln!("Failed to send connected event 1: {}", e));

    while let Some(msg) = read.next().await {
        let msg = msg?;
        if (msg.is_text() || msg.is_binary()) && !msg.to_string().is_empty() {
            event_sender
                .send(Box::new(PeerEvent::Input(PeerInput::new(
                    address,
                    msg.to_string(),
                ))))
                .unwrap();
        }
    }

    Ok(())
}
