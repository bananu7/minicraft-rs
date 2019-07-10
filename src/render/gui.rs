use crate::render::bmfont::*;
use crate::render::bmfont_render::*;

use crate::game::traits::MouseState;
use std::path::Path;
use crate::render::rect::*;

pub struct Gui {
    font: DisplayFont,
    ms: MouseState,
    drawjets: Vec<Box<Fn(&mut TargettedGuiDisplay) -> Result<(), glium::DrawError>>>,
}

impl Gui {
    pub fn new(display: &glium::Display) -> Self {
        let fd = FontDescriptor::load(Path::new("data/font.xml"));
        Gui {
            font: DisplayFont::new(fd.unwrap(), &display),
            ms: MouseState::new(),
            drawjets: Vec::new(),
        }
    }

    pub fn draw(&self, target: &mut glium::Frame) -> Result<(), glium::DrawError> {
        let mut tgd = TargettedGuiDisplay::new(target, &self.font);
        for d in &self.drawjets {
            match d(&mut tgd) {
                Ok(()) => (),
                err => return err
            };
        }
        Ok(())
    }

    pub fn begin(&mut self, ms: MouseState) {
        self.ms = ms;
        self.drawjets.clear();
    }

    pub fn button(&mut self, caption: &str, bounds: Rect) -> bool {
        let clicked =
           self.ms.left &&
           self.ms.x >= bounds.x &&
           self.ms.x <= bounds.x + bounds.w &&
           self.ms.y >= bounds.y &&
           self.ms.y <= bounds.y + bounds.h;

        self.label(caption, (bounds.x, bounds.y));

        return clicked
    }

    pub fn label(&mut self, text: &str, position: (f64, f64)) {
        let c = text.to_string();
        let d = move |gd: &mut TargettedGuiDisplay| {
            gd.print(c.clone(), position)
        };
        self.drawjets.push(Box::new(d));
    }
}


struct TargettedGuiDisplay<'a> {
    font: &'a DisplayFont,
    target: &'a mut glium::Frame,
}

impl<'a> TargettedGuiDisplay<'a> {
    pub fn new(target: &'a mut glium::Frame, font: &'a DisplayFont) -> Self {
        TargettedGuiDisplay {
            font: font,
            target: target,
        }
    }
    pub fn print(&mut self, text: String, pos: (f64,f64)) -> Result<(), glium::DrawError> {
        self.font.print(self.target, &text, pos)
    }
}
