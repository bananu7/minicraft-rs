use glium::{glutin, Surface};
use glium::index::PrimitiveType;

fn create_program(display: &glium::Display) -> glium::Program {
    // compiling shaders and linking them together
    let program = program!(display,
        450 => {
            vertex: "
                #version 450
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
                #version 450
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
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            color: [f32; 3],
        }

        implement_vertex!(Vertex, position, color);

        glium::VertexBuffer::new(&display,
            &[
                Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
                Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
            ]
        ).unwrap()
    };

    // building the index buffer
    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
                                               &[0u16, 1, 2]).unwrap();

    let program = create_program(&display);

    // Here we draw the black background and triangle to the screen using the previously
    // initialised resources.
    //
    // In this case we use a closure for simplicity, however keep in mind that most serious
    // applications should probably use a function that takes the resources as an argument.
    let draw = || {
        // building the uniforms
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
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