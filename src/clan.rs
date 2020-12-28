use serde_derive::{Deserialize, Serialize};

use crate::user;

#[derive(Debug, Serialize, Deserialize)]
pub struct Clan {
    clan_id: i8,
    members: Vec<User>,
}
