use crate::render::util::glm_mat4_to_raw_array;
use glm::vec2;
use glium::{Surface};
use glium::{uniform, program, implement_vertex};

use std::collections::HashMap;

use crate::render::gui::bmfont::*;
use crate::render::camera::*;

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
                uniform mat4 matrix;
                in vec2 position;
                in vec2 source;
                out vec2 vSource;
                void main() {
                    gl_Position = matrix * vec4((position + offset), 0.0, 1.0);
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
                    // assume font is RGBA, but only white for now.
                    f_color.a = f_color.r;
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
    pub fn new(fd: FontDescriptor, display: &glium::Display) -> Result<Self, ()> {
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
                Vertex { position: [xo,     yo      ], source: [x    /xs,  1.0 - y    /ys] },
                Vertex { position: [xo + w, yo      ], source: [(x+w)/xs,  1.0 - y    /ys] },
                Vertex { position: [xo,     (yo + h)], source: [x    /xs,  1.0 - (y+h)/ys] },
                Vertex { position: [xo + w, (yo + h)], source: [(x+w)/xs,  1.0 - (y+h)/ys] },
            ]);
        }

        let vertex_buffer = {
            glium::VertexBuffer::new(display, &verts).unwrap()
        };


        let image = image::open("data/font.png")
            .map(|i| i.to_rgba() )
            .map_err(|_| () )?;

        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let opengl_texture = glium::texture::CompressedSrgbTexture2d::new(display, image).unwrap();

        Ok(DisplayFont {
            fd: fd,
            char_to_num: char_to_num,
            vbo: vertex_buffer,
            program: create_font_program(&display),
            texture: opengl_texture,
        })
    }

    fn print_single(self: &Self, target: &mut glium::Frame, char_idx: usize, offset: [f32; 2])
        -> Result<(), glium::DrawError>
    {
        const NO_INDICES: glium::index::NoIndices =
            glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

        let (screen_width, screen_height) = target.get_dimensions();

        let camera = CameraOrtho2D::new(vec2(screen_width as f32, screen_height as f32), vec2(0.0, 0.0));

        let uniforms = uniform! {
            matrix: glm_mat4_to_raw_array(camera.calculate_view()),
            tex: &self.texture,
            offset: offset,
        };

        let draw_parameters = glium::DrawParameters {
            blend: glium::draw_parameters::Blend::alpha_blending(),
            .. Default::default()
        };

        target.draw(self.vbo.slice(char_idx..(char_idx+4)).unwrap(), &NO_INDICES, &self.program, &uniforms, &draw_parameters)?;
        Ok(())
    }

    pub fn print(self: &Self, target: &mut glium::Frame, s: &str, pos: (f64, f64)) -> Result<(), glium::DrawError> {
        let mut x_offset = 0.0;

        for c in s.chars() {
            let n = self.char_to_num.get(&(c as i64));
            let cd = self.fd.data.get(&(c as i64));

            if let Some(n) = n {
                let ci = *n as usize * 4;
                let off = [x_offset + pos.0 as f32, pos.1 as f32];
                self.print_single(target, ci, off)?;
                x_offset += cd.unwrap().x_advance as f32;
            }
        }

        Ok(())
    }
}