use super::traits::*;
use glium::Surface;
use crate::render::gui::*;
use crate::render::rect::*;
use std::time::{Duration, Instant};

pub struct MenuState {
    gui: Gui,
    time: Instant,
    last_delta: Duration,
}

impl MenuState {
    pub fn new(display: &glium::backend::glutin::Display) -> Self {
        MenuState {
            gui: Gui::new(&display),
            time: Instant::now(),
            last_delta: Duration::new(0, 0)
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

    fn update(&mut self, ms: MouseState, _display: &glium::backend::glutin::Display) -> Option<GameStateTag> {
        let current_time = Instant::now();
        self.last_delta = current_time - self.time;
        self.time = current_time;

        self.gui.begin(ms);

        self.gui.label(&format!("Delta: {}", self.last_delta.as_micros()), (100.0, 300.0));

        let mut change_state = None;

        if self.gui.button("Build ship!", Rect::new(100.0, 100.0, 40.0, 40.0))  {
            change_state = Some(GameStateTag::BuildShip);
        }

        self.gui.label(&format!("Mouse: ({}, {})", ms.x, ms.y), (100.0, 200.0));

        return change_state;
    }
}