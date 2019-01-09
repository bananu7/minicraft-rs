use super::traits::*;

use core::cell::RefCell;
use glium::glutin;
use glium::Surface;
use glium::{program};
use crate::render::camera::*;
use crate::world::{Block, Field, raycast, Orientation, setup};
use crate::render::pipeline::Pipeline;
use crate::render::render_world::DisplayField;
use crate::render::shaders;

pub struct BuildShipGameState<'a> {
    pipeline: RefCell<Pipeline>,
    cursor_grabbed: RefCell<bool>,
    display_field: DisplayField,
    field: RefCell<Field>,
    display: &'a glium::Display,
    should_exit: bool,
}

pub fn create_program(display : &glium::Display) -> glium::Program {
    // compiling shaders and linking them together
    // NVidia: 450
    // Intel: 430
    // OSX: 410
    let program = program!(display,
        410 => {
            vertex: &(shaders::LIGHT_VERT_SHADER),
            fragment: "
                #version 410 core
                in vec3 vColor;
                out vec4 f_color;
                void main() {
                    f_color = vec4(vColor, 1.0);
                }
            "
        },
    ).unwrap();
    return program
}

impl<'a> BuildShipGameState<'a> {
    pub fn new(display: &'a glium::backend::glutin::Display) -> Self {
        let program = create_program(&display);
        let pipeline = std::cell::RefCell::new(Pipeline::new(program));

        BuildShipGameState {
            pipeline: pipeline,
            display: display,
            cursor_grabbed: RefCell::new(false),
            display_field: DisplayField {},
            field: std::cell::RefCell::new(setup()),
            should_exit: false,
        }
    }
}

impl<'a> GameState for BuildShipGameState<'a> {
    fn draw (&self) -> Result<(), glium::DrawError> {
        {
            let mut target = self.display.draw();
            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

            let pip = self.pipeline.borrow();
            self.display_field.draw(&mut target, &self.display, &self.field.borrow(), &pip);
            //font_display.print(&mut target, "Hello, world!")?;

            target.finish().unwrap();
        }
        Ok(())
    }

    //let update_camera_look = |position: glutin::dpi::LogicalPosition| {
    fn react_to_mouse_move(&mut self, position: (f64, f64)) {
        if *self.cursor_grabbed.borrow() {
            let mut pip = self.pipeline.borrow_mut();
            let (x,y) = position;
            pip.camera.look_dir.x -= x as f32 / 1000.0;
            pip.camera.look_dir.y -= y as f32 / 1000.0;
        }
    }

    fn react_to_keyboard(&mut self, input: glutin::KeyboardInput) {
        if input.state == glutin::ElementState::Released {
            return;
        }
        {
            let mut pip = self.pipeline.borrow_mut();
            let key = input.scancode;

            // first OSX, 2nd Windows
            if key == 13 || key == 17 { // W
                pip.camera.fly(0.2);
            }
            else if key == 1 || key == 31 { // S
                pip.camera.fly(-0.2);
            }
            else if key == 0 || key == 30 { // A
                pip.camera.strafe(0.2);
            }
            else if key == 2 || key == 32 { // D
                pip.camera.strafe(-0.2);
            }
            else if key == 3 || key == 33 { // F
                let mut cg = self.cursor_grabbed.borrow_mut();
                *cg = !(*cg);

                match self.display.gl_window().grab_cursor(*cg) {
                    Err(e) => println!("Window grab({}) error: {}", *cg, e),
                    _ => println!("Window grab succeeded")
                }

                self.display.gl_window().hide_cursor(*cg);
            }
            /*else {
                print!("{}\n", key);
            }*/
        }
    }

    fn react_to_mouse_click(&mut self, state: glutin::ElementState, _button: glutin::MouseButton) {
        if state != glutin::ElementState::Pressed {
            return
        }

        {
            let pip = self.pipeline.borrow();
            let pos = pip.camera.position;
            let dir = camera_fly::get_direction_vec(&pip.camera.calculate_view());

            let blocks = raycast::raycast(raycast::RaycastParams {
                pos: pos,
                dir: dir,
                len: 10f32,
                include_first: true,
            });

            let mut f = self.field.borrow_mut();
            for coord in blocks {
                f.set(coord, Block { value: 1, orientation: Orientation::Up });
            }
        }
    }

    fn update(&mut self, ms: MouseState) -> Option<GameStateTag> {
        if self.should_exit { 
            Some(GameStateTag::Menu)
        } else {
            None
        }
    }
}