use glium::glutin;

#[derive(Debug, Copy, Clone)]
pub struct MouseState {
    pub x: f64,
    pub y: f64,
    pub left: bool,
    pub right: bool,
    pub middle: bool,
}

impl MouseState {
    pub fn new() -> Self { 
        MouseState {
            x: 0.0,
            y: 0.0,
            left: false,
            right: false,
            middle: false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct KeyboardState {
}

impl KeyboardState {
    pub fn new() -> Self { KeyboardState {} }
}

pub trait GameState {
    fn react_to_mouse_move(&mut self, _position: (f64, f64)) {}
    fn react_to_mouse_click(&mut self, _state: glutin::event::ElementState, _button: glutin::event::MouseButton) {}
    fn react_to_keyboard(&mut self, _input: glutin::event::KeyboardInput) {}

    fn draw(&self, display: &glium::backend::glutin::Display) -> Result<(), glium::DrawError>;
    fn update(&mut self, ms: MouseState) -> Option<GameStateTag>;
}

#[derive(Debug, Clone, Copy)]
pub enum GameStateTag {
    Menu,
    BuildShip,
}