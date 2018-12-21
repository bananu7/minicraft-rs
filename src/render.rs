use glium::{glutin, Surface};
use glium::index::PrimitiveType;
use glium::{program, uniform, implement_vertex};

mod camera_fly;
mod render_world;
use self::render_world::DisplayChunk;
use crate::world::coord::OuterChunkCoord;

fn create_program(display : &glium::Display) -> glium::Program {
    // compiling shaders and linking them together
    let program = program!(display,
        430 => {
            vertex: "
                #version 430
                uniform mat4 matrix;
                in vec2 position;
                in vec3 color;
                out vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 430
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
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    //    .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 4)));
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let program = create_program(&display);

    let display_chunk = DisplayChunk::new(OuterChunkCoord::new(0,0,0));
    let draw = || {
        display_chunk.draw();
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
                _ => (),
            },
            _ => (),
        }
        glutin::ControlFlow::Continue
    });
}
