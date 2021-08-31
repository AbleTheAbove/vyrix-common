#[derive(Debug, Clone, Copy)]
pub enum Materials {
    Copper,
    Iron,
    Sand,
    Stone,
    Titanium,
    Wood,
}

#[derive(Debug, Clone, Copy)]
pub enum Block {
    Dirt,
    Water { level: u8 },
    Ice { level: u8 },
    Ore { mat: Materials },
    Fire { size: u8, ttl: u32 },
}
// IJK Chunk coordinate
// XYZ Global coordinate
