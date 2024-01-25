use std::{thread::sleep, time::Duration};

use crate::glium::Surface;
use crate::vertex::Vertex;
use crate::xy_axis;
use glium::{glutin::surface::WindowSurface, Display};

pub fn bubble_sort(list: &mut Vec<f64>, display: &Display<WindowSurface>) {
    let mut sorted = false;
    let mut list_len = list.len();

    while !sorted {
        sleep(Duration::new(0, 20000));
        let mut swapped = false;
        for i in 1..list_len {
            if list[i - 1] > list[i] {
                list.swap(i - 1, i);
                draw(list, display);
                swapped = true;
            }
        }
        if !swapped {
            sorted = true;
        }
        list_len -= 1;
    }
}

pub fn draw(list: &mut Vec<f64>, display: &Display<WindowSurface>) {
    let graph_positions = glium::VertexBuffer::new(display, &xy_axis::VERTICES).unwrap();
    let indices = glium::IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TrianglesList,
        &xy_axis::INDICES,
    )
    .unwrap();

    let size = list.len();

    let mut rect_positions: Vec<glium::VertexBuffer<Vertex>> = Vec::new();
    let rect_indices_unbuffered: Vec<u16> = vec![1, 2, 3, 1, 3, 4];
    let rect_indices = glium::IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TrianglesList,
        &rect_indices_unbuffered,
    )
    .unwrap();

    for (index, value) in list.iter().enumerate() {
        let left_vertex_x = -0.85 + 1.7 * (index as f32 / size as f32);
        let right_vertex_x = left_vertex_x + 0.05;
        let hight = 0.9 * (*value as f32 / 100.0);

        let rect_vertices = vec![
            //Dummy
            Vertex::new(0.0, 0.0),
            //Bottem Left
            Vertex::new(left_vertex_x, -0.88),
            //Bottom Right
            Vertex::new(right_vertex_x, -0.88),
            //Top Right
            Vertex::new(right_vertex_x, hight),
            //Top Left
            Vertex::new(left_vertex_x, hight),
        ];

        let rect_position = glium::VertexBuffer::new(display, &rect_vertices).unwrap();
        rect_positions.push(rect_position);
    }

    let vertex_shader_src = r#"
        #version 330

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330

        out vec4 color;

        void main() {
            color = vec4(0.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let mut target = display.draw();
    target.clear_color(1.0, 1.0, 1.0, 1.0);
    target
        .draw(
            &graph_positions,
            &indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();

    for position in rect_positions {
        target
            .draw(
                &position,
                &rect_indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
    }

    target.finish().unwrap();
}
