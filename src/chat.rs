use crate::User;

pub enum ChatDestination {
    Public,
    Clan,
}

pub struct ChatMessage {
    pub sender: User,
    pub destination: ChatDestination,
    pub message: String,
}
