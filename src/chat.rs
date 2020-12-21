use crate::User;
#[derive(Debug)]
pub enum ChatDestination {
    Public,
    Clan,
}

#[derive(Debug)]
pub struct ChatMessage {
    pub sender: User,
    pub destination: ChatDestination,
    pub message: String,
}
