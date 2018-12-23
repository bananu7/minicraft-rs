
use glm::vec3;
use glm::vec2;
use crate::render::camera_fly::CameraFly;

pub struct Pipeline {
    program: glium::Program,
    pub camera: CameraFly,
}

impl Pipeline {
    pub fn new(program: glium::Program) -> Self {
        Pipeline {
            camera: crate::render::camera_fly::CameraFly {
                look_dir: vec2(0.0, -0.1),
                position: vec3(0.0, 0.5, 0.0),
            },
            program: program,
        }
    }

    pub fn get_program(&self) -> &glium::Program {
        return &self.program;
    }

    pub fn get_vp_matrix(&self) -> [[f32; 4]; 4] {
        let mut matrix: [[f32; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];
        let cam_view = self.camera.calculate_view();
        let cam_proj = glm::ext::perspective(3.1416 * 0.5, 1.0, -10.0, 10.0);
        let cam_vp = cam_proj.mul_m(&cam_view);

        for i in 0..4 {
            for j in 0..4 {
                matrix[i][j] = cam_vp[i][j];
            }
        }

        return matrix
    }
}