pub mod rect;
pub mod pipeline;
pub mod shaders;
pub mod spritesheet;

use glm::*;

pub fn glm_mat4_to_raw_array(mat: Mat4) -> [[f32; 4]; 4] {
    let mut matrix: [[f32; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];

    for i in 0..4 {
        for j in 0..4 {
            matrix[i][j] = mat[i][j];
        }
    }

    return matrix;
}

pub type DrawResult =  Result<(), glium::DrawError>;