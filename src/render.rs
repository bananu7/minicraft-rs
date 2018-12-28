use glium::{glutin, Surface};
use glium::{program, uniform, implement_vertex};
use crate::world::Chunk;

mod camera_fly;
mod render_world;
mod pipeline;
mod shaders;

use self::pipeline::Pipeline;
use self::render_world::DisplayChunk;
use crate::world::coord::OuterChunkCoord;

fn create_program(display : &glium::Display) -> glium::Program {
    // compiling shaders and linking them together
    // NVidia: 450
    // Intel: 430
    // OSX: 410
    let program = program!(display,
        410 => {
            vertex: &(shaders::light_vert_shader),
            fragment: "
                #version 410
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

pub fn setup() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(glutin::dpi::LogicalSize{ width: 800.0, height: 600.0 });
    let context = glutin::ContextBuilder::new();
    //    .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 4)));
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let program = create_program(&display);
    let pipeline = std::cell::RefCell::new(Pipeline::new(program));

    let mut chunk = Chunk::new();
    chunk.fill();
    let display_chunk = DisplayChunk::new(OuterChunkCoord::new(0,0,0), &chunk, &display);
    let draw = || {
        let pip = pipeline.borrow();
        display_chunk.draw(&pip);
    };

    let update_camera_look = |position: glutin::dpi::LogicalPosition| {
        {
            let mut pip = pipeline.borrow_mut();
            pip.camera.look_dir.x = ((position.x as f32 / 800.0) - 0.5) * -2.0;
            pip.camera.look_dir.y = ((position.y as f32 / 600.0) - 0.5) * -3.0;
            //print!("pos: {},{}\n", position.x, position.y);
        }

        draw();
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
            else if key == 0 || key == 30 {
                pip.camera.strafe(0.2);
            }
            else if key == 2 || key == 32 {
                pip.camera.strafe(-0.2);
            }
            /*else {
                print!("{}\n", key);
            }*/
        }
        draw();
    };

    // Draw the triangle to the screen.
    draw();

    // the main loop
    events_loop.run_forever(|event| {
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::WindowEvent::CloseRequested => return glutin::ControlFlow::Break,
                // Redraw the triangle when the window is resized.
                glutin::WindowEvent::Resized(..) => draw(),

                glutin::WindowEvent::CursorMoved { position, .. } => update_camera_look(position),
                glutin::WindowEvent::KeyboardInput { input, .. } => update_camera_pos(input),

                _ => (),
            },
            _ => (),
        }
        glutin::ControlFlow::Continue
    });
}
