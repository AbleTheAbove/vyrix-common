use super::*;

// We need 5.17 layers of chunks to get to MC's world border.
// Obviously, this is impossible, so instead we'll round up to 6.
// Now, if we just fill chunks with chunks, we'll get memory overflow
// because (32^6)^3 is 32^18, which is a REALLY big number.
// So, everything will be boxed for the higher levels, and unboxed
// for the lower levels. This way we don't all of a sudden run out of
// memory because we generate an entire world in RAM. Yikes.

// pub enum ChunkLevel {
// 	Level0 = 0, // I am a World Chunk. I contain Blocks
// 	Level1,     // I contain World Chunks.
// 	Level2,     // I contain chunks that contain World Chunks.
// 	Level3,     // I contain chunks that contain chunks that contain World Chunks.
// 	Level4,     // I contain chunks that contain chunks that contain chunks that contain World Blocks.
// 	Level5,     // I contain chunks that contain chunks that contain chunks that contain chunks that contain World Blocks. I am at the very top!
// }

/// This is called chunk because it represents a big chunk of data.
#[derive(Clone, Debug, Copy)]
pub struct Chunk<T> {
	// pub position: ChunkPosition,
	// pub unload: bool,
	pub cells: [[[T; 32]; 32]; 32],
}

impl<T> Chunk<T>
where
	T: Default + Copy,
{
	pub fn new() -> Self {
		Self {
			cells: [[[T::default(); 32]; 32]; 32],
		}
	}
}

// impl Chunk {
// 	pub fn put_block(&mut self) {

// 	}
// 	pub fn get_block(&self, coords: Coordinates) -> Block {
// 		// if (ChunkPosition {
// 		// 	x: (coords.x / 32.0).floor() as i32,
// 		// 	y: (coords.y / 32.0).floor() as i32,
// 		// 	z: (coords.z / 32.0).floor() as i32,
// 		// }) == self.position
// 		// {
// 			// Some(
// 				self.voxels[(coords.x % 32.0) as usize][(coords.y % 32.0) as usize]
// 					[(coords.z % 32.0) as usize]
// 			// )
// 		// } else {
// 		// 	None
// 		// }
// 	}
// }

// pub type ChunkList = Vec<Chunk>;

// pub fn chunk_gen() {
// 	let chunk = Chunk {
// 		position: ChunkPosition { x: 7, y: 8, z: 6 },
// 		unload: false,
// 		voxels: [[[Block::Dirt; 32]; 32]; 32]
// 	};
// 	let mut chunklist = Box::new(vec![chunk; 2]);
// 	println!("{:?}", chunklist); // Testing
// 	let mut chunk_index = 0;
// 	chunklist[0].unload = true; // Testing
// 	chunklist[1].unload = true; // Testing
// 	let mut chunk_remove_index = vec![];
// 	for chunk in &*chunklist {
// 		if chunk.unload {
// 			chunk_remove_index.push(chunk_index);
// 		}
// 		chunk_index += 1;
// 	}
// 	chunk_remove_index.sort();
// 	chunk_remove_index.reverse();
// 	for index in chunk_remove_index {
// 		chunklist.remove(index);
// 	}
// 	println!("{:?}", chunklist); // Testing
// }
