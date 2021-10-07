use serde_derive::{Deserialize, Serialize};
use std::fmt;
pub mod world;
pub use world::*;

pub const COMMON_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const SERVER_PORT: u16 = 40300;
pub const CLIENT_PORT: u16 = 40301;

pub enum Rank {
    God,
    Top,
    Mid,
    Low,
}

#[derive(Serialize, Deserialize)]
pub struct IPConfig {
    pub server: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub enum Holidays {
    None,
    ArborDay,
    HallowsEve,
}

#[derive(Serialize, Deserialize)]
pub struct HolidayMap {
    pub current: Holidays,
    pub tomorrow: Holidays,
}

#[derive(Serialize, Deserialize)]
pub struct Health(pub f64, pub f64);
#[derive(Serialize, Deserialize)]
pub struct Gilt(pub u64);
#[derive(Serialize, Deserialize)]
pub struct GuildTag(ID);
// Temporary data will be updated later
pub struct TimeDate {
    pub tick: u64,   // when hits 40 inc second
    pub second: u64, // when hits 60 inc minute
    pub minute: u64, // when hits 120 inc day
    pub day: u8,     // when hits 255 inc season
    pub season: u8,  // when hits 4 inc year and restart all previous counters
    pub year: u64,   // ahhhhhhhhh
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChunkPosition {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}
pub struct Coordinates {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type LIGHTLEVEL = u8;
pub type COLOR = (u8, u8, u8);
pub type BANNER = [[COLOR; 8]; 16];
pub type ID = u64;

pub const BANNER_WIDTH: u32 = 8;
pub const BANNER_HEIGHT: u32 = 16;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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
            Channel::Party(id) => {
                write!(f, "Party chat {}", id)
            }
            Channel::Guild(id) => {
                write!(f, "Guild chat")
            }
        };

        write!(f, "#",)
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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
        id: 9,
        sender: 7,
        text: "Hi".to_string(),
        channel: Channel::Guild(64),
    };

    println!("{}", message);
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Version {
    pub common: String,
    pub server: String,
    pub client: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetData {
    Ping,
    Message(Message),
    Connect,
    Version(Version),
    SyncPosition(Coordinates),
}

// All duration is in number of seconds
// Multiplier followed by duration
pub enum Effects {
    None,
    Agility(u8, u64),
    Poison(u8, u64),
    Regeneration(u8, u64),
    Burning(u8, u64),
    Slowness(u8, u64),
}
