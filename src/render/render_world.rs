use crate::render::pipeline::Pipeline;
use crate::world::coord::{OuterChunkCoord, InnerChunkCoord, WorldCoord};
use crate::world::{Field, Chunk, SIZE, combine_coord};
use glium::{Surface};
use glium::index::PrimitiveType;
use glium::{uniform, implement_vertex};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
    normal: [f32; 3],
}
implement_vertex!(Vertex, position, color, normal);

struct DisplayChunk {
    vbo: glium::VertexBuffer<Vertex>,
    ibo: glium::IndexBuffer<u32>,
}

fn add_coord(position: [f32; 3], coord: &WorldCoord) -> [f32; 3] {
    [
        position[0] + coord.x as f32,
        position[1] + coord.y as f32,
        position[2] + coord.z as f32,
    ]
}

fn generate_cube(coord: WorldCoord) -> Vec<Vertex> {
    let vertex_list = [
        // Front
        Vertex { position: [0.0, 0.0, 0.0], color: [0.0, 0.8, 0.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [0.0, 1.0, 0.0], color: [0.0, 0.8, 0.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [1.0, 1.0, 0.0], color: [0.0, 0.8, 0.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [1.0, 0.0, 0.0], color: [0.0, 0.8, 0.0], normal: [0.0, 0.0, -1.0] },

        // Back
        Vertex { position: [0.0, 0.0, 1.0], color: [0.8, 0.0, 0.0], normal: [0.0, 0.0, 1.0] },
        Vertex { position: [0.0, 1.0, 1.0], color: [0.8, 0.0, 0.0], normal: [0.0, 0.0, 1.0] },
        Vertex { position: [1.0, 1.0, 1.0], color: [0.8, 0.0, 0.0], normal: [0.0, 0.0, 1.0] },
        Vertex { position: [1.0, 0.0, 1.0], color: [0.8, 0.0, 0.0], normal: [0.0, 0.0, 1.0] },

        // Top
        Vertex { position: [0.0, 1.0, 0.0], color: [0.0, 0.0, 0.8], normal: [0.0, 1.0, 0.0] },
        Vertex { position: [0.0, 1.0, 1.0], color: [0.0, 0.0, 0.8], normal: [0.0, 1.0, 0.0] },
        Vertex { position: [1.0, 1.0, 1.0], color: [0.0, 0.0, 0.8], normal: [0.0, 1.0, 0.0] },
        Vertex { position: [1.0, 1.0, 0.0], color: [0.0, 0.0, 0.8], normal: [0.0, 1.0, 0.0] },

        // Bottom
        Vertex { position: [0.0, 0.0, 0.0], color: [0.0, 0.8, 0.8], normal: [0.0, -1.0, 0.0] },
        Vertex { position: [0.0, 0.0, 1.0], color: [0.0, 0.8, 0.8], normal: [0.0, -1.0, 0.0] },
        Vertex { position: [1.0, 0.0, 1.0], color: [0.0, 0.8, 0.8], normal: [0.0, -1.0, 0.0] },
        Vertex { position: [1.0, 0.0, 0.0], color: [0.0, 0.8, 0.8], normal: [0.0, -1.0, 0.0] },

        // Right
        Vertex { position: [1.0, 0.0, 1.0], color: [0.8, 0.8, 0.0], normal: [1.0, 0.0, 0.0] },
        Vertex { position: [1.0, 1.0, 1.0], color: [0.8, 0.8, 0.0], normal: [1.0, 0.0, 0.0] },
        Vertex { position: [1.0, 1.0, 0.0], color: [0.8, 0.8, 0.0], normal: [1.0, 0.0, 0.0] },
        Vertex { position: [1.0, 0.0, 0.0], color: [0.8, 0.8, 0.0], normal: [1.0, 0.0, 0.0] },

        // Left
        Vertex { position: [0.0, 0.0, 1.0], color: [0.8, 0.0, 0.8], normal: [-1.0, 0.0, 0.0] },
        Vertex { position: [0.0, 1.0, 1.0], color: [0.8, 0.0, 0.8], normal: [-1.0, 0.0, 0.0] },
        Vertex { position: [0.0, 1.0, 0.0], color: [0.8, 0.0, 0.8], normal: [-1.0, 0.0, 0.0] },
        Vertex { position: [0.0, 0.0, 0.0], color: [0.8, 0.0, 0.8], normal: [-1.0, 0.0, 0.0] },
    ];

    let translated_list = vertex_list.into_iter().map(|v| {
        Vertex { position: add_coord(v.position, &coord), color: v.color, normal: v.normal }
    });

    return translated_list.collect()
}

impl DisplayChunk {
    // Generate the display rendition for a chunk
    pub fn new(chunk_coord: OuterChunkCoord, chunk: &Chunk, display: &glium::Display) -> Self {
        let mut vertices = Vec::new();
        let index_list = 
        [
              0u32, 1, 2, 0, 2, 3,
              4, 5, 6, 4, 6, 7,
              8, 9,10, 8,10,11,
              12,13,14,12,14,15,
              16,17,18,16,18,19,
              20,21,22,20,22,23,
        ];

        let mut indices = Vec::new();
        let mut index_offset = 0;

        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    let coord = InnerChunkCoord::new(x,y,z);
                    let block = chunk.get(&coord);
                    if block.value > 0 {
                        vertices.extend(generate_cube(combine_coord(coord, chunk_coord.clone())));
                        indices.extend(index_list.iter().map(|i|{ i + index_offset * 24}));
                        index_offset += 1;
                    }
                }
            }
        }

        let vertex_buffer = {
            glium::VertexBuffer::new(display, &vertices).unwrap()
        };

        let index_buffer = glium::IndexBuffer::new(
            display,
            PrimitiveType::TrianglesList,
            &indices
        ).unwrap();

        DisplayChunk {
            vbo: vertex_buffer,
            ibo: index_buffer,            
        }
    }

    pub fn draw(self: &Self, target: &mut glium::Frame, pip: &Pipeline) {
        // building the uniforms
        let matrix = pip.get_vp_matrix();
        let uniforms = uniform! {
            matrix: matrix,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        target.draw(&self.vbo, &self.ibo, &pip.get_program(), &uniforms, &params).unwrap();
    }
}

pub struct DisplayField {
    
}

impl DisplayField {
    pub fn draw(self: &Self, target: &mut glium::Frame, display: &glium::Display, field: &Field, pip: &Pipeline) {
        let chunks = field.get_chunks();
        let mut display_chunks = Vec::new();

        for (coord, chunk) in chunks.get_map() {
            display_chunks.push(DisplayChunk::new((*coord).clone(), chunk, display));
        }

        for dc in &display_chunks {        
            dc.draw(target, pip);
        }
    }
}