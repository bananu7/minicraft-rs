use glium::{glutin};
use glium::{program};
use glium::{Surface};
use crate::world::Chunk;

mod camera_fly;
mod render_world;
mod pipeline;
mod shaders;

use self::pipeline::Pipeline;
use self::render_world::DisplayField;
use crate::world::coord::OuterChunkCoord;
use crate::world::raycast;
use crate::world::Field;

fn create_program(display : &glium::Display) -> glium::Program {
    // compiling shaders and linking them together
    // NVidia: 450
    // Intel: 430
    // OSX: 410
    let program = program!(display,
        410 => {
            vertex: &(shaders::LIGHT_VERT_SHADER),
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

pub fn setup(field: &Field) {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(glutin::dpi::LogicalSize{ width: 800.0, height: 600.0 });
    let context = glutin::ContextBuilder::new();
    //    .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 4)));
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let program = create_program(&display);
    let pipeline = std::cell::RefCell::new(Pipeline::new(program));

    //let display_chunk = DisplayChunk::new(OuterChunkCoord::new(0,0,0), &chunk, &display);
    let display_field = DisplayField { };
    let draw = || {
        {
            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

            let pip = pipeline.borrow();
            display_field.draw(&mut target, &display, &field, &pip);

            target.finish().unwrap();
        }
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

    let click = |state: glutin::ElementState, button: glutin::MouseButton| {
        {
            let pip = pipeline.borrow();
            let pos = pip.camera.position;
            let dir = camera_fly::get_direction_vec(&pip.camera.calculate_view());

            /*
            let blocks = raycast(RaycastParams {
                pos: pos,
                dir: dir,
                len: 10f32,
                include_first: true,
            });
            */
        }
        draw();
    };

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
                glutin::WindowEvent::MouseInput { state, button, .. } => click(state, button),

                _ => (),
            },
            _ => (),
        }
        glutin::ControlFlow::Continue
    });
}
