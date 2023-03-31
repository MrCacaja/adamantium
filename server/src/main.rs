mod log;
mod event;
mod objects;
mod state;

use futures_util::{StreamExt};
use tokio::sync::{Mutex};
use objects::Object;
use std::net::{SocketAddr};
use std::string::ToString;
use std::sync::Arc;
use rand::random;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use tungstenite::Result;
use crate::event::{remove_from_game, send_game_state, send_simple_message};
use crate::log::{Log, LogLevel};
use crate::state::{create_obj_id, GAME_STATE, PEER_MAP};
use crate::objects::{ObjectModel};

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => {
                Log::new(LogLevel::Info, peer.clone().ip().to_string() + " disconnected");
                let mut peer_map = PEER_MAP.lock().await;
                let game_state = GAME_STATE.lock().await;
                let player_objs: Vec<String> = game_state.iter()
                    .filter(|(_, value)| value.get_owner().to_string() == peer.to_string())
                    .map(|(id, _)| id.to_string())
                    .collect::<Vec<String>>().try_into().unwrap();
                peer_map.remove(&peer);
                drop(game_state);
                drop(peer_map);

                for id in player_objs.clone() {
                    remove_from_game(id).await.unwrap();
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

    authenticate_peer(peer).await.expect("Failed authentication");

    while let Some(msg) = ws_stream.lock().await.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {

        }
    }

    Ok(())
}

async fn authenticate_peer(peer: SocketAddr) -> Result<(), Error> {
    let rng = random::<i16>() % 10;
    let id = create_obj_id();
    send_simple_message(peer, id.clone()).await.unwrap();
    Object::new_with_id(id.clone(), peer, ObjectModel::Player).await;
    let mut game_state = GAME_STATE.lock().await;
    game_state.get_mut(&*id).unwrap().set_pos(rng, rng, rng).await;
    drop(game_state);
    send_game_state(peer).await.unwrap();

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