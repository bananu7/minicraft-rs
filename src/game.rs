
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
pub struct Game {
    current_state: Box<dyn GameState>,

    mouse_state: MouseState,
}

fn construct_next_state(tag: GameStateTag, display: &glium::Display) -> Result<Box<dyn GameState>, ()> {
    match tag {
        GameStateTag::BuildShip => Ok(Box::new(BuildShipGameState::new(&display))),
        GameStateTag::Menu => Ok(Box::new(MenuState::new(&display)?)),
    }
}

impl Game {
    pub fn new(display: &glium::Display) -> Result<Self, ()> {
        Ok(Game {
            current_state: Box::new(MenuState::new(&display)?),
            mouse_state: MouseState::new(),
        })
    }

    pub fn draw (&mut self, display: &glium::Display) -> Result<(), glium::DrawError> {
        self.current_state.draw(&display)
    }

    pub fn react_to_mouse_move(&mut self, delta: (f64, f64)) {
        self.current_state.react_to_mouse_move(delta);
    }

    pub fn react_to_cursor_move(&mut self, pos: glutin::dpi::LogicalPosition<f64>) {
        self.mouse_state.x = pos.x;
        self.mouse_state.y = pos.y;
    }

    pub fn react_to_keyboard(&mut self, input: glutin::event::KeyboardInput) {
        self.current_state.react_to_keyboard(input)
    }

    pub fn react_to_mouse_click(&mut self, state: glutin::event::ElementState, button: glutin::event::MouseButton) {
        self.current_state.react_to_mouse_click(state, button);

        match button {
            glutin::event::MouseButton::Left => self.mouse_state.left = state == glutin::event::ElementState::Pressed,
            glutin::event::MouseButton::Right => self.mouse_state.right = state == glutin::event::ElementState::Pressed,
            glutin::event::MouseButton::Middle => self.mouse_state.middle = state == glutin::event::ElementState::Pressed,
            glutin::event::MouseButton::Other(_) => (),
        }
    }

    pub fn update(&mut self, display: &glium::Display) {
        let change_state = self.current_state.update(self.mouse_state.clone(), &display);

        // TODO : unwrap
        match change_state {
            Some (next_state) => {
                self.current_state = construct_next_state(next_state, &display).unwrap();
            }
            None => (),
        }   
    }
}
