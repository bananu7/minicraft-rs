mod default_hash_map;
mod coord;
use self::coord::WorldCoord;
use self::coord::OuterChunkCoord;
use self::coord::InnerChunkCoord;

const SIZE: i64 = 16;

fn from_world_to_local(wc: WorldCoord) -> (OuterChunkCoord, InnerChunkCoord) {
    let oc = OuterChunkCoord::new(wc.x % SIZE, wc.y % SIZE, wc.z % SIZE);
    let ic = InnerChunkCoord::new(wc.x / SIZE, wc.y / SIZE, wc.z / SIZE);
    return (oc,ic)
}

enum Orientation {
    Up, Down, Left, Right, Front, Back
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Block {
    value: u64,
    //orientation: Orientation,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Chunk {
    data: Vec<Block>
}
impl Chunk {
    pub fn new() -> Self {
        Chunk {
            data: vec![Block { value: 0 }; SIZE * SIZE * SIZE as usize],
        }
    }
}

impl Chunk {
    fn get(&self, c: InnerChunkCoord) -> &Block {
        unsafe {
            return self.data.get_unchecked((c.x + c.y * SIZE + c.z * SIZE * SIZE) as usize)
        }
    }
}

pub struct Field {
    chunks: default_hash_map::DefaultHashMap<OuterChunkCoord, Chunk>,
}

impl Field {
    fn get(&self, c: WorldCoord) -> &Block {
        let (outer_coord, inner_coord) = from_world_to_local(c);
        let chunk = self.chunks.get(&outer_coord);
        return chunk.get(inner_coord)
    }
}

pub fn setup() {

}
