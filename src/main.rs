#[macro_use]
extern crate glium;
use glium::{
    glutin,
    Display,
    Surface,
    Program,
};
use std::time::{Instant, Duration};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32;2],
}

impl Vertex {
    fn new(x: f32, y: f32) -> Self {
        Self{ position: [x, y] }
    }
}

implement_vertex!(Vertex, position);

fn main(){
    let mut event_loop = glutin::event_loop::EventLoop::new(); // create event loop
    let wb = glutin::window::WindowBuilder::new() // create window builder
        .with_title("Shapes") // title
        .with_position(glutin::dpi::PhysicalPosition::new(0, 0)) // in top left corner
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0)); // size
    let cb = glutin::ContextBuilder::new(); // create context builder
    let display = Display::new(wb, cb, &event_loop).unwrap(); // create display

    let mut t: f32 = 0.0;
    const TWO_PI: f32 = 3.14159265 * 2.0;
    event_loop.run(move |ev, _, control_flow|{ // run event loop

        let tri_shape = vec![
            Vertex::new(0.0, 0.5),
            Vertex::new(-0.5, -0.5),
            Vertex::new(0.5, -0.5),
        ];
        let vertex_buffer = glium::VertexBuffer::new(&display, &tri_shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let vertex_shader_src = r#"
            #version 140

            in vec2 position;
            out vec2 pos;

            uniform mat4 matrix;

            void main() {
                pos = position;
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140

            in vec2 pos;
            out vec4 color;

            void main() {
                color = vec4(pos, 0.5, 1.0);
            }
        "#;
        let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        t += 0.01;
        if t > TWO_PI {
            t = 0.0;
        }

        let uniforms = uniform!{
            matrix: [
                [t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0); // set to blank;
        frame.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        frame.finish().unwrap();

        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                },
                _ => return,
            },
            _ => (),
        }
    });
}