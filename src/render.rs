use glium::{glutin, Surface};
use glium::index::PrimitiveType;
use glium::{program, uniform, implement_vertex};

mod camera_fly;
mod render_world;
mod pipeline;

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
            vertex: "
                #version 410
                uniform mat4 matrix;
                in vec3 position;
                in vec3 color;
                out vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 1.0) * matrix;
                    vColor = color;
                }
            ",

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
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    //    .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 4)));
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let program = create_program(&display);
    let pipeline = std::cell::RefCell::new(Pipeline::new(program));

    let display_chunk = DisplayChunk::new(OuterChunkCoord::new(0,0,0), &display);
    let draw = || {
        display_chunk.draw(&*pipeline.borrow());
    };

    let update_camera = |position: glutin::dpi::LogicalPosition| {
        let mut pip = pipeline.borrow_mut();
        pip.camera.look_dir.x = position.x as f32 / 1280.0;
        pip.camera.look_dir.y = position.y as f32 / 800.0;
        print!("pos: {},{}\n", position.x, position.y);
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

                glutin::WindowEvent::CursorMoved { position, .. } => update_camera(position),

                _ => (),
            },
            _ => (),
        }
        glutin::ControlFlow::Continue
    });
}
