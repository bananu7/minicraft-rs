use crate::render::pipeline::Pipeline;
use crate::world::coord::{OuterChunkCoord};
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
    ibo: glium::IndexBuffer<u16>,
    display: &'a glium::Display,
}

impl<'a> DisplayChunk<'a> {
    pub fn new(_coord: OuterChunkCoord, display: &'a glium::Display) -> Self {
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

            // Ground plane
            Vertex { position: [00.0, 0.0,  0.0], color: [0.3, 0.3, 0.3] },
            Vertex { position: [00.0, 0.0, 10.0], color: [0.3, 0.3, 0.3] },
            Vertex { position: [10.0, 0.0, 10.0], color: [0.3, 0.3, 0.3] },
            Vertex { position: [10.0, 0.0,  0.0], color: [0.3, 0.3, 0.3] },
        ];

        let vertex_buffer = {
            glium::VertexBuffer::new(display, &vertex_list).unwrap()
        };

        let index_buffer = glium::IndexBuffer::new(
            display,
            PrimitiveType::TrianglesList,
            &[
              0u16, 1, 2, 0, 2, 3,
              4, 5, 6, 4, 6, 7,
              8, 9,10, 8,10,11,
              12,13,14,12,14,15,
              16,17,18,16,18,19,
              20,21,22,20,22,23,
              24,25,26,24,26,27,
            ]
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

