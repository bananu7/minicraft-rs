use crate::render::world::display_chunk::{Vertex, DisplayChunk};
use crate::render::world::traits::{DisplayChunkGen};

use crate::world::{Chunk, SIZE, combine_coord};
use crate::world::coord::{OuterChunkCoord, InnerChunkCoord, WorldCoord};
use crate::world::orientation::Orientation;

fn add_coord(position: [f32; 3], coord: &WorldCoord) -> [f32; 3] {
    [
        position[0] + coord.x as f32,
        position[1] + coord.y as f32,
        position[2] + coord.z as f32,
    ]
}

fn generate_cube(value: u64, coord: WorldCoord) -> Vec<Vertex> {
    let green = [0.259, 0.6, 0.012];
    let brown = [0.388, 0.306, 0.161];

    let tex_in_atlas = 4;
    let tc_width = 1.0 / (tex_in_atlas as f32);
    let tc_height = 1.0 / (tex_in_atlas as f32);

    let tc_left = ((value-1) % tex_in_atlas) as f32 * tc_width;
    let tc_bottom = ((value-1) / tex_in_atlas) as f32 * tc_width;
    let tc_right = tc_left + tc_width;
    let tc_top = tc_bottom + tc_height;

    let vertex_list = [
        // Front
        Vertex { position: [0.0, 0.0, 0.0], color: brown, normal: [0.0, 0.0, -1.0], texCoord: [tc_left, tc_bottom] },
        Vertex { position: [0.0, 1.0, 0.0], color: brown, normal: [0.0, 0.0, -1.0], texCoord: [tc_left, tc_top] },
        Vertex { position: [1.0, 1.0, 0.0], color: brown, normal: [0.0, 0.0, -1.0], texCoord: [tc_right, tc_top] },
        Vertex { position: [1.0, 0.0, 0.0], color: brown, normal: [0.0, 0.0, -1.0], texCoord: [tc_right, tc_bottom] },

        // Back
        Vertex { position: [0.0, 0.0, 1.0], color: brown, normal: [0.0, 0.0, 1.0], texCoord: [tc_left, tc_bottom] },
        Vertex { position: [0.0, 1.0, 1.0], color: brown, normal: [0.0, 0.0, 1.0], texCoord: [tc_left, tc_top] },
        Vertex { position: [1.0, 1.0, 1.0], color: brown, normal: [0.0, 0.0, 1.0], texCoord: [tc_right, tc_top] },
        Vertex { position: [1.0, 0.0, 1.0], color: brown, normal: [0.0, 0.0, 1.0], texCoord: [tc_right, tc_bottom] },

        // Top
        Vertex { position: [0.0, 1.0, 0.0], color: green, normal: [0.0, 1.0, 0.0], texCoord: [tc_left, tc_bottom] },
        Vertex { position: [0.0, 1.0, 1.0], color: green, normal: [0.0, 1.0, 0.0], texCoord: [tc_left, tc_top] },
        Vertex { position: [1.0, 1.0, 1.0], color: green, normal: [0.0, 1.0, 0.0], texCoord: [tc_right, tc_top] },
        Vertex { position: [1.0, 1.0, 0.0], color: green, normal: [0.0, 1.0, 0.0], texCoord: [tc_right, tc_bottom] },

        // Bottom
        Vertex { position: [0.0, 0.0, 0.0], color: brown, normal: [0.0, -1.0, 0.0], texCoord: [tc_left, tc_bottom] },
        Vertex { position: [0.0, 0.0, 1.0], color: brown, normal: [0.0, -1.0, 0.0], texCoord: [tc_left, tc_top] },
        Vertex { position: [1.0, 0.0, 1.0], color: brown, normal: [0.0, -1.0, 0.0], texCoord: [tc_right, tc_top] },
        Vertex { position: [1.0, 0.0, 0.0], color: brown, normal: [0.0, -1.0, 0.0], texCoord: [tc_right, tc_bottom] },

        // Right
        Vertex { position: [1.0, 0.0, 1.0], color: brown, normal: [1.0, 0.0, 0.0], texCoord: [tc_left, tc_bottom] },
        Vertex { position: [1.0, 1.0, 1.0], color: brown, normal: [1.0, 0.0, 0.0], texCoord: [tc_left, tc_top] },
        Vertex { position: [1.0, 1.0, 0.0], color: brown, normal: [1.0, 0.0, 0.0], texCoord: [tc_right, tc_top] },
        Vertex { position: [1.0, 0.0, 0.0], color: brown, normal: [1.0, 0.0, 0.0], texCoord: [tc_right, tc_bottom] },

        // Left
        Vertex { position: [0.0, 0.0, 1.0], color: brown, normal: [-1.0, 0.0, 0.0], texCoord: [tc_left, tc_bottom] },
        Vertex { position: [0.0, 1.0, 1.0], color: brown, normal: [-1.0, 0.0, 0.0], texCoord: [tc_left, tc_top] },
        Vertex { position: [0.0, 1.0, 0.0], color: brown, normal: [-1.0, 0.0, 0.0], texCoord: [tc_right, tc_top] },
        Vertex { position: [0.0, 0.0, 0.0], color: brown, normal: [-1.0, 0.0, 0.0], texCoord: [tc_right, tc_bottom] },
    ];

    let translated_list: Vec<Vertex> = vertex_list.iter().map(|v| {
        Vertex { position: add_coord(v.position, &coord), color: v.color, normal: v.normal, texCoord: v.texCoord }
    }).collect();

    let index_list = 
    [
          0u32, 1, 2, 0, 2, 3,
          4, 5, 6, 4, 6, 7,
          8, 9,10, 8,10,11,
          12,13,14,12,14,15,
          16,17,18,16,18,19,
          20,21,22,20,22,23,
    ];

    let expanded_size = 6 * 2 * 3; // 6 faces, two triangles each
    let mut expanded_list = Vec::with_capacity(expanded_size);

    for index in index_list.iter() {
        expanded_list.push(translated_list[*index as usize]);
    }

    return expanded_list
}

