#[derive(Debug, Clone, Copy)]
pub enum Metals {
    Copper,
    Iron,
    Titanium,
}

#[derive(Debug, Clone, Copy)]
pub enum Block {
    Air,
    Dirt { grown: bool },
    Water { level: u8 },
    Ice { level: u8 },
    Ore { mat: Metals },
    BarBlock { mat: Metals },
    Fire { size: u8, ttl: u32 },
}
// IJK Chunk coordinate
// XYZ Global coordinate
