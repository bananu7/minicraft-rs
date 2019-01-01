use glium::{glutin};
use std::time::{Duration, Instant};
use std::thread;
use std::path::Path;

pub mod camera;
pub mod render_world;
pub mod pipeline;
pub mod shaders;
mod bmfont;
mod bmfont_render;

use self::bmfont::*;
use self::bmfont_render::*;

use crate::game::BuildShipGameState;
use crate::game::traits::GameState;

pub fn setup() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(glutin::dpi::LogicalSize{ width: 800.0, height: 600.0 });
    let context = glutin::ContextBuilder::new();
    //    .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 4)));
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let font_descriptor = FontDescriptor::load(Path::new("data/font.xml"));
    /*match font_descriptor {
        Ok(fd) => println!("Loading font succeeded, {} characters loaded", fd.count()),
        Err(e) => println!("Loading font failed: {}", e),
    }*/
    let _font_display = DisplayFont::new(font_descriptor.unwrap(), &display);

    // TODO: make this actual game state with a field saying whether
    // the cursor must be grabbed or not
    // the main loop
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    let mut game_state = BuildShipGameState::new(&display);

    loop {
        // events
        let mut should_break = false;
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the main loop when the window is closed.
                    glutin::WindowEvent::CloseRequested => should_break = true,
                    // Redraw the triangle when the window is resized.
                    //glutin::WindowEvent::Resized(..) => game_state.draw(),

                    //glutin::WindowEvent::CursorMoved { position, .. } => update_camera_look(position),
                    glutin::WindowEvent::KeyboardInput { input, .. } => game_state.react_to_keyboard(input),
                    glutin::WindowEvent::MouseInput { state, button, .. } => game_state.react_to_mouse_click(state, button),

                    _ => (),
                },
                glutin::Event::DeviceEvent { event, .. } => match event {
                    glutin::DeviceEvent::MouseMotion { delta } => {
                        game_state.react_to_mouse_move(delta)
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
        game_state.draw();

        // sleep
        thread::sleep(fixed_time_stamp - accumulator);
    }
}
