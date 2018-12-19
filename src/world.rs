use std::marker::PhantomData;
use std::ops;

pub struct Coord<CoordSystemTag> {
	x: i64,
	y: i64,
	z: i64,

	tag: PhantomData<CoordSystemTag>
}

impl<Tag> ops::Add<Coord<Tag>> for Coord<Tag> {
    type Output = Coord<Tag>;

    fn add(self, _rhs: Coord<Tag>) -> Coord<Tag> {
    	return Coord::<Tag> { 
    		x: self.x + _rhs.x,
    		y: self.y + _rhs.y,
    		z: self.z + _rhs.z,
    		tag: PhantomData
    	}
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
