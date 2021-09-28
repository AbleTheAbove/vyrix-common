#[derive(Debug, Clone, Copy)]
pub enum Metals {
    Copper,
    Iron,
    Titanium,
}
pub type BlockLevel = u8;
#[derive(Debug, Clone, Copy)]
pub enum Block {
    Air,
    Dirt { grown: bool },
    Water { level: BlockLevel },
    Ice { level: BlockLevel },
    Ore { mat: Metals },
    BarBlock { mat: Metals },
    Fire { size: u8, ttl: u32 },
}
// IJK Chunk coordinate
// XYZ Global coordinate
