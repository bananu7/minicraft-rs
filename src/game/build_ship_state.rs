use super::traits::*;

//use std::time::{Instant};
use std::fs;
use core::cell::RefCell;
use glium::glutin;
use glium::Surface;
use glium::{program};
use crate::render::camera::*;
use crate::world::{Block, Field, raycast, setup};
use crate::world::orientation::Orientation;
use crate::render::util::pipeline::Pipeline;
use crate::render::world::DisplayField;

enum Strafe {
    Stop, Left, Right
}
enum Fly {
    Stop, Forward, Backward
}

pub struct BuildShipGameState {
    pipeline: RefCell<Pipeline>,
    cursor_grabbed: RefCell<bool>,
    display_field: DisplayField,
    field: RefCell<Field>,
    should_exit: bool,
    display_dirty: bool,

    strafe: Strafe,
    fly: Fly,
}

pub fn create_program(display : &glium::Display) -> glium::Program {
    // compiling shaders and linking them together
    // NVidia: 450
    // Intel: 430
    // OSX: 410
    let vertex_source = fs::read_to_string("data/shaders/voxel.vs").unwrap();
    let fragment_source = fs::read_to_string("data/shaders/voxel.fs").unwrap();
    let tes_source = fs::read_to_string("data/shaders/voxel.tes").unwrap();
    let tcs_source = fs::read_to_string("data/shaders/voxel.tcs").unwrap();

    let program = program!(display,
        410 => {
            vertex: vertex_source.as_str(),
            fragment: fragment_source.as_str(),
            tessellation_control: tcs_source.as_str(),
            tessellation_evaluation: tes_source.as_str(),
        },
    ).unwrap();
    return program
}

impl BuildShipGameState {
    pub fn new(display: &glium::backend::glutin::Display) -> Self {
        let program = create_program(&display);
        let pipeline = std::cell::RefCell::new(Pipeline::new(program));

        pipeline.borrow_mut().camera.position.y = 20.0;

        BuildShipGameState {
            pipeline: pipeline,
            cursor_grabbed: RefCell::new(false),
            display_field: DisplayField::new(display),
            field: std::cell::RefCell::new(setup()),
            should_exit: false,
            display_dirty: true,

            strafe: Strafe::Stop,
            fly: Fly::Stop,
        }
    }
}

impl GameState for BuildShipGameState {
    fn draw (&mut self, display: &glium::backend::glutin::Display) -> Result<(), glium::DrawError> {
        //let start_time = Instant::now();
        {
            let cg = self.cursor_grabbed.borrow();
            match display.gl_window().window().set_cursor_grab(*cg) {
                Err(e) => println!("Window grab({}) error: {}", *cg, e),
                _ => ()
            }

            display.gl_window().window().set_cursor_visible(! *cg);

            let mut target = display.draw();
            target.clear_color_and_depth((0.247, 0.843, 0.988, 1.0), 1.0);

            let pip = self.pipeline.borrow();
            self.display_field.draw(&mut target, &display, &pip);
            //font_display.print(&mut target, "Hello, world!")?;

            target.finish().unwrap();
        }
        //let delta = Instant::now() - start_time;
        //println!("Render time: {}", delta.as_micros());

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

    fn react_to_keyboard(&mut self, input: glutin::event::KeyboardInput) {
        let key = input.scancode;

        if input.state == glutin::event::ElementState::Released {
            if key == 13 || key == 17 || key == 1 || key == 31 { // W or S
                self.fly = Fly::Stop;
            }
            else if key == 0 || key == 30 || key == 2 || key == 32 { // A or D
                self.strafe = Strafe::Stop;
            }
        } else {
            // first OSX, 2nd Windows
            if key == 1 || key == 31 { // S
                self.fly = Fly::Backward;
            }
            else if key == 13 || key == 17 { // W
                self.fly = Fly::Forward;
            }
            else if key == 0 || key == 30 { // A
                self.strafe = Strafe::Left;
            }
            else if key == 2 || key == 32 { // D
                self.strafe = Strafe::Right;
            }
            else if key == 3 || key == 33 { // F
                let mut cg = self.cursor_grabbed.borrow_mut();
                *cg = !(*cg);
            }
            /*else {
                print!("{}\n", key);
            }*/
        }
    }

    fn react_to_mouse_click(&mut self, state: glutin::event::ElementState, _button: glutin::event::MouseButton) {
        if state != glutin::event::ElementState::Pressed {
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
                f.set(coord, Block { value: 1, orientation: Orientation::YPlus });
            }

            self.display_dirty = true;
        }
    }

    fn update(&mut self, _ms: MouseState, display: &glium::backend::glutin::Display) -> Option<GameStateTag> {
        if self.display_dirty {
            self.display_field.update(&self.field.borrow(), &display);
            self.display_dirty = false;
        }

        let mut pip = self.pipeline.borrow_mut();

        match self.fly {
            Fly::Forward => pip.camera.fly(0.2),
            Fly::Backward => pip.camera.fly(-0.2),
            _ => (),
        }

        match self.strafe {
            Strafe::Left => pip.camera.strafe(0.2),
            Strafe::Right => pip.camera.strafe(-0.2),
            _ => (),
        }

        if self.should_exit { 
            Some(GameStateTag::Menu)
        } else {
            None
        }
    }
}