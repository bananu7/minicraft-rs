use crate::render::bmfont::*;
use crate::render::bmfont_render::*;

use crate::game::traits::MouseState;
use std::path::Path;

pub struct Gui {
    font: DisplayFont,
    ms: MouseState,
    drawjets: Vec<Box<Fn(&mut TargettedGuiDisplay)>>,
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
            d(&mut tgd);
        }
        Ok(())
    }

    pub fn begin(&mut self, ms: MouseState) {
        self.ms = ms;
    }

    pub fn button(&mut self, caption: &str, bounds: (f64,f64,f64,f64)) -> bool {
        let c = caption.to_string();
        let b = bounds.clone();
        let d = move |gd: &mut TargettedGuiDisplay| {
            let cc = &c;
            let bb = &b;
            gd.print(cc.clone(), (bb.0, bb.1))
        };
        self.drawjets.push(Box::new(d));

        if !self.ms.left {
            return false
        }

        if self.ms.x >= bounds.0 &&
           self.ms.x <= bounds.0 + bounds.2 &&
           self.ms.y >= bounds.1 &&
           self.ms.y <= bounds.1 + bounds.3 {
            return true
        }
        return false
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
    pub fn print(&mut self, text: String, pos: (f64,f64)) {
        self.font.print(self.target, &text, pos);
    }
}
