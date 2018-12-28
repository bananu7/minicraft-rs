use glm::*;
use crate::world::{WorldCoord};

/*
This is the raycasting algorithm. It works by finding the exit point
from the current voxel, storing the voxel it enters, and repeating.

The 't' name used throughout means "time" or "distance" spent during the
casting, and is limited by the 'len' param.

It returns the list of traversed voxels.
*/

pub struct RaycastParams {
    pub pos: Vec3,
    pub dir: Vec3,
    pub len: f32,

    pub include_first: bool,
}

fn pos_to_coord(p: Vec3) -> WorldCoord {
    WorldCoord::new(
        p.x.floor() as i64,
        p.y.floor() as i64,
        p.z.floor() as i64
    )
}

fn d_to_next_plane(dir_part: f32, f: f32) -> f32 {
    if dir_part > 0.0 {
        let m = if f == f.floor() { 1.0 } else { 0.0 };
        return f - f.floor() - m
    } else {
        let m = if f == f.ceil() { 1.0 } else { 0.0 };
        return f.ceil() - f + m
    }
}

fn determine_next_hit(dir: Vec3, p: Vec3) -> Vec3 {
    let d_to_x_plane = d_to_next_plane(dir.x, p.x);
    let d_to_y_plane = d_to_next_plane(dir.y, p.x);
    let d_to_z_plane = d_to_next_plane(dir.x, p.x);

    let t_to_x_plane = (d_to_x_plane / dir.x).abs();
    let t_to_y_plane = (d_to_y_plane / dir.y).abs();
    let t_to_z_plane = (d_to_z_plane / dir.z).abs();

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

pub fn raycast(params: RaycastParams) -> Vec<WorldCoord> {
    let mut blocks = Vec::new();
    let mut p = params.pos;

    if params.include_first {
        let starting_coord = pos_to_coord(p);
        blocks.push(starting_coord);
    }

    p = determine_next_hit(params.dir, p);
    while glm::length(params.pos - p) < params.len {
        //print!("Traveled {} so far\n", glm::length(params.pos - p));
        //print!("Next point ({}, {}, {})\n", p.x, p.y, p.z);
        blocks.push(pos_to_coord(p));
        p = determine_next_hit(params.dir, p);
    }

    return blocks
}