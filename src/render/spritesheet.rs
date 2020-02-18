use crate::render::util::DrawResult;

use crate::render::util::glm_mat4_to_raw_array;
use glm::vec2;
use glium::{Surface};
use glium::{uniform, program, implement_vertex};

use std::io::Cursor;
use std::collections::HashMap;

use crate::render::camera::*;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    source: [f32; 2],
}
implement_vertex!(Vertex, position, source);

pub struct SpriteDescriptor {
    pub id: i64,

    pub x: i64, pub y: i64,
    pub width: i64, pub height: i64,

    pub x_offset: i64, pub y_offset: i64,
}

pub struct SpriteSheet {
    pub data: HashMap<String, SpriteDescriptor>,
    //pub data: Vec<SpriteDescriptor>,
    pub x_size: i64,
    pub y_size: i64,
}

impl SpriteSheet {
    pub fn new() -> Self {
        SpriteSheet {
            data: HashMap::new(),
           // data: Vec::new(),
            x_size: 256,
            y_size: 256
        }
    }
}

pub struct DisplaySpriteSheet {
    sheet: SpriteSheet,

    vbo: glium::VertexBuffer<Vertex>,
    program: glium::Program,
    texture: glium::texture::CompressedSrgbTexture2d,
}

fn create_sprite_program(display : &glium::Display) -> glium::Program {
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
                }
            "
        },
    ).unwrap();
    return program
}

impl DisplaySpriteSheet {
    pub fn new(sheet: SpriteSheet, display: &glium::Display) -> Self {
        let mut verts = Vec::new();

        for (i,c) in &sheet.data {
            let w = c.width as f32;
            let h = c.height as f32;
            let xs = sheet.x_size as f32;
            let ys = sheet.y_size as f32;
            let x = c.x as f32;
            let y = c.y as f32;
            let xo = c.x_offset as f32;
            let yo = c.y_offset as f32;

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

        let image = image::load(Cursor::new(&include_bytes!("../../data/gui.png")[..]),
                            image::ImageFormat::Png).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let opengl_texture = glium::texture::CompressedSrgbTexture2d::new(display, image).unwrap();

        DisplaySpriteSheet {
            sheet: sheet,

            vbo: vertex_buffer,
            program: create_sprite_program(&display),
            texture: opengl_texture,
        }
    }

    pub fn draw_sprite(
        self: &Self, target: &mut glium::Frame,
        name: &str, offset: [f32; 2]
    ) -> DrawResult {
        const NO_INDICES: glium::index::NoIndices =
            glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

        let screen_width = 800.0;
        let screen_height  = 600.0;

        let camera = CameraOrtho2D::new(vec2(screen_width, screen_height), vec2(0.0, 0.0));

        let uniforms = uniform! {
            matrix: glm_mat4_to_raw_array(camera.calculate_view()),
            tex: &self.texture,
            offset: offset,
        };

        let draw_parameters = glium::DrawParameters {
            blend: glium::draw_parameters::Blend::alpha_blending(),
            .. Default::default()
        };

        match self.sheet.data.get(name) {
            Some(sprite) => {
                let sprite_index = sprite.id as usize;
                target.draw(
                    self.vbo.slice(sprite_index..(sprite_index+4)).unwrap(),
                    &NO_INDICES,
                    &self.program,
                    &uniforms,
                    &draw_parameters
                )
            },
            None => Err(glium::DrawError::WrongQueryOperation)
        }
    }
}