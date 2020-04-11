use std::fs;

use glium::{Surface};
use glium::{uniform, implement_vertex};

use crate::render::world::display_chunk::{Vertex, DisplayChunk};
use crate::render::world::traits::{DisplayChunkGen};

use crate::world::{Chunk, SIZE};
use crate::world::coord::{OuterChunkCoord, InnerChunkCoord};
//use crate::world::orientation::Orientation;


#[derive(Copy, Clone)]
struct RootVertex {
    position: [f32; 3],
    value: i32,
}
implement_vertex!(RootVertex, position, value);

pub struct DisplayChunkGenHot {
    vertices_cache: Vec<RootVertex>,
    input_vbo: glium::VertexBuffer<RootVertex>,
    tfb_program: glium::Program,
}

impl DisplayChunkGenHot {
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
            fragment_shader: fragment_source.as_str(),
            transform_feedback_varyings: Some((tfb_varyings, glium::program::TransformFeedbackMode::Interleaved)),
            outputs_srgb: false,
            uses_point_size: false,
        };

        let program = glium::program::Program::new(display, input).unwrap();

        let mut vertices = Vec::with_capacity((SIZE*SIZE*SIZE) as usize);
        vertices.resize((SIZE*SIZE*SIZE) as usize,  RootVertex { position: [0.,0.,0.], value: 0 });

        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    let v = 0; // placeholder

                    vertices[(x*SIZE*SIZE + y*SIZE + z + 0) as usize] =
                        RootVertex { position: [x as f32, y as f32, z as f32], value: v as i32 };
                }
            }
        }

        let input_buffer: glium::VertexBuffer<RootVertex> = glium::VertexBuffer::new(display, &vertices).unwrap();

        DisplayChunkGenHot {
            vertices_cache: vertices,
            tfb_program: program,
            input_vbo: input_buffer,
        }
    }
}

impl DisplayChunkGen for DisplayChunkGenHot {
    fn generate(self: &mut Self, chunk_coord: OuterChunkCoord, chunk: &Chunk, display: &glium::Display) -> DisplayChunk {
        {
            for x in 0..SIZE {
                for y in 0..SIZE {
                    for z in 0..SIZE {
                        let v = chunk.get(&InnerChunkCoord::new(x,y,z)).value;

                        self.vertices_cache[(x*SIZE*SIZE + y*SIZE + z + 0) as usize] =
                            RootVertex { position: [x as f32, y as f32, z as f32], value: v as i32 };
                    }
                }
            }

            self.input_vbo.write(&self.vertices_cache);
        }

        let mut out_buffer: glium::VertexBuffer<Vertex> =
            glium::VertexBuffer::empty(display, (SIZE*SIZE*SIZE * 4 * 6) as usize).unwrap();

        // TFB
        {
            const NO_INDICES: glium::index::NoIndices =
                glium::index::NoIndices(glium::index::PrimitiveType::Points);

            let session = glium::vertex::TransformFeedbackSession::new(display, &self.tfb_program,
                                                               &mut out_buffer).unwrap();

            let uniforms = uniform! {
                chunk_position: [(chunk_coord.x * SIZE) as f32, (chunk_coord.y * SIZE) as f32, (chunk_coord.z * SIZE) as f32],
            };

            let params = glium::DrawParameters {
                transform_feedback: Some(&session),
                draw_primitives: false,
                .. Default::default()
            };

            if glium::framebuffer::EmptyFrameBuffer::is_supported(display) {
                let mut empty_fbo = glium::framebuffer::EmptyFrameBuffer::new(display, 1, 1, None, None, false).unwrap();
                empty_fbo.draw(&self.input_vbo, NO_INDICES, &self.tfb_program, &uniforms, &params).unwrap();
            }
            else {
                let rbo = glium::framebuffer::RenderBuffer::new(
                    display, glium::texture::UncompressedFloatFormat::U8, 1, 1
                ).unwrap();
                let mut framebuffer = glium::framebuffer::SimpleFrameBuffer::new(display, &rbo).unwrap();
                framebuffer.draw(&self.input_vbo, NO_INDICES, &self.tfb_program, &uniforms, &params).unwrap();
            }
        }

        DisplayChunk {   
            vbo: out_buffer,
        }
    }
}