pub struct DisplayChunkGenCold {

}

impl DisplayChunkGenCold {
    pub fn new(_display: &glium::Display) -> Self { DisplayChunkGenCold {} }
}

impl DisplayChunkGen for DisplayChunkGenCold {
    fn generate(self: &mut Self, chunk_coord: OuterChunkCoord, chunk: &Chunk, display: &glium::Display) -> DisplayChunk {
        let mut vertices = Vec::new();

        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    let coord = InnerChunkCoord::new(x,y,z);
                    let block = chunk.get(&coord);

                    if block.value == 0 {
                        continue;
                    }

                    // TODO - make this less bad
                    // check neighbours
                    { 
                        let not_on_edge =
                               coord.x > 0 
                            && coord.y > 0
                            && coord.z > 0
                            && coord.x < SIZE-1
                            && coord.y < SIZE-1
                            && coord.z < SIZE-1;

                        if not_on_edge {
                            let surrounded_on_all_sides =
                                   chunk.get(&coord.neighbour(Orientation::XMinus)).value != 0
                                && chunk.get(&coord.neighbour(Orientation::YMinus)).value != 0
                                && chunk.get(&coord.neighbour(Orientation::ZMinus)).value != 0
                                && chunk.get(&coord.neighbour(Orientation::XPlus)).value != 0
                                && chunk.get(&coord.neighbour(Orientation::YPlus)).value != 0
                                && chunk.get(&coord.neighbour(Orientation::ZPlus)).value != 0;

                            if surrounded_on_all_sides {
                                continue;
                            }
                        }
                    }

                    {
                        vertices.extend(generate_cube(block.value, combine_coord(coord, chunk_coord.clone())));
                    }
                }
            }
        }

        let vertex_buffer = {
            glium::VertexBuffer::new(display, &vertices).unwrap()
        };

        DisplayChunk {
            vbo: vertex_buffer,
        }
    }
}