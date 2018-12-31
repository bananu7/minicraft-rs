use crate::render::bmfont::*;

use glium::{Surface};
use glium::{uniform, program, implement_vertex};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    source: [f32; 2],
}
implement_vertex!(Vertex, position);

fn create_font_program(display : &glium::Display) -> glium::Program {
    let program = program!(display,
        330 => {
            vertex: "
                #version 330 core
                in vec2 position;
                in vec2 source;
                out vec2 vSource;
                void main() {
                    gl_Position = vec4(position / 800.0, 0.0, 1.0);
                    vSource = source;
                }
            ",
            fragment: "
                #version 330 core
                in vec2 source;
                uniform sampler2D tex;
                out vec4 f_color;

                void main() {
                    f_color = texture(source, tex);
                }
            "
        },
    ).unwrap();
    return program
}

struct DisplayFont {
    fd: FontDescriptor,

    vbo: glium::VertexBuffer<Vertex>,
    program: glium::Program,
}

impl DisplayFont {
    // Generate the display rendition for a chunk
    pub fn new(fd: FontDescriptor, display: &glium::Display) -> Self {
        let mut verts = Vec::new();

        for (_i,c) in &fd.data {
            let w = c.width as f32;
            let h = c.height as f32;
            let xs = fd.x_size as f32;
            let ys = fd.y_size as f32;
            let x = c.x as f32;
            let y = c.y as f32;
            let xo = c.x_offset as f32;
            let yo = c.y_offset as f32;

            verts.append(&mut vec![
                Vertex { position: [x, y],
                         source: [xo / xs, yo / ys] },
                Vertex { position: [x + w, y],
                         source: [(xo + w) / xs, yo] },
                Vertex { position: [x, y + h],
                         source: [xo, (yo + h) / ys] },
                Vertex { position: [x + w, y + h],
                         source: [(xo + w) / xs, (yo + h) / ys] },
            ]);
        }

        let vertex_buffer = {
            glium::VertexBuffer::new(display, &verts).unwrap()
        };

        DisplayFont {
            fd: fd,
            vbo: vertex_buffer,
            program: create_font_program(&display),
        }
    }

    pub fn print(self: &Self, target: &mut glium::Frame, s: &str) -> Result<(), glium::DrawError> {
        const NO_INDICES: glium::index::NoIndices =
            glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        let draw_parameters = glium::DrawParameters {
            .. Default::default()
        };

        for c in s.chars() {
            target.draw(&self.vbo, &NO_INDICES, &self.program, &uniforms, &draw_parameters)?;
        }

        Ok(())
    }
}