use super::traits::*;
use glium::glutin;
use glium::Surface;

pub struct MenuState<'a> {
    display: &'a glium::backend::glutin::Display,
    change_state: Option<GameStateTag>,
}

impl<'a> MenuState<'a> {
    pub fn new(display: &'a glium::backend::glutin::Display) -> Self {
        MenuState {
            display: display,
            change_state: None,
        }
    }
}

impl<'a> GameState for MenuState<'a> {
    fn draw (&self) -> Result<(), glium::DrawError> {
        {
            let mut target = self.display.draw();
            target.clear_color_and_depth((0.0, 0.1, 0.4, 1.0), 1.0);

            //let pip = self.pipeline.borrow();
            //font_display.print(&mut target, "Hello, world!")?;

            target.finish().unwrap();
        }
        Ok(())
    }

    //let update_camera_look = |position: glutin::dpi::LogicalPosition| {
    fn react_to_mouse_move(&mut self, _position: (f64, f64)) {
    }

    fn react_to_keyboard(&mut self, _input: glutin::KeyboardInput) {
    }

    fn react_to_mouse_click(&mut self, state: glutin::ElementState, _button: glutin::MouseButton) {
        if state != glutin::ElementState::Pressed {
            return
        }
        self.change_state = Some(GameStateTag::BuildShip);
    }

    fn update(&mut self) -> Option<GameStateTag> {
        self.change_state.clone()
    }
}