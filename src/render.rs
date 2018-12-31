use glium::{glutin};
use glium::{program};
use glium::{Surface};
use std::time::{Duration, Instant};
use std::thread;
use std::path::Path;
use std::cell::RefCell;

mod camera_fly;
mod render_world;
mod pipeline;
mod shaders;
mod bmfont;
mod bmfont_render;

use self::pipeline::Pipeline;
use self::render_world::DisplayField;
use self::bmfont::*;
use self::bmfont_render::*;

use crate::world::{Block, Field, raycast, Orientation};

fn create_program(display : &glium::Display) -> glium::Program {
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

pub fn setup(field: std::cell::RefCell<Field>) {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(glutin::dpi::LogicalSize{ width: 800.0, height: 600.0 });
    let context = glutin::ContextBuilder::new();
    //    .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 4)));
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let program = create_program(&display);
    let pipeline = std::cell::RefCell::new(Pipeline::new(program));

    let font_descriptor = FontDescriptor::load(Path::new("data/font.xml"));
    /*match font_descriptor {
        Ok(fd) => println!("Loading font succeeded, {} characters loaded", fd.count()),
        Err(e) => println!("Loading font failed: {}", e),
    }*/
    let font_display = DisplayFont::new(font_descriptor.unwrap(), &display);

    // TODO: make this actual game state with a field saying whether
    // the cursor must be grabbed or not
    let cursor_grabbed = RefCell::new(false);

    let display_field = DisplayField { };
    let draw = || {
        {
            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

            let pip = pipeline.borrow();
            display_field.draw(&mut target, &display, &field.borrow(), &pip);

            font_display.print(&mut target, "Hello, world!");

            target.finish().unwrap();
        }
    };

    //let update_camera_look = |position: glutin::dpi::LogicalPosition| {
    let update_camera_look = |position: (f64, f64)| {
        {
            let mut pip = pipeline.borrow_mut();
            let (x,y) = position;
            pip.camera.look_dir.x -= x as f32 / 1000.0;
            pip.camera.look_dir.y -= y as f32 / 1000.0;
            //print!("pos: {},{}\n", position.x, position.y);
        }
    };

    let update_camera_pos = |input: glutin::KeyboardInput| {
        if input.state == glutin::ElementState::Released {
            return;
        }
        {
            let mut pip = pipeline.borrow_mut();
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
                let mut cg = cursor_grabbed.borrow_mut();
                *cg = !(*cg);

                match display.gl_window().grab_cursor(*cg) {
                    Err(e) => println!("Window grab({}) error: {}", *cg, e),
                    _ => println!("Window grab succeeded")
                }

                display.gl_window().hide_cursor(*cg);
            }
            /*else {
                print!("{}\n", key);
            }*/
        }
    };

    let click = |state: glutin::ElementState, _button: glutin::MouseButton| {
        if state != glutin::ElementState::Pressed {
            return
        }

        {
            let pip = pipeline.borrow();
            let pos = pip.camera.position;
            let dir = camera_fly::get_direction_vec(&pip.camera.calculate_view());

            let blocks = raycast::raycast(raycast::RaycastParams {
                pos: pos,
                dir: dir,
                len: 10f32,
                include_first: true,
            });

            let mut f = field.borrow_mut();
            for coord in blocks {
                f.set(coord, Block { value: 1, orientation: Orientation::Up });
            }
        }
    };

    // the main loop
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    loop {
        // events
        let mut should_break = false;
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the main loop when the window is closed.
                    glutin::WindowEvent::CloseRequested => should_break = true,
                    // Redraw the triangle when the window is resized.
                    glutin::WindowEvent::Resized(..) => draw(),

                    //glutin::WindowEvent::CursorMoved { position, .. } => update_camera_look(position),
                    glutin::WindowEvent::KeyboardInput { input, .. } => update_camera_pos(input),
                    glutin::WindowEvent::MouseInput { state, button, .. } => click(state, button),

                    _ => (),
                },
                glutin::Event::DeviceEvent { event, .. } => match event {
                    glutin::DeviceEvent::MouseMotion { delta } => {
                        if *cursor_grabbed.borrow() { // this only works in FPP mode
                            update_camera_look(delta)
                        }
                    },
                    _ => (),
                }
                _ => (),
            }
        });

        if should_break {
            break;
        }

        // time
        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        // update
        let fixed_time_stamp = Duration::new(0, 16666667);
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;

            // if you have a game, update the state here
        }

        // draw
        draw();

        // sleep
        thread::sleep(fixed_time_stamp - accumulator);
    }
}
