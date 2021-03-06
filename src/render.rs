use glium::{glutin};

pub mod camera;
pub mod gui;
pub mod util;
pub mod world;

use crate::game::Game;

pub fn setup() {
    let events_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()   
        .with_inner_size(glutin::dpi::LogicalSize{ width: 800.0, height: 600.0 });
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    //    .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 4)));
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut game_state = Game::new(&display).unwrap();

    display.gl_window().window().set_title("minicraft-rs");

    events_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            //std::time::Duration::from_nanos(16_666_667);
            std::time::Duration::from_nanos(10_666_667);
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

                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    let scale = display.gl_window().window().scale_factor();
                    let logical_position = position.to_logical(scale);
                    game_state.react_to_cursor_move(logical_position);
                }
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
                match game_state.draw(&display) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Error during rendering!");
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                    }
                }
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
