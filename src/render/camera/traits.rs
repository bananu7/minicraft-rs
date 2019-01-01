use glm::*;

pub trait Camera {
    //fn new(name: &'static str) -> Self;
    fn calculate_view(&self) -> Mat4;
}