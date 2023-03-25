mod log;
mod event;
mod command;
mod types;

use std::collections::HashMap;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::{Mutex};
use types::Object;
use std::net::{SocketAddr};
use std::sync::Arc;
use rand::random;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error, WebSocketStream};
use tungstenite::Result;
use crate::event::{Action};
use crate::log::{Log, LogLevel};
use crate::types::{GameState, ObjectModel, PeerMap};

async fn accept_connection(peer: SocketAddr, stream: TcpStream, game_state: GameState, peer_map: PeerMap) {
    if let Err(e) = handle_connection(peer, stream, game_state, peer_map.clone()).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => {
                Log::new(LogLevel::Info, peer.clone().ip().to_string() + " disconnected");
                peer_map.lock().await.remove(&peer);
            },
            err => { Log::new_error(err); }
        };
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream, game_state: GameState, peer_map: PeerMap) -> Result<(), Error> {
    let ws_stream = Arc::new(Mutex::new(accept_async(stream).await.expect("Failed to accept")));
    peer_map.lock().await.insert(peer, ws_stream.clone());
    Log::new(LogLevel::Info, "New WebSocket connection: ".to_string() + &*peer.ip().to_string());

    authenticate_peer(peer_map, game_state, ws_stream.clone()).await.expect("Failed authentication");

    while let Some(msg) = ws_stream.lock().await.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {

        }
    }

    Ok(())
}

async fn send_event_global(peer_map: PeerMap, action: Action, arg: String) -> Result<(), Error> {
    for (_, stream) in peer_map.lock().await.iter() {
        stream.lock().await.send((action.to_string() + arg.as_str()).into()).await.unwrap();
    }

    Ok(())
}

async fn authenticate_peer(peer_map: PeerMap, game_state: GameState, ws_stream: Arc<Mutex<WebSocketStream<TcpStream>>>) -> Result<(), Error> {
    let peer_map_mutex = peer_map.clone();
    let mut game_state = game_state.lock().await;

    let rng = random::<i16>() % 10;
    let player_id = game_state.len().to_string();
    let mut player_obj = Object::new(player_id.clone(), ObjectModel::Player);
    player_obj.set_pos(rng, rng, rng);
    game_state.insert(player_id.clone(), player_obj);
    let player_ref = game_state.get(player_id.as_str()).unwrap();
    let player_json = serde_json::to_string(player_ref).unwrap();

    {
        let mut ws_stream_guard = ws_stream.lock().await;
        ws_stream_guard.send(player_id.into()).await?;
    }

    //ws_stream_guard.send((Action::Spawn.to_string() + player_json.as_str()).into()).await.unwrap();
    send_event_global(peer_map_mutex, Action::Spawn, player_json.clone()).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let game_state = GameState::new(Mutex::new(HashMap::new()));
    let peer_map = PeerMap::new(Mutex::new(HashMap::new()));
    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    Log::new(LogLevel::Info, "Listening on: ".to_string() + &*addr.to_string());

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        Log::new(LogLevel::Info, "Peer address: ".to_string() + &*peer.ip().to_string());

        tokio::spawn(accept_connection(peer, stream, game_state.clone(), peer_map.clone()));
    }
}