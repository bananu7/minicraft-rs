use crate::render::bmfont::*;
use crate::render::bmfont_render::*;

use crate::game::traits::MouseState;
use std::path::Path;

pub struct Gui {
    font: DisplayFont,
}

impl Gui {
    pub fn new(display: &glium::Display) -> Self {
        let fd = FontDescriptor::load(Path::new("data/font.xml"));
        Gui {
            font: DisplayFont::new(fd.unwrap(), &display),
        }
    }

    pub fn draw(&self) -> Result<(), glium::DrawError> {
        Ok(())
    }

    pub fn begin(&self, ms: MouseState) {

    }

    pub fn button(&self, caption: &str) -> bool {
        return false
    }
}