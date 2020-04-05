use super::traits::*;
use glium::Surface;
use crate::render::gui::*;
use crate::render::rect::*;

pub struct MenuState {
    gui: Gui,
}

impl MenuState {
    pub fn new(display: &glium::backend::glutin::Display) -> Self {
        MenuState {
            gui: Gui::new(&display),
        }
    }
}

impl GameState for MenuState {
    fn draw (&self, display: &glium::backend::glutin::Display) -> Result<(), glium::DrawError> {
        {
            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.1, 0.4, 1.0), 1.0);

            self.gui.draw(&mut target)?;
            target.finish().unwrap();
        }
        Ok(())
    }

    fn update(&mut self, ms: MouseState) -> Option<GameStateTag> {
        self.gui.begin(ms);

        let mut change_state = None;

        if self.gui.button("Build ship!", Rect::new(100.0, 100.0, 40.0, 40.0))  {
            change_state = Some(GameStateTag::BuildShip);
        }

        self.gui.label(&format!("Mouse: ({}, {})", ms.x, ms.y), (100.0, 200.0));

        return change_state;
    }
}