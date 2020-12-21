//use bincode::{deserialize, serialize};

use serde_derive::{Deserialize, Serialize};

///use crate::chat::ChatDestination;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChatDestination {
    Public,
    Clan,
}

#[derive(Debug, Serialize, Deserialize)]
enum DataType {
    Coords {
        longitude: f32,
        latitude: f32,
        altitude: f32,
    },
    ChatMessage {
        sender: User,
        destination: ChatDestination,
        message: String,
    },
}
