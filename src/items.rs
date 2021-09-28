use super::{Effect, Rank};

pub struct DiceDamage {
    quantity: u8,
    sides: u8,
    modifier: u8,
}

pub enum Enhancement {
    LifeSteal,
    Reach,
    Sharpness,
}

pub type Material = u8;
pub struct BladedWeapon {
    pub BladeMaterial: Material,
    pub GuardMaterial: Material,
    pub Enhancements: Vec<Enhancement>,
}
