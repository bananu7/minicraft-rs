use std::marker::PhantomData;
use std::ops;
use std::hash::Hash;
use std::hash::Hasher;

pub struct Coord<CoordSystemTag> {
	pub x: i64,
	pub y: i64,
	pub z: i64,

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

impl<Tag> PartialEq for Coord<Tag> {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x &&
        self.y == rhs.y &&
        self.z == rhs.z
    }
}
impl<Tag> Eq for Coord<Tag> {}

impl<Tag> Hash for Coord<Tag> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}


pub struct InnerChunkCoordTag;
pub struct OuterChunkCoordTag;
pub struct WorldCoordTag;
pub type InnerChunkCoord = Coord<InnerChunkCoordTag>;
pub type OuterChunkCoord = Coord<OuterChunkCoordTag>;
pub type WorldCoord = Coord<WorldCoordTag>;