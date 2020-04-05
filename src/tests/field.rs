use crate::world::from_world_to_local;
use crate::world::coord::{InnerChunkCoord, OuterChunkCoord, WorldCoord};
use crate::world::SIZE;

// Those helpers are just to shorten the code somewhat
// Not that Rust could use unnamed uniform initialization
// which I despised in C++ before but now, esp. for test
// code actually seems nice. Well there are macros I guess...
fn w(x: i64, y: i64, z: i64) -> WorldCoord {
    WorldCoord::new(x, y, z)
}

fn o(x: i64, y: i64, z: i64) -> OuterChunkCoord {
    OuterChunkCoord::new(x, y, z)
}

fn i(x: i64, y: i64, z: i64) -> InnerChunkCoord {
    InnerChunkCoord::new(x, y, z)
}

#[test]
fn test_from_world_to_local_trivial() {
    assert_eq!(
        from_world_to_local(w(0,0,0)),
        (o(0,0,0), i(0,0,0))
    );

    assert_eq!(
        from_world_to_local(w(1,0,0)),
        (o(0,0,0), i(1,0,0))
    );
}

#[test]
fn test_from_world_to_local_cross() {
    assert_eq!(
        from_world_to_local(w(SIZE,0,0)),
        (o(1,0,0), i(0,0,0))
    );

    assert_eq!(
        from_world_to_local(w(SIZE+1,0,0)),
        (o(1,0,0), i(1,0,0))
    );
}

#[test]
fn test_from_world_to_local_negative() {
    assert_eq!(
        from_world_to_local(w(-1,0,0)),
        (o(-1,0,0), i(SIZE-1,0,0)),
        "world x: -1"
    );

    assert_eq!(
        from_world_to_local(w(-SIZE,0,0)),
        (o(-1,0,0), i(0,0,0)),
        "world x: -SIZE ({})",
        -SIZE
    );

    assert_eq!(
        from_world_to_local(w(-SIZE - 1,0,0)),
        (o(-2,0,0), i(SIZE-1,0,0)),
        "world x: -SIZE-1 ({})",
        -SIZE-1
    );

    assert_eq!(
        from_world_to_local(w(- 2*SIZE,0,0)),
        (o(-2,0,0), i(0,0,0)),
        "world x: -SIZE-1 ({})",
        -SIZE-1
    );
}