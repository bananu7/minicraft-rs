pub mod default_hash_map;
pub mod coord;

use self::coord::WorldCoord;
use self::coord::OuterChunkCoord;
use self::coord::InnerChunkCoord;
use self::default_hash_map::DefaultHashMap;

pub const SIZE: i64 = 16;

fn from_world_to_local(wc: WorldCoord) -> (OuterChunkCoord, InnerChunkCoord) {
    let oc = OuterChunkCoord::new(wc.x % SIZE, wc.y % SIZE, wc.z % SIZE);
    let ic = InnerChunkCoord::new(wc.x / SIZE, wc.y / SIZE, wc.z / SIZE);
    return (oc,ic)
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Orientation {
    Up, Down, Left, Right, Front, Back
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Block {
    pub value: u64,
    pub orientation: Orientation,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Chunk {
    data: Vec<Block>
}
impl Chunk {
    pub fn new() -> Self {
        Chunk {
            data: vec![Block { value: 0, orientation: Orientation::Up }; (SIZE * SIZE * SIZE) as usize],
        }
    }

    pub fn fill(&mut self) {
        for i in 0..SIZE*SIZE*SIZE {
            if i % 13 == 0 {
                self.data[i as usize] = Block { value: 1, orientation: Orientation::Up };
            }
        }
    }

    pub fn get(&self, c: &InnerChunkCoord) -> &Block {
        unsafe {
            return self.data.get_unchecked((c.x + c.y * SIZE + c.z * SIZE * SIZE) as usize)
        }
    }
}

pub struct Field {
    chunks: DefaultHashMap<OuterChunkCoord, Chunk>,
}

impl Field {
    pub fn new() -> Self {
        Field {
            chunks: DefaultHashMap::new(Chunk::new())
        }
    }

    pub fn get(&self, c: WorldCoord) -> &Block {
        let (outer_coord, inner_coord) = from_world_to_local(c);
        let chunk = self.chunks.get(&outer_coord);
        return chunk.get(&inner_coord)
    }

    pub fn fill(&mut self) {
        self.chunks.get_mut(OuterChunkCoord::new(0,0,0)).fill();        
    }
}

pub fn setup() -> Field {
    let mut f = Field::new();
    f.fill();
    return f
}
