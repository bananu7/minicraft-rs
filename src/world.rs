pub mod default_hash_map;
pub mod coord;
pub mod raycast;
pub mod orientation;

use self::orientation::Orientation;
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
            data: vec![Block { value: 0, orientation: Orientation::YPlus }; (SIZE * SIZE * SIZE) as usize],
        }
    }

    pub fn fill(&mut self) {
        for x in 0..SIZE {
            for z in 0..SIZE {
                let e = (((x as f32 + z as f32) / 10.0).sin() + 1.0) * 3.0 + 1.0;
                let h = e as i64;

                let v = if x % 2 == 0 { 5 } else { 5 };
                for y in 0..h {
                    *self.get_mut(&InnerChunkCoord::new(x,y,z)) = Block { value: v, orientation: Orientation::YPlus };
                }
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

    #[allow(dead_code)]
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
        for x in 0..2 {
            for z in 0..2 {
                self.chunks.get_mut(OuterChunkCoord::new(x,0,z)).fill();
            }
        }
        //self.chunks.get_mut(OuterChunkCoord::new(0,0,0)).fill();
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
