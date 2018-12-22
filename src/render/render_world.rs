use crate::world::coord::{OuterChunkCoord};
use glium::{Surface};
use glium::index::PrimitiveType;
use glium::{uniform, implement_vertex};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

pub struct DisplayChunk<'a> {
    vbo: glium::VertexBuffer<Vertex>,
    ibo: glium::IndexBuffer<u16>,
    display: &'a glium::Display,
}

impl<'a> DisplayChunk<'a> {
    pub fn new(coord: OuterChunkCoord, display: &'a glium::Display) -> Self {
        let vertex_buffer = {
            glium::VertexBuffer::new(display,
                &[
                    Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                    Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
                    Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
                ]
            ).unwrap()
        };

        let index_buffer = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList,
                                           &[0u16, 1, 2]).unwrap();

        DisplayChunk {
            vbo: vertex_buffer,
            ibo: index_buffer,
            display: display,
        }
    }

    pub fn draw(self: &Self, program: &glium::Program) {
        // building the uniforms
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        // drawing a frame
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&self.vbo, &self.ibo, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    }
}

