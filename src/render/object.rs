use glium::{implement_vertex, uniform};
use glium::{Surface};

use crate::render::util::pipeline::Pipeline;

pub struct Object {
    pub document: gltf::Document,
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
}

impl Object {
    pub fn new(path: &str) -> Result<Object, ()> {
        let (document, buffers, images) = gltf::import(path).map_err(|_|())?;
        Ok(Object {
            document: document,
            buffers: buffers,
            images: images,
        })
    }
}

// -- 

#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct Vertex {
    pub position: [f32; 3],
}
implement_vertex!(Vertex, position);

pub struct DisplayObject {
    pub vbo: glium::VertexBuffer<Vertex>,
    pub ibo: glium::IndexBuffer<u32>,
}

impl DisplayObject {
    pub fn new(o: &Object, display: &glium::Display) -> DisplayObject {
        let scene = &o.document.scenes().next().unwrap();         // TODO only gets first scene
        let node = &scene.nodes().next().unwrap();                // TODO only gets first node
        let mesh = &node.mesh().unwrap();                // TODO requires a mesh present
        let primitive = &mesh.primitives().next().unwrap();       // TODO only gets first primitive

        let reader = &primitive.reader(|buf| Some(&o.buffers[buf.index()]) );

        let mut vertices = Vec::new();
        for position in reader.read_positions().unwrap() {
            vertices.push(Vertex { position: position });
        }

        let mut indices = Vec::<u32>::new();
        match reader.read_indices().unwrap() {
            gltf::mesh::util::ReadIndices::U8(rd) => for index in rd { indices.push(index.into())},
            gltf::mesh::util::ReadIndices::U16(rd) => for index in rd { indices.push(index.into())},
            gltf::mesh::util::ReadIndices::U32(rd) => for index in rd { indices.push(index)},
        };

        let vbo = glium::VertexBuffer::new(display, &vertices).unwrap();
        let ibo = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

        DisplayObject { vbo, ibo }
    }

    pub fn draw(&mut self, pip: &Pipeline, target: &mut glium::Frame) {
        let matrix = pip.get_vp_matrix();

        let uniforms = uniform! {
            matrix: matrix,
            eye: [pip.camera.position.x, pip.camera.position.y, pip.camera.position.z],
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            polygon_mode: glium::PolygonMode::Fill,
            .. Default::default()
        };

        target.draw(&self.vbo, &self.ibo, &pip.get_program(), &uniforms, &params).unwrap();
    }
}