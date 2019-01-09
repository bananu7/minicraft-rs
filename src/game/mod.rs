
pub mod traits;
pub mod menu_state;
pub mod build_ship_state;

pub use self::traits::*;
pub use self::build_ship_state::BuildShipGameState;
pub use self::menu_state::MenuState;

use glium::glutin;

/*
Game is an uber-state encompassing all the states and the
transitions between them.
*/
pub struct Game<'a> {
    current_state: Box<dyn GameState + 'a>,
    display: &'a glium::Display,

    mouse_state: MouseState,
    keyboard_state: KeyboardState,
}

fn construct_next_state<'a>(tag: GameStateTag, display: &'a glium::Display) -> Box<dyn GameState + 'a> {
    match tag {
        GameStateTag::BuildShip => Box::new(BuildShipGameState::new(display)),
        GameStateTag::Menu => Box::new(MenuState::new(display)),
    }
}

impl<'a> Game<'a> {
    pub fn new(display: &'a glium::Display) -> Self {
        Game {
            current_state: Box::new(MenuState::new(display)),
            display: display,

            mouse_state: MouseState::new(),
            keyboard_state: KeyboardState::new(),
        }
    }

    pub fn draw (&self) -> Result<(), glium::DrawError> {
        self.current_state.draw()
    }

    pub fn react_to_mouse_move(&mut self, delta: (f64, f64)) {
        self.current_state.react_to_mouse_move(delta);
    }

    pub fn react_to_cursor_move(&mut self, pos: glutin::dpi::LogicalPosition) {
        self.mouse_state.x = pos.x;
        self.mouse_state.y = pos.y;
    }

    pub fn react_to_keyboard(&mut self, input: glutin::KeyboardInput) {
        self.current_state.react_to_keyboard(input)
    }

    pub fn react_to_mouse_click(&mut self, state: glutin::ElementState, button: glutin::MouseButton) {
        self.current_state.react_to_mouse_click(state, button);
        println!("ms {} {}", self.mouse_state.x, self.mouse_state.y);

        match button {
            glutin::MouseButton::Left => self.mouse_state.left = state == glutin::ElementState::Pressed,
            glutin::MouseButton::Right => self.mouse_state.right = state == glutin::ElementState::Pressed,
            glutin::MouseButton::Middle => self.mouse_state.middle = state == glutin::ElementState::Pressed,
            glutin::MouseButton::Other(_) => (),
        }
    }

    pub fn update(&mut self) {
        let change_state = self.current_state.update(self.mouse_state.clone());

        match change_state {
            Some (next_state) => {
                self.current_state = construct_next_state(next_state, self.display);
            }
            None => (),
        }   
    }
}
