use core::f64;
use std::{
    sync::mpsc,
    thread::spawn,
    time::{Duration, SystemTime},
};

use bogo_sort::bogo_sort;
use bubble_sort::bubble_sort;
use merge_sort::merge_sort;
use rand::Rng;
use winit::event::{Event, WindowEvent};

use crate::draw::draw;

#[macro_use]
extern crate glium;
extern crate lazy_static;

fn main() {
    create_window();
}

fn create_window() {
    let mut list: Vec<f64> = vec![];
    for _ in 0..7 {
        list.push(rand::thread_rng().gen::<f64>() * 100.0);
    }
    let list_clone = list.clone();

    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Sorting Algorithms!")
        .build(&event_loop);

    let (sender, receiver) = mpsc::channel();

    let vertex_shader = r#"
        #version 330

        in vec2 position;
        in vec3 color;
        out vec3 vertex_color;

        void main() {
            vertex_color = color;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader = r#"
        #version 330

        in vec3 vertex_color;
        out vec4 color;

        void main() {
            color = vec4(vertex_color, 1.0);
        }
    "#;
    let graph_positions = glium::VertexBuffer::new(&display, &xy_axis::VERTICES).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &xy_axis::INDICES,
    )
    .unwrap();

    let program =
        glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

    spawn(move || bogo_sort(&mut list_clone.clone(), sender));

    let mut sorted = false;
    let is_sorted = false;
    let mut swap_index = 0;

    let mut frame_counter = 0;
    let mut now = SystemTime::now();
    let mut time_elapsed = Duration::new(0, 0);

    event_loop
        .run(move |ev, control_flow| match ev {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow.exit(),

            Event::WindowEvent {
                event: WindowEvent::Resized(window_size),
                ..
            } => display.resize(window_size.into()),

            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                if !sorted {
                    for _ in 0..1 {
                        if !is_sorted {
                            frame_counter += 1;
                            (list, swap_index, sorted) = receiver.recv().unwrap();
                        }
                    }
                }
                draw(
                    &list,
                    &display,
                    &program,
                    (&graph_positions, &indices),
                    swap_index,
                );
                if frame_counter % 100 == 1 && frame_counter != 1 {
                    time_elapsed = now.elapsed().unwrap();
                    let fps = frame_counter as f64 / time_elapsed.as_secs_f64();
                    println!("{} {}", fps.round(), "fps");
                    now = SystemTime::now();
                    frame_counter = 0;
                }
            }

            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        })
        .unwrap();
}

mod bogo_sort;
mod bubble_sort;
mod draw;
mod merge_sort;
mod vertex;
mod xy_axis;
