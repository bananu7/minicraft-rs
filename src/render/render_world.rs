use crate::render::pipeline::Pipeline;
use crate::world::coord::{OuterChunkCoord, InnerChunkCoord};
use crate::world::{Chunk, SIZE};
use glium::{Surface};
use glium::index::PrimitiveType;
use glium::{uniform, implement_vertex};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

pub struct DisplayChunk<'a> {
    vbo: glium::VertexBuffer<Vertex>,
    ibo: glium::IndexBuffer<u32>,
    display: &'a glium::Display,
}

fn add_coord(position: [f32; 3], coord: &InnerChunkCoord) -> [f32; 3] {
    [
        position[0] + coord.x as f32,
        position[1] + coord.y as f32,
        position[2] + coord.z as f32,
    ]
}

fn generate_cube(coord: InnerChunkCoord) -> Vec<Vertex> {
        let vertex_list = [
            // Front
            Vertex { position: [0.0, 0.0, 0.0], color: [0.0, 0.8, 0.0] },
            Vertex { position: [0.0, 1.0, 0.0], color: [0.0, 0.8, 0.0] },
            Vertex { position: [1.0, 1.0, 0.0], color: [0.0, 0.8, 0.0] },
            Vertex { position: [1.0, 0.0, 0.0], color: [0.0, 0.8, 0.0] },

            // Back
            Vertex { position: [0.0, 0.0, 1.0], color: [0.8, 0.0, 0.0] },
            Vertex { position: [0.0, 1.0, 1.0], color: [0.8, 0.0, 0.0] },
            Vertex { position: [1.0, 1.0, 1.0], color: [0.8, 0.0, 0.0] },
            Vertex { position: [1.0, 0.0, 1.0], color: [0.8, 0.0, 0.0] },

            // Top
            Vertex { position: [0.0, 1.0, 0.0], color: [0.0, 0.0, 0.8] },
            Vertex { position: [0.0, 1.0, 1.0], color: [0.0, 0.0, 0.8] },
            Vertex { position: [1.0, 1.0, 1.0], color: [0.0, 0.0, 0.8] },
            Vertex { position: [1.0, 1.0, 0.0], color: [0.0, 0.0, 0.8] },

            // Bottom
            Vertex { position: [0.0, 0.0, 0.0], color: [0.0, 0.8, 0.8] },
            Vertex { position: [0.0, 0.0, 1.0], color: [0.0, 0.8, 0.8] },
            Vertex { position: [1.0, 0.0, 1.0], color: [0.0, 0.8, 0.8] },
            Vertex { position: [1.0, 0.0, 0.0], color: [0.0, 0.8, 0.8] },

            // Right
            Vertex { position: [1.0, 0.0, 1.0], color: [0.8, 0.8, 0.0] },
            Vertex { position: [1.0, 1.0, 1.0], color: [0.8, 0.8, 0.0] },
            Vertex { position: [1.0, 1.0, 0.0], color: [0.8, 0.8, 0.0] },
            Vertex { position: [1.0, 0.0, 0.0], color: [0.8, 0.8, 0.0] },

            // Left
            Vertex { position: [0.0, 0.0, 1.0], color: [0.8, 0.0, 0.8] },
            Vertex { position: [0.0, 1.0, 1.0], color: [0.8, 0.0, 0.8] },
            Vertex { position: [0.0, 1.0, 0.0], color: [0.8, 0.0, 0.8] },
            Vertex { position: [0.0, 0.0, 0.0], color: [0.8, 0.0, 0.8] },
        ];

        let translated_list = vertex_list.into_iter().map(|v| {
            Vertex { position: add_coord(v.position, &coord), color: v.color }
        });

        return translated_list.collect()
}

impl<'a> DisplayChunk<'a> {
    // Generate the display rendition for a chunk
    pub fn new(_coord: OuterChunkCoord, chunk: &Chunk, display: &'a glium::Display) -> Self {
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

        let index_list2 = 
        [
              24u32, 25, 26, 24, 26, 27,
              28, 29, 30, 28, 30, 31,
              32, 33,34, 8,10,11,
              36,37,38,12,14,15,
              40,41,42,16,18,19,
              44,45,46,20,22,23,
        ];

        let mut indices = Vec::new();
        let mut index_offset = 0;

        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    let coord = InnerChunkCoord::new(x,y,z);
                    let block = chunk.get(&coord);
                    if block.value > 0 {
                        vertices.extend(generate_cube(coord));
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
            display: display,
        }
    }

    pub fn draw(self: &Self, pip: &Pipeline) {
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

        // drawing a frame
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        target.draw(&self.vbo, &self.ibo, &pip.get_program(), &uniforms, &params).unwrap();
        target.finish().unwrap();
    }
}

