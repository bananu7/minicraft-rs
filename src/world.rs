pub mod default_hash_map;
pub mod coord;
pub mod raycast;

use self::coord::WorldCoord;
use self::coord::OuterChunkCoord;
use self::coord::InnerChunkCoord;
use self::default_hash_map::DefaultHashMap;

pub const SIZE: i64 = 16;

pub fn from_world_to_inner(wc: WorldCoord) -> InnerChunkCoord {
    InnerChunkCoord::new(
        if wc.x >= 0 { wc.x % SIZE } else { (wc.x % SIZE + SIZE) % SIZE },
        if wc.y >= 0 { wc.y % SIZE } else { (wc.y % SIZE + SIZE) % SIZE },
        if wc.z >= 0 { wc.z % SIZE } else { (wc.z % SIZE + SIZE) % SIZE }
    )
}

pub fn from_world_to_outer(wc: WorldCoord) -> OuterChunkCoord {
    OuterChunkCoord::new(
        if wc.x >= 0 { wc.x / SIZE } else { (wc.x + 1) / SIZE - 1 },
        if wc.y >= 0 { wc.y / SIZE } else { (wc.y + 1) / SIZE - 1 },
        if wc.z >= 0 { wc.z / SIZE } else { (wc.z + 1) / SIZE - 1 }
    )
}

pub fn from_world_to_local(wc: WorldCoord) -> (OuterChunkCoord, InnerChunkCoord) {
    return (from_world_to_outer(wc.clone()),from_world_to_inner(wc))
}

pub fn combine_coord(i: InnerChunkCoord, o: OuterChunkCoord) -> WorldCoord {
    return WorldCoord::new(i.x + o.x * SIZE, i.y + o.y * SIZE, i.z + o.z * SIZE)
}

#[allow(dead_code)]
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

    pub fn get_mut(&mut self, c: &InnerChunkCoord) -> &mut Block {
        unsafe {
            return self.data.get_unchecked_mut((c.x + c.y * SIZE + c.z * SIZE * SIZE) as usize)
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

    pub fn set(&mut self, c: WorldCoord, b: Block) {
        let (o,i) = from_world_to_local(c);
        self.chunks.get_mut(o).get_mut(&i).value = b.value;
    }

    pub fn fill(&mut self) {
        self.chunks.get_mut(OuterChunkCoord::new(0,0,0)).fill();
        self.chunks.get_mut(OuterChunkCoord::new(1,0,0)).fill();
    }

    pub fn get_chunks(&self) -> &DefaultHashMap<OuterChunkCoord, Chunk> {
        return &self.chunks
    }
}

pub fn setup() -> Field {
    let mut f = Field::new();
    f.fill();
    return f
}
