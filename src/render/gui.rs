use crate::render::spritesheet::SpriteDescriptor;
use crate::render::spritesheet::SpriteSheet;
use crate::render::spritesheet::DisplaySpriteSheet;
use crate::render::bmfont::*;
use crate::render::bmfont_render::*;

use crate::game::traits::MouseState;
use std::path::Path;
use crate::render::rect::*;

pub struct Gui {
    font: DisplayFont,
    image_data: DisplaySpriteSheet,
    ms: MouseState,
    drawjets: Vec<Box<dyn Fn(&Gui, &mut TargettedGuiDisplay) -> Result<(), glium::DrawError>>>,
}

impl Gui {
    pub fn new(display: &glium::Display) -> Self {
        let fd = FontDescriptor::load(Path::new("data/font.xml"));

        let mut image_spritesheet = SpriteSheet::new();
        image_spritesheet.data.insert(String::from("bg"), SpriteDescriptor {
            id: 0,
            x: 0, y: 0,
            width: 100, height: 50,
            x_offset: -10, y_offset: -10
        });
        image_spritesheet.x_size = 256;
        image_spritesheet.y_size = 256;

        Gui {
            font: DisplayFont::new(fd.unwrap(), &display),
            image_data: DisplaySpriteSheet::new(image_spritesheet, &display),
            ms: MouseState::new(),
            drawjets: Vec::new(),
        }
    }

    pub fn draw(&self, target: &mut glium::Frame) -> Result<(), glium::DrawError> {
        let mut tgd = TargettedGuiDisplay::new(target, &self.font);
        for d in &self.drawjets {
            match d(&self, &mut tgd) {
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

        let d = move |sself: &Gui, gd: &mut TargettedGuiDisplay| {
            let b = bounds.clone();
            sself.image_data.draw_sprite(gd.target, "bg", [b.x as f32, b.y as f32])
        };
        self.drawjets.push(Box::new(d));

        self.label(caption, (bounds.x, bounds.y));

        return clicked
    }

    pub fn label(&mut self, text: &str, position: (f64, f64)) {
        let c = text.to_string();
        let d = move |_self: &Gui, gd: &mut TargettedGuiDisplay| {
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
