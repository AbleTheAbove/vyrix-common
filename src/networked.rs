//use bincode::{deserialize, serialize};
use serde_derive::{Deserialize, Serialize};
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
