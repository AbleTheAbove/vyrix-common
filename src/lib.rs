use serde_derive::{Deserialize, Serialize};
use std::fmt;

pub const COMMON_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Health(f64, f64);
pub struct Gilt(u64);

// Temporary data will be updated later
pub struct TimeDate {
    pub tick: u64,   // when hits 40 inc second
    pub second: u64, // when hits 60 inc minute
    pub minute: u64, // when hits 120 inc day
    pub day: u8,     // when hits 255 inc season
    pub season: u8,  // when hits 4 inc year and restart all previous counters
    pub year: u64,   // ahhhhhhhhh
}
pub struct Postition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
pub struct Coordinates {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type COLOR = (u8, u8, u8);
pub type BANNER = [[COLOR; 8]; 16];
pub type ID = String;

pub const BANNER_WIDTH: u32 = 8;
pub const BANNER_HEIGHT: u32 = 16;

#[derive(Debug, Serialize, Deserialize)]
pub enum Channel {
    Public,
    Private(ID),
    Party(ID), // Steam lobby ID
    Guild(ID),
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Channel::Public => {
                write!(f, "Public",)
            }
            Channel::Private(id) => {
                write!(f, "private chat {}", id)
            }
            Channel::Guild(id) => {
                write!(f, "Guild chat")
            }
        };

        write!(f, "#",)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: ID,
    pub sender: ID,
    pub text: String,
    pub channel: Channel,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#""{}" sent by {} in channel {:?} with message id: {}"#,
            self.text, self.sender, self.channel, self.id
        )
    }
}

fn test() {
    let message = Message {
        id: "hi".to_string(),
        sender: "hi".to_string(),
        text: "Hi".to_string(),
        channel: Channel::Guild("hi".to_string()),
    };

    println!("{}", message);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub common: String,
    pub server: String,
    pub client: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NetData {
    Ping,
    Message(Message),
    Connect,
    Version(Version),
}
