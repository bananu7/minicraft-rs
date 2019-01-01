
use glm::vec3;
use glm::vec2;
use crate::render::camera::*;

pub struct Pipeline {
    program: glium::Program,
    pub camera: CameraFly,
}

impl Pipeline {
    pub fn new(program: glium::Program) -> Self {
        Pipeline {
            camera: CameraFly {
                look_dir: vec2(0.0, 0.0),
                position: vec3(0.5, 0.5, -5.0),
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

        let fov: f32 = 3.141592 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();
        let ar = 800.0/600.0;

        let _cam_proj = glm::ext::perspective(fov, ar, znear, zfar);
        // note: remember that this is column-major, so the lines of code are actually columns
        let cam_proj2 = glm::mat4(
            f / ar,  0.0,              0.0              ,   0.0,
               0.0,    f,              0.0              ,   0.0,
               0.0,  0.0,  (zfar+znear)/(zfar-znear)    ,   1.0,
               0.0,  0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0,
        );

        let cam_vp = cam_proj2.mul_m(&cam_view);

        for i in 0..4 {
            for j in 0..4 {
                matrix[i][j] = cam_vp[i][j];
            }
        }

        return matrix
    }
}