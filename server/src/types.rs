use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::WebSocketStream;

pub type ObjectId = String;
pub type PeerId = String;
pub type ObjectIdCounter = Arc<Mutex<u16>>;

#[derive(Serialize, Deserialize)]
pub enum ObjectModel {
    Player
}

#[derive(Serialize, Deserialize)]
pub struct Object {
    id: ObjectId,
    model: ObjectModel,
    owner: SocketAddr,
    transform: Transform,
}

impl Object {
    pub fn new(id: ObjectId, owner: SocketAddr, model: ObjectModel) -> Self {
        Self {transform: Transform::new(0, 0, 0), owner, model, id}
    }

    pub fn set_pos(&mut self, x: i16, y: i16, z: i16) {
        self.transform.position.x = x;
        self.transform.position.y = y;
        self.transform.position.z = z;
    }

    pub fn get_owner(&self) -> &SocketAddr {
        &self.owner
    }
}


#[derive(Serialize, Deserialize)]
pub struct Position {
    x: i16,
    y: i16,
    z: i16
}

impl Position {
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Self {x, y, z}
    }
}

#[derive(Serialize, Deserialize)]
pub struct Transform {
    position: Position
}

impl Transform {
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Self {position: Position::new(x, y, z)}
    }
}

pub type GameState = Arc<Mutex<HashMap<ObjectId, Object>>>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<WebSocketStream<TcpStream>>>>>>;
