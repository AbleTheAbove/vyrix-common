use super::{Effect, Rank};

pub struct DiceDamage {
    quantity: u8,
    sides: u8,
    modifier: u8,
}

pub struct Weapon {
    name: String,
    rank: Rank,
    damage: DiceDamage,
    cooldown: u64,
    effect: Effect,
}
