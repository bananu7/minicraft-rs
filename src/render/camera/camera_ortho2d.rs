use glm::*;
use num;

use super::traits::Camera;

pub struct CameraOrtho2D {
    // Those are specified in screen-space units
    pub offset: Vec2,
    pub size: Vec2,

    // This will be used by the caller
    pub internal_size: Vec2
}

impl CameraOrtho2D {
    pub fn new(size: Vec2, offset: Vec2) -> Self {
        CameraOrtho2D {
            size: size,
            offset: offset,
            internal_size: size,
        }
    }
}
impl Camera for CameraOrtho2D {
    fn calculate_view(&self) -> Mat4 {
        let r = self.size.x;
        let b = self.size.y;

        glm::mat4(
            2.0/r,    0.0,  0.0,  0.0,
            0.0, 2.0/-b,  0.0,  0.0,
            0.0,    0.0, -1.0,  0.0,
            -1.0,    1.0,  0.0,  1.0
        )
    }
}
