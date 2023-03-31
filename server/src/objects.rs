use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::WebSocketStream;
use crate::{create_obj_id};
use crate::event::{add_to_game, send_update_obj_event};

pub type ObjectId = String;

#[derive(Serialize, Deserialize, Clone)]
pub enum ObjectModel {
    Player
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Object {
    id: ObjectId,
    model: ObjectModel,
    owner: SocketAddr,
    transform: Transform,
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
    x: i16,
    y: i16,
    z: i16
}

impl Position {
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Self {x, y, z}
    }
}

#[derive(Serialize, Deserialize, Clone)]
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
