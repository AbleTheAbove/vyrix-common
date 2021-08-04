use std::fmt;

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

#[derive(Debug)]
pub enum Channel {
    Public,
    Private(ID),
    Guild(ID),
}

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

pub enum NetData {
    Ping,
    Message(Message),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
