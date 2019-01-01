use glm::*;
use glm::ext::rotate;
use glm::ext::translate;
use num;

use super::traits::Camera;

pub struct CameraOrbit
{
    pub look_dir: Vec2,

    pub center: Vec3,
    pub distance: f32,
}

pub fn get_direction_vec(m: &Mat4) -> Vec3 {
    vec3(m.c0.z, m.c1.z, m.c2.z)
}

impl CameraOrbit {
    fn get_rotation_mat(&self) -> Mat4 {
        let mut m = num::one::<Mat4>();
        m = rotate(&m, self.look_dir.y, vec3(1.0, 0.0, 0.0));
        m = rotate(&m, self.look_dir.x, vec3(0.0, 1.0, 0.0));
        return m;
    }
}

impl Camera for CameraOrbit {
    fn calculate_view(&self) -> Mat4 {
        let mut m = num::one::<Mat4>();
        m = translate(&m, -self.center);
        m.mul_m(&self.get_rotation_mat());
        return m
    }
}
