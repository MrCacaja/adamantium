mod log;
mod event;
mod command;
mod types;

use std::collections::HashMap;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::{Mutex, MutexGuard};
use types::Object;
use std::net::{SocketAddr};
use std::ops::Deref;
use std::string::ToString;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use once_cell::sync::Lazy;
use rand::random;
use syn::__private::str;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error, WebSocketStream};
use tungstenite::Result;
use crate::event::{Action};
use crate::log::{Log, LogLevel};
use crate::types::{GameState, ObjectId, ObjectModel, PeerId, PeerMap};

static GAME_STATE: Lazy<GameState> = Lazy::new(|| {
    GameState::new(Mutex::new(HashMap::new()))
});
static PEER_MAP: Lazy<PeerMap> = Lazy::new(|| {
    PeerMap::new(Mutex::new(HashMap::new()))
});
static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

async fn remove_from_game() {

}

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => {
                Log::new(LogLevel::Info, peer.clone().ip().to_string() + " disconnected");
                let mut peer_map = PEER_MAP.lock().await;
                let game_state = GAME_STATE.lock().await;
                let player_objs: Vec<String> = game_state.iter().filter(|(_, value)| value.get_owner().to_string() == peer.to_string()).map(|(id, _)| id.to_string()).collect::<Vec<String>>().try_into().unwrap();
                peer_map.remove(&peer);
                drop(game_state);
                for id in player_objs.clone() {
                    for (_, stream) in peer_map.iter() {
                        stream.lock().await.send((Action::Destroy.to_string() + id.as_str()).into()).await.unwrap();
                    }
                }
                let mut game_state = GAME_STATE.lock().await;
                for id in player_objs {
                    game_state.remove(id.as_str());
                }
            },
            err => { Log::new_error(err); }
        };
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<(), Error> {
    let ws_stream = Arc::new(Mutex::new(accept_async(stream).await.expect("Failed to accept")));
    PEER_MAP.lock().await.insert(peer, ws_stream.clone());
    Log::new(LogLevel::Info, "New WebSocket connection: ".to_string() + &*peer.ip().to_string());

    authenticate_peer(peer, ws_stream.clone()).await.expect("Failed authentication");

    while let Some(msg) = ws_stream.lock().await.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {

        }
    }

    Ok(())
}

async fn send_event_global(action: Action, arg: String) -> Result<(), Error> {
    for (_, stream) in PEER_MAP.lock().await.iter() {
        stream.lock().await.send((action.to_string() + arg.as_str()).into()).await.unwrap();
    }

    Ok(())
}

async fn authenticate_peer(peer: SocketAddr, ws_stream: Arc<Mutex<WebSocketStream<TcpStream>>>) -> Result<(), Error> {
    let mut game_state = GAME_STATE.lock().await;

    let rng = random::<i16>() % 10;
    let player_id = ID_COUNTER.fetch_add(1, Ordering::Relaxed).to_string();
    let mut player_obj = Object::new(player_id.clone(), peer, ObjectModel::Player);
    player_obj.set_pos(rng, rng, rng);
    game_state.insert(player_id.clone(), player_obj);
    let player_ref = game_state.get(player_id.as_str()).unwrap();
    let player_json = serde_json::to_string(player_ref).unwrap();

    {
        let mut ws_stream_guard = ws_stream.lock().await;
        ws_stream_guard.send(player_id.into()).await?;
        let game_state_json = serde_json::to_string(&game_state.deref()).unwrap();
        ws_stream_guard.send((Action::SendState.to_string() + game_state_json.as_str()).into()).await.unwrap();
    }

    send_event_global(Action::Spawn, player_json.clone()).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9002").await.expect("Can't listen");
    Log::new(LogLevel::Info, "Listening on: ".to_string() + &*"127.0.0.1:9002".to_string());

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        Log::new(LogLevel::Info, "Peer address: ".to_string() + &*peer.ip().to_string());

        tokio::spawn(accept_connection(peer, stream));
    }
}