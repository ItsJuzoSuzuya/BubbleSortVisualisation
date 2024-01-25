use bubble_sort::{bubble_sort, draw};
use rand::Rng;
use winit::event::{Event, WindowEvent};

#[macro_use]
extern crate glium;

fn main() {
    create_window();
}

fn create_window() {
    let mut list = vec![];
    let mut sorted = false;

    for _ in 0..25 {
        list.push(rand::thread_rng().gen::<f64>() * 100.0);
    }

    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Sorting Algorithms!")
        .build(&event_loop);

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
                    draw(&mut list, &display);
                    bubble_sort(&mut list, &display);
                    sorted = true;
                }
                draw(&mut list, &display);
            }

            _ => (),
        })
        .unwrap();
}

mod bubble_sort;
mod vertex;
mod xy_axis;
