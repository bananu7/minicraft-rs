use crate::render::util::pipeline::Pipeline;
use crate::world::coord::{OuterChunkCoord, InnerChunkCoord};
use crate::world::{Field, Chunk, SIZE, combine_coord};
use glium::{Surface};
use glium::{uniform, implement_vertex};
use glium::{program};
use std::fs;

#[derive(Copy, Clone)]
struct RootVertex {
    position: [f32; 3],
    value: i32,
}
implement_vertex!(RootVertex, position, value);

#[derive(Copy, Clone)]
#[allow(non_snake_case)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texCoord: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, normal, texCoord, color);

struct DisplayChunkHot {
    input_vbo: glium::VertexBuffer<RootVertex>,
    output_vbo: glium::VertexBuffer<Vertex>,
}

impl DisplayChunkHot {
    // Generate the display rendition for a chunk
    pub fn new(chunk_coord: OuterChunkCoord, chunk: &Chunk, display: &glium::Display) -> Self {
        let mut vertices = Vec::with_capacity((SIZE*SIZE*SIZE) as usize);
        vertices.resize((SIZE*SIZE*SIZE) as usize,  RootVertex { position: [0.,0.,0.], value: 0 });

        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    let v = chunk.get(&InnerChunkCoord::new(x,y,z)).value;

                    vertices[(x*SIZE*SIZE + y*SIZE + z + 0) as usize] = RootVertex { position: [x as f32, y as f32, z as f32], value: v as i32 };
                }
            }
        }

        let input_buffer: glium::VertexBuffer<RootVertex> = glium::VertexBuffer::new(display, &vertices).unwrap();
        let mut out_buffer: glium::VertexBuffer<Vertex> = glium::VertexBuffer::empty(display, (SIZE*SIZE*SIZE * 4 * 6) as usize).unwrap();

        DisplayChunkHot {
            input_vbo: input_buffer,          
            output_vbo: out_buffer,
        }
    }

    pub fn draw(self: &mut Self, display: &glium::Display, target: &mut glium::Frame, pip: &Pipeline, tfb_program: &glium::Program) {
        // TFB
        {
            const NO_INDICES: glium::index::NoIndices =
                glium::index::NoIndices(glium::index::PrimitiveType::Points);

            let session = glium::vertex::TransformFeedbackSession::new(display, &tfb_program,
                                                               &mut self.output_vbo).unwrap();

            let uniforms = uniform! {
            };

            let params = glium::DrawParameters {
                transform_feedback: Some(&session),
                .. Default::default()
            };

            target.draw(&self.input_vbo, NO_INDICES, tfb_program, &uniforms, &params).unwrap();
        }
        // Actual
        {
            let matrix = pip.get_vp_matrix();
            let uniforms = uniform! {
                matrix: matrix,
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

            target.draw(&self.output_vbo, NO_INDICES, &pip.get_program(), &uniforms, &params).unwrap();
        }
        
    }
}

pub struct DisplayField {
    display_chunks: Vec<DisplayChunkHot>,
    tfb_program: glium::Program,
}

impl DisplayField {
    pub fn new(display: &glium::Display) -> Self {
        let vertex_source = fs::read_to_string("data/shaders/voxel_hot.vs").unwrap();
        let fragment_source = fs::read_to_string("data/shaders/voxel_hot.fs").unwrap();
        let gs_source = fs::read_to_string("data/shaders/voxel_hot.gs").unwrap();

        let tfb_varyings = vec![
            "out_position".to_string(),
            "out_normal".to_string(),
            "out_texCoord".to_string(),
            "out_color".to_string(),
        ];

        let input = glium::program::ProgramCreationInput::SourceCode {
            vertex_shader: vertex_source.as_str(),
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            geometry_shader: Some(gs_source.as_str()),
            //geometry_shader: None,
            fragment_shader: fragment_source.as_str(),
            transform_feedback_varyings: Some((tfb_varyings, glium::program::TransformFeedbackMode::Interleaved)),
            outputs_srgb: false,
            uses_point_size: false,
        };

        let program = glium::program::Program::new(display, input).unwrap();

        DisplayField {
            tfb_program: program,
            display_chunks: Vec::new()
        }
    }

    pub fn update(self: &mut Self, field: &Field, display: &glium::Display) {
        let chunks = field.get_chunks();
        self.display_chunks.clear();

        for (coord, chunk) in chunks.get_map() {
            self.display_chunks.push(DisplayChunkHot::new((*coord).clone(), chunk, display));
        }
    }

    pub fn draw(self: &mut Self, target: &mut glium::Frame, display: &glium::Display, pip: &Pipeline) {
        for dc in &mut self.display_chunks {        
            dc.draw(display, target, pip, &self.tfb_program);
        }
    }
}