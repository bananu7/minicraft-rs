use super::traits::*;
use glium::Surface;
use gltf::Semantic;

use crate::render::gui::*;
use crate::render::util::rect::*;
use crate::render::object::*;

pub struct ViewObjectState {
    gui: Gui,
    object: Object,
}

impl ViewObjectState {
    pub fn new(display: &glium::backend::glutin::Display) -> Result<Self, ()> {
        let object = Object::new("data/objects/chest.gltf")?;

        Ok(ViewObjectState {
            gui: Gui::new(&display)?,
            object: object
        })
    }
}

impl GameState for ViewObjectState {
    fn draw (&mut self, display: &glium::backend::glutin::Display) -> Result<(), glium::DrawError> {
        {
            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.4, 0.1, 1.0), 1.0);

            self.gui.draw(&mut target)?;
            target.finish().unwrap();
        }
        Ok(())
    }

    fn update(&mut self, ms: MouseState, _display: &glium::backend::glutin::Display) -> Option<GameStateTag> {
        self.gui.begin(ms);
        
        //self.gui.label(&format!("Mouse: ({}, {})", ms.x, ms.y), (100.0, 200.0));
        let mut y = 10.0;
        for scene in self.object.document.scenes() {
            self.gui.label(&format!("Scene {}", scene.index()), (10.0, y));
            y += 20.0;
            for node in scene.nodes() {
                let s = format!("{}. {} ({} children)",
                    node.index(),
                    node.name().unwrap_or("<no name>"),
                    node.children().count()
                );

                self.gui.label(&s, (30.0, y));
                y += 20.0;

                let m = &node.mesh();
                match m {
                    None => (),
                    Some(mesh) => {
                        self.gui.label(&format!("Primitives: {}", mesh.primitives().count()), (50.0, y));
                        y += 20.0;

                        for primitive in mesh.primitives() {
                            self.gui.label(&format!("- Material: {}, Mode: {:?}, Indices?: {}",
                                    primitive.material().name().unwrap_or("<no name>"),
                                    primitive.mode(),
                                    primitive.indices().is_some(),
                                ),
                                (70.0, y)
                            );
                            y += 20.0;

                            for (attribute, attr_acc) in primitive.attributes() {
                                let attr_str = match &attribute {
                                    Semantic::Normals => "normal".to_string(),
                                    Semantic::Positions => "position".to_string(),
                                    Semantic::Colors(n) => format!("color ({})", &n),
                                    Semantic::TexCoords(unit) => format!("texcoord (unit {})", &unit),

                                    _ => "unknown".to_string(),
                                };

                                self.gui.label(
                                    &format!("- {} | count: {}", attr_str, attr_acc.count()),
                                    (110.0, y)
                                );
                                y += 20.0;
                            }
                        }
                    }
                }
            }
        }

        let change_state =
            if self.gui.button("Back to menu", Rect::new(50.0, 700.0, 40.0, 40.0)) {
                Some(GameStateTag::Menu)
            } else {
                None
            };


        return change_state;
    }
}