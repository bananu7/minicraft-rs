use glium::{glutin};
use std::time::{Duration, Instant};
use std::path::Path;

pub mod camera;
pub mod render_world;
pub mod pipeline;
pub mod gui;
pub mod shaders;
pub mod util;
pub mod rect;
pub mod spritesheet;
mod bmfont;
mod bmfont_render;

use self::bmfont::*;
use self::bmfont_render::*;

use crate::game::Game;

pub fn setup() {
    let mut events_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize{ width: 800.0, height: 600.0 });
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
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
    let mut _accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    let mut game_state = Game::new(&display);

    events_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    display.gl_window().window().request_redraw();
                    return;
                }

                glutin::event::WindowEvent::CursorMoved { position, .. } =>game_state.react_to_cursor_move(position),
                glutin::event::WindowEvent::KeyboardInput { input, .. } => game_state.react_to_keyboard(input),
                glutin::event::WindowEvent::MouseInput { state, button, .. } => game_state.react_to_mouse_click(state, button),

                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => {
                    display.gl_window().window().request_redraw();
                },
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            glutin::event::Event::RedrawRequested(_) => {
                game_state.draw(&display);
                return;
            }
            glutin::event::Event::DeviceEvent { event, .. } => match event {
                glutin::event::DeviceEvent::MouseMotion { delta } => {
                    game_state.react_to_mouse_move(delta);
                    return;
                },
                _ => return,
            }
            _ => return,
        }
        game_state.update(&display);
        display.gl_window().window().request_redraw();
    });
}
