use glium::glutin;

pub trait GameState {
    fn react_to_mouse_move(&mut self, position: (f64, f64));
    fn react_to_mouse_click(&mut self, state: glutin::ElementState, _button: glutin::MouseButton);
    fn react_to_keyboard(&mut self, input: glutin::KeyboardInput);

    fn draw(&self) -> Result<(), glium::DrawError>;
    fn update(&mut self) -> Option<GameStateTag>;
}

#[derive(Debug, Clone, Copy)]
pub enum GameStateTag {
    Menu,
    BuildShip,
}