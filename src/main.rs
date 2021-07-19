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
    position: [f32; 3],
}

impl Vertex {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self{ position: [x, y, z] }
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

        let vertices = [
            Vertex::new(-1.0, -1.0, -1.0),
            Vertex::new(1.0, -1.0, -1.0),
            Vertex::new(1.0, 1.0, -1.0),
            Vertex::new(-1.0, 1.0, -1.0),
            Vertex::new(-1.0, -1.0, 1.0),
            Vertex::new(1.0, -1.0, 1.0),
            Vertex::new(1.0, 1.0, 1.0),
            Vertex::new(-1.0, 1.0, 1.0),
        ];
        let normals_arr = [
            Vertex::new(0.0, 0.0, 1.0),
            Vertex::new(1.0, 0.0, 0.0),
            Vertex::new(0.0, 0.0, -1.0),
            Vertex::new(-1.0, 0.0, 0.0),
            Vertex::new(0.0, 1.0, 0.0),
            Vertex::new(0.0, -1.0, 0.0)
        ];
        let indices_arr = [
            0, 1, 3,
            3, 1, 2,
            1, 5, 2,
            2, 5, 6,
            5, 4, 6,
            6, 4, 7,
            4, 0, 7,
            7, 0, 3,
            3, 2, 7,
            7, 2, 6,
            4, 5, 0,
            0, 5, 1u16
        ];
        let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
        let normals = glium::VertexBuffer::new(&display, &normals_arr).unwrap();
        let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices_arr).unwrap();
        let vertex_shader_src = r#"
            #version 140

            in vec3 position;
            out vec3 pos;

            uniform mat4 matrix;
            uniform mat4 scale_mat;

            void main() {
                pos = position;
                gl_Position = scale_mat * (matrix * vec4(position, 1.0));
            }
        "#;
        let fragment_shader_src = r#"
            #version 140

            in vec3 pos;
            out vec4 color;

            void main() {
                color = vec4(pos, 1.0);
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
            ],
            scale_mat: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 0.0, 0.01f32],
            ]
        };

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0); // set to blank;
        // frame.draw((&vertex_buffer, &normals), &indices, &program, &uniforms, &Default::default()).unwrap();
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
