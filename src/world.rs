use std::marker::PhantomData;
use std::ops;

pub struct Coord<CoordSystemTag> {
	x: i64,
	y: i64,
	z: i64,

	_tag: PhantomData<CoordSystemTag>
}

impl<CoordSystemTag> Coord<CoordSystemTag> {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Coord {
            x,
            y,
            z,
            _tag: PhantomData,
        }
    }
}

impl<Tag> ops::Add for Coord<Tag> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}


struct InnerChunkCoordTag;
struct OuterChunkCoordTag;
type InnerChunkCoord = Coord<InnerChunkCoordTag>;
type OuterChunkCoord = Coord<OuterChunkCoordTag>;

struct World {

}

pub fn setup() {

}
