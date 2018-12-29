use glm::*;
use glm::ext::rotate;
use glm::ext::translate;
use num;

pub struct CameraFly
{
    // x,y of look_dir correspond to mouse movements,
    // so .x is rotation around y-axis (yaw), and .y is around x-axis (pitch)
    pub look_dir: Vec2, 
    pub position: Vec3,
}

pub fn get_direction_vec(m: &Mat4) -> Vec3 {
    // third column, {0,1,2} row of a view matrix is "direction"
    vec3(-m.c2.x, -m.c2.y, m.c2.z)
}

impl CameraFly {
    fn get_rotation_mat(&self) -> Mat4 {
        let mut m = num::one::<Mat4>();
        m = rotate(&m, self.look_dir.y, vec3(1.0, 0.0, 0.0));
        m = rotate(&m, self.look_dir.x, vec3(0.0, 1.0, 0.0));
        return m;
    }

    pub fn calculate_view(&self) -> Mat4 {
        let mut m = self.get_rotation_mat();
        m = translate(&m, -self.position);
        return m
    }

    pub fn fly(&mut self, dist: f32) {
        let m = self.get_rotation_mat();

        let delta_norm = get_direction_vec(&m);
        let delta = delta_norm * dist;
        self.position = self.position + delta
    }

    pub fn strafe(&mut self, left: f32) {
        self.position.x -= self.look_dir.x.cos() * left;
        self.position.z -= self.look_dir.x.sin() * left;
    }
}
