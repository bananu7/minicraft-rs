use std::marker::PhantomData;
use std::ops;
use std::hash::Hash;
use std::hash::Hasher;

use crate::world::orientation::Orientation;

#[derive(Debug, Clone, Copy)]
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

    pub fn neighbour(self: &Self, o: Orientation) -> Coord<CoordSystemTag> {
        match o {
            Orientation::XMinus => Coord::new(self.x - 1, self.y, self.z),
            Orientation::YMinus => Coord::new(self.x, self.y - 1, self.z),
            Orientation::ZMinus => Coord::new(self.x, self.y, self.z - 1),
            Orientation::XPlus => Coord::new(self.x + 1, self.y, self.z),
            Orientation::YPlus => Coord::new(self.x, self.y + 1, self.z),
            Orientation::ZPlus => Coord::new(self.x, self.y, self.z + 1)
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


#[derive(Debug, Clone)]
pub struct InnerChunkCoordTag;
#[derive(Debug, Clone)]
pub struct OuterChunkCoordTag;
#[derive(Debug, Clone)]
pub struct WorldCoordTag;

pub type InnerChunkCoord = Coord<InnerChunkCoordTag>;
pub type OuterChunkCoord = Coord<OuterChunkCoordTag>;
pub type WorldCoord = Coord<WorldCoordTag>;
