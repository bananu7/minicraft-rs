use glm::{vec2, vec3};
use crate::world::coord::{OuterChunkCoord};
use glium::{Surface};
use glium::index::PrimitiveType;
use glium::{uniform, implement_vertex};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

pub struct DisplayChunk<'a> {
    vbo: glium::VertexBuffer<Vertex>,
    ibo: glium::IndexBuffer<u16>,
    display: &'a glium::Display,
}

impl<'a> DisplayChunk<'a> {
    pub fn new(coord: OuterChunkCoord, display: &'a glium::Display) -> Self {
        let vertex_buffer = {
            glium::VertexBuffer::new(display,
                &[
                    Vertex { position: [0.0, 0.0, 0.0], color: [0.0, 1.0, 0.0] },
                    Vertex { position: [0.0, 1.0, 0.0], color: [0.0, 0.0, 1.0] },
                    Vertex { position: [1.0, 1.0, 0.0], color: [1.0, 0.0, 0.0] },
                    Vertex { position: [1.0, 0.0, 0.0], color: [1.0, 0.0, 0.0] },

                    Vertex { position: [0.0, 0.0, 1.0], color: [0.0, 1.0, 0.0] },
                    Vertex { position: [0.0, 1.0, 1.0], color: [0.0, 0.0, 1.0] },
                    Vertex { position: [1.0, 1.0, 1.0], color: [1.0, 0.0, 0.0] },
                    Vertex { position: [1.0, 0.0, 1.0], color: [1.0, 0.0, 0.0] },
                ]
            ).unwrap()
        };

        let index_buffer = glium::IndexBuffer::new(
            display,
            PrimitiveType::TrianglesList,
            &[0u16, 1, 2, 0, 2, 3,
              4, 5, 6, 4, 6, 7
            ]
        ).unwrap();

        DisplayChunk {
            vbo: vertex_buffer,
            ibo: index_buffer,
            display: display,
        }
    }

    pub fn draw(self: &Self, program: &glium::Program) {
        // building the uniforms
        let cam = crate::render::camera_fly::CameraFly {
            look_dir: vec2(0.0, 0.0),
            position: vec3(0.0, 0.0, 0.0),
        };

        let mut matrix: [[f32; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];
        let camm = cam.calculate_view();
        for i in 0..4 {
            for j in 0..4 {
                matrix[i][j] = camm[i][j];
            }
        }

        let uniforms = uniform! {

            matrix: matrix,
        };

        // drawing a frame
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&self.vbo, &self.ibo, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    }
}

