/*
The standard format for display chunk rendering has 4 attributes, interleaved
- vec3 pos
- vec3 normal
- vec2 tc
- vec3 color
*/

use glium::{implement_vertex, uniform};
use glium::{Surface};

use crate::render::util::pipeline::Pipeline;

#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texCoord: [f32; 2],
    pub color: [f32; 3],
}
implement_vertex!(Vertex, position, color, normal, texCoord);

pub struct AtlasTextures {
    pub color: glium::texture::SrgbTexture2d,
    pub normal: glium::texture::Texture2d,
    pub depth: glium::texture::Texture2d,
}

pub struct DisplayChunk {
    pub vbo: glium::VertexBuffer<Vertex>,
}

impl DisplayChunk {
    pub fn draw(self: &Self, target: &mut glium::Frame, pip: &Pipeline, textures: &AtlasTextures, time: f32) {
        let matrix = pip.get_vp_matrix();

        let uniforms = uniform! {
            matrix: matrix,
            normalMap: &textures.normal,
            colorMap: &textures.color,
            depthMap: &textures.depth,
            eye: [pip.camera.position.x, pip.camera.position.y, pip.camera.position.z],
            time: time,
        };

        const NO_INDICES: glium::index::NoIndices =
                glium::index::NoIndices(glium::index::PrimitiveType::Patches {vertices_per_patch: 3});

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            polygon_mode: glium::PolygonMode::Fill,
            .. Default::default()
        };

        target.draw(&self.vbo, NO_INDICES, &pip.get_program(), &uniforms, &params).unwrap();
    }
}