
use glium::{implement_vertex, uniform};
use glium::Surface;
use crate::render::util::pipeline::Pipeline;

pub struct Object {
    pub document: gltf::Document,
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
}

#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct Vertex {
    pub position: [f32; 3],

implement_vertex!(Vertex, position);

pub struct RenderObject {
    pub vbo: glium::VertexBuffer<Vertex>,
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



impl RenderObject {
    pub fn new(object: &Object, display: &glium::Display) -> RenderObject {
        // TEMP
        
        let mut vertices = Vec::new();

        let scene = object.document.scenes().next();
        //for scene in object.document.scenes() {
        {
            let node = scene.nodes().next();
            //for node in scene.nodes() {
            {
                let m = &node.mesh();
                match m {
                    None => (),
                    Some(mesh) => {
                        for primitive in mesh.primitives() {
                            for (attribute, attr_acc) in primitive.attributes() {
                                vertices.extend(attr_acc.view());
                            }
                        }
                    }
                }
            }
        }

        let vbo = {
            glium::VertexBuffer::new(display, &vertices).unwrap()
        };

        RenderObject {
            vbo: vbo
        }
    }

    pub fn draw(
        &self,
        target: &mut glium::Frame,
        pip: &Pipeline
    ) {
        let matrix = pip.get_vp_matrix();

        let uniforms = uniform! {
            matrix: matrix,
            //normalMap: &textures.normal,
            //colorMap: &textures.color,
            //depthMap: &textures.depth,
            eye: [pip.camera.position.x, pip.camera.position.y, pip.camera.position.z],
            //time: time,
        };

        target.clear_color_and_depth((0.247, 0.843, 0.988, 1.0), 1.0);

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

        target.draw(&self.vbo, NO_INDICES, &pip.get_program(), &uniforms, &params).unwrap()
    }
}
