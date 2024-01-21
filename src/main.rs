use crate::bubble_sort::bubble_sort;
use rand::Rng;

#[macro_use]
extern crate glium;
use glium::Surface;

fn main() {
    let mut list = vec![];

    create_window();

    for _ in 0..10 {
        list.push(rand::thread_rng().gen::<f64>() * 100.0);
    }

    bubble_sort(&mut list);
    println!("{:?}", list);
}

fn create_window() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Sorting Algorithms!")
        .build(&event_loop);

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);

    let shape = vec![
        Vertex {
            position: [-0.5, -0.5],
        },
        Vertex {
            position: [0.0, 0.5],
        },
        Vertex {
            position: [0.5, -0.25],
        },
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 330

        in vec2 position;
        uniform float x_off;

        void main() {
            vec2 pos = position;
            pos.x += x_off;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

    let mut t: f32 = 0.0;

    event_loop
        .run(move |ev, control_flow| match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    control_flow.exit();
                }

                winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                }

                winit::event::WindowEvent::RedrawRequested => {
                    t += 0.02;
                    let x_off = t.sin();

                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);
                    let uniforms = uniform! { x_off: x_off };
                    target
                        .draw(
                            &vertex_buffer,
                            &indices,
                            &program,
                            &uniforms,
                            &Default::default(),
                        )
                        .unwrap();
                    target.finish().unwrap();
                }
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        })
        .unwrap();
}

mod bubble_sort;
