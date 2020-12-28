//use bincode::{deserialize, serialize};

use serde_derive::{Deserialize, Serialize};

///use crate::chat::ChatDestination;
use crate::User;

#[derive(Debug, Serialize, Deserialize)]
pub enum ChatDestination {
    Public,
    Clan,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataType {
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
