use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::WebSocketStream;
use crate::{create_obj_id};
use crate::event::{add_to_game, remove_from_game, send_update_obj_event};
use crate::state::{GAME_STATE, PEER_MAP};
use strum_macros::EnumIter;

pub type ObjectId = String;

#[derive(Serialize, Deserialize, Clone)]
pub enum ObjectModel {
    Player
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Object {
    pub id: ObjectId,
    pub model: ObjectModel,
    pub owner: SocketAddr,
    pub transform: Transform,
}

pub async fn get_player_obj_ids(peer: SocketAddr) -> Vec<String> {
    GAME_STATE.lock().await.iter()
        .filter(|(_, value)| value.get_owner().to_string() == peer.to_string())
        .map(|(id, _)| id.to_string())
        .collect::<Vec<String>>().try_into().unwrap()
}

pub async fn remove_player(peer: SocketAddr) {
    let obj_ids = get_player_obj_ids(peer).await;
    PEER_MAP.lock().await.remove(&peer);
    for id in obj_ids {
        remove_from_game(id).await.unwrap();
    }
}

async fn new_obj_with_id(id: ObjectId, owner: SocketAddr, model: ObjectModel) -> ObjectId {
    let obj = Object {transform: Transform::new(0, 0, 0), owner, model, id: id.clone()};
    add_to_game(obj).await.unwrap();
    id
}

impl Object {
    pub async fn new(&self, owner: SocketAddr, model: ObjectModel) -> ObjectId {
        new_obj_with_id(create_obj_id(), owner, model).await
    }

    pub async fn new_with_id(id: ObjectId, owner: SocketAddr, model: ObjectModel) -> ObjectId {
        new_obj_with_id(id, owner, model).await
    }

    pub async fn set_pos(&mut self, x: i16, y: i16, z: i16) {
        self.transform.position.x = x;
        self.transform.position.y = y;
        self.transform.position.z = z;

        send_update_obj_event(serde_json::to_string(self).unwrap()).await.unwrap();
    }

    pub fn get_owner(&self) -> &SocketAddr {
        &self.owner
    }

    pub fn get_id(&self) -> ObjectId {
        self.id.clone()
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Position {
    pub x: i16,
    pub y: i16,
    pub z: i16
}

impl Position {
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Self {x, y, z}
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transform {
    pub position: Position
}

#[derive(EnumIter, PartialEq)]
pub enum Direction {
    NORTH,
    WEST,
    SOUTH,
    EAST,
    NORTHWEST,
    NORTHEAST,
    SOUTHWEST,
    SOUTHEAST
}

impl Transform {
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Self {position: Position::new(x, y, z)}
    }
}

pub async fn move_player_obj(actor_id: ObjectId, direction: Direction) {
    let mut game_state = GAME_STATE.lock().await;
    let mut obj = game_state.get_mut(&*actor_id).unwrap();
    if direction == Direction::NORTH {
        obj.transform.position.z -= 1;
    }
    if direction == Direction::WEST  {
        obj.transform.position.x -= 1;
    }
    if direction == Direction::SOUTH  {
        obj.transform.position.z += 1;
    }
    if direction == Direction::EAST  {
        obj.transform.position.x += 1;
    }

    tokio::spawn(send_update_obj_event(serde_json::to_string(obj).unwrap()));
}

pub type GameState = Arc<Mutex<HashMap<ObjectId, Object>>>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<WebSocketStream<TcpStream>>>>>>;
