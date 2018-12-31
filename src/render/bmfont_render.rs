use crate::render::bmfont::*;

use glium::{Surface};
use glium::{uniform, program, implement_vertex};

use std::io::Cursor;
use std::collections::HashMap;

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
                uniform vec2 offset;
                in vec2 position;
                in vec2 source;
                out vec2 vSource;
                void main() {
                    gl_Position = vec4((position + offset) / 320.0, 0.0, 1.0);
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
    char_to_num: HashMap<i64, i64>,

    vbo: glium::VertexBuffer<Vertex>,
    program: glium::Program,
    texture: glium::texture::CompressedSrgbTexture2d,
}

impl DisplayFont {
    // Generate the display rendition for a chunk
    pub fn new(fd: FontDescriptor, display: &glium::Display) -> Self {
        let mut verts = Vec::new();
        let mut char_to_num = HashMap::new();
        let mut num = 0;

        for (i,c) in &fd.data {
            let w = c.width as f32;
            let h = c.height as f32;
            let xs = fd.x_size as f32;
            let ys = fd.y_size as f32;
            let x = c.x as f32;
            let y = c.y as f32;
            let xo = c.x_offset as f32;
            let yo = c.y_offset as f32;

            char_to_num.insert(*i, num);
            num += 1;

            verts.append(&mut vec![
                Vertex { position: [xo,     -yo      ], source: [x    /xs,  1.0 - y    /ys] },
                Vertex { position: [xo + w, -yo      ], source: [(x+w)/xs,  1.0 - y    /ys] },
                Vertex { position: [xo,     -(yo + h)], source: [x    /xs,  1.0 - (y+h)/ys] },
                Vertex { position: [xo + w, -(yo + h)], source: [(x+w)/xs,  1.0 - (y+h)/ys] },
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
            char_to_num: char_to_num,
            vbo: vertex_buffer,
            program: create_font_program(&display),
            texture: opengl_texture,
        }
    }

    fn print_single(self: &Self, target: &mut glium::Frame, char_idx: usize, offset: [f32; 2])
        -> Result<(), glium::DrawError>
    {
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
            offset: offset,
        };

        let draw_parameters = glium::DrawParameters {
            .. Default::default()
        };

        target.draw(self.vbo.slice(char_idx..(char_idx+4)).unwrap(), &NO_INDICES, &self.program, &uniforms, &draw_parameters)?;
        Ok(())
    }

    pub fn print(self: &Self, target: &mut glium::Frame, s: &str) -> Result<(), glium::DrawError> {
        let mut x_offset = 0.0;

        for c in s.chars() {
            let n = self.char_to_num.get(&(c as i64));
            let cd = self.fd.data.get(&(c as i64));


            if let (Some(n)) = n {
                let ci = *n as usize * 4;

                self.print_single(target, ci, [x_offset, 0.0])?;
                x_offset += cd.unwrap().x_advance as f32;
            }
        }

        Ok(())
    }
}