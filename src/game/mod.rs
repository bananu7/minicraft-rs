
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
        }
    }

    pub fn draw (&self) -> Result<(), glium::DrawError> {
        self.current_state.draw()
    }

    //let update_camera_look = |position: glutin::dpi::LogicalPosition| {
    pub fn react_to_mouse_move(&mut self, position: (f64, f64)) {
        self.current_state.react_to_mouse_move(position)
    }

    pub fn react_to_keyboard(&mut self, input: glutin::KeyboardInput) {
        self.current_state.react_to_keyboard(input)
    }

    pub fn react_to_mouse_click(&mut self, state: glutin::ElementState, button: glutin::MouseButton) {
        self.current_state.react_to_mouse_click(state, button)
    }

    pub fn update(&mut self) {
        let change_state = self.current_state.update();
        match change_state {
            Some (next_state) => {
                self.current_state = construct_next_state(next_state, self.display);
            }
            None => (),
        }   
    }
}
