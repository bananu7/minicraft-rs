use crate::world::from_world_to_local;
use crate::world::coord::{InnerChunkCoord, OuterChunkCoord};

use super::*;

#[test]
fn test_from_world_to_local() {
    assert_eq!(
        from_world_to_local(WorldCoord::new(0,0,0)),
        (OuterChunkCoord::new(0,0,0), InnerChunkCoord::new(0,0,0))
    );
}