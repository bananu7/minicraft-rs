use glm::*;
use crate::world::{Field, WorldCoord};

/*
This is the raycasting algorithm. It works by finding the exit point
from the current voxel, storing the voxel it enters, and repeating.

The 't' name used throughout means "time" or "distance" spent during the
casting, and is limited by the 'len' param.
*/

struct RaycastParams {
    pos: Vec3,
    dir: Vec3,
    len: f32,

    include_first: bool,
}

fn pos_to_coord(p: Vec3) -> WorldCoord {
    WorldCoord::new(
        p.x.floor() as i64,
        p.y.floor() as i64,
        p.z.floor() as i64
    )
}

fn determine_next_hit(dir: Vec3, p: Vec3) -> Vec3 {
    let t_to_x_plane = if dir.x > 0.0 { p.x.ceil() - p.x } else { p.x - p.x.floor() };
    let t_to_y_plane = if dir.y > 0.0 { p.y.ceil() - p.y } else { p.y - p.y.floor() };
    let t_to_z_plane = if dir.z > 0.0 { p.z.ceil() - p.z } else { p.z - p.z.floor() };

    // x
    if t_to_x_plane <= t_to_y_plane && t_to_x_plane <= t_to_z_plane {
        return p + dir * t_to_x_plane;
    }
    // y
    else if t_to_y_plane <= t_to_z_plane && t_to_y_plane <= t_to_z_plane {
        return p + dir * t_to_y_plane;
    }
    // z
    else {
        return p + dir * t_to_z_plane;
    }
}

fn raycast(field: &Field, params: RaycastParams) {
    let mut blocks = Vec::new();
    let mut p = params.pos;

    if params.include_first {
        let starting_coord = pos_to_coord(p);
        blocks.push(starting_coord);
    }

    while glm::length(params.pos - p) < params.len {
        p = determine_next_hit(params.dir, p);
        // if field(pos_to_coord(p).value != 0) {
        blocks.push(pos_to_coord(p));
    }
}