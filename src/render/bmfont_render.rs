use crate::render::bmfont::*;

use glium::{Surface};
use glium::{uniform, program, implement_vertex};

use std::io::Cursor;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    source: [f32; 2],
}
implement_vertex!(Vertex, position, source);

fn create_font_program(display : &glium::Display) -> glium::Program {
    let program = program!(display,
        330 => {
            vertex: "
                #version 330 core
                in vec2 position;
                in vec2 source;
                out vec2 vSource;
                void main() {
                    gl_Position = vec4(position / 40.0, 0.0, 1.0);
                    vSource = source;
                }
            ",
            fragment: "
                #version 330 core
                in vec2 vSource;
                uniform sampler2D tex;
                out vec4 f_color;

                void main() {
                    f_color = texture(tex, vSource);
                }
            "
        },
    ).unwrap();
    return program
}

pub struct DisplayFont {
    fd: FontDescriptor,

    vbo: glium::VertexBuffer<Vertex>,
    program: glium::Program,
    texture: glium::texture::CompressedSrgbTexture2d,
}

impl DisplayFont {
    // Generate the display rendition for a chunk
    pub fn new(fd: FontDescriptor, display: &glium::Display) -> Self {
        let mut verts = Vec::new();

        for (_i,c) in &fd.data {
            if *_i != 106 {
                continue
            }

            let w = c.width as f32;
            let h = c.height as f32;
            let xs = fd.x_size as f32;
            let ys = fd.y_size as f32;
            let x = c.x as f32;
            let y = c.y as f32;
            let xo = c.x_offset as f32;
            let yo = c.y_offset as f32;

            println!("x: {} xs: {}, w: {}", x, xs, w);
            println!("y: {} ys: {}, h: {}", y, ys, h);

            println!("y/ys: {} (y+h)/ys {}", y/ys, (y+h)/ys);

            verts.append(&mut vec![
                Vertex { position: [0.0, 0.0],
                         source: [x / xs, 1.0 - (y+h)/ys] },
                Vertex { position: [w, 0.0],
                         source: [(x+w)/xs, 1.0 - (y+h)/ys] },
                Vertex { position: [0.0, h],
                         source: [x/xs, 1.0 - y / ys] },
                Vertex { position: [w, h],
                         source: [(x+w)/xs, 1.0 - y / ys] },
            ]);
        }

        let vertex_buffer = {
            glium::VertexBuffer::new(display, &verts).unwrap()
        };

        let image = image::load(Cursor::new(&include_bytes!("../../data/font.png")[..]),
                            image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let opengl_texture = glium::texture::CompressedSrgbTexture2d::new(display, image).unwrap();

        DisplayFont {
            fd: fd,
            vbo: vertex_buffer,
            program: create_font_program(&display),
            texture: opengl_texture,
        }
    }

    pub fn print(self: &Self, target: &mut glium::Frame, s: &str) -> Result<(), glium::DrawError> {
        const NO_INDICES: glium::index::NoIndices =
            glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            tex: &self.texture,
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