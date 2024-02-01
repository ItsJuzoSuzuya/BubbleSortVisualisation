use crate::glium::Surface;
use crate::vertex::Vertex;
use glium::{glutin::surface::WindowSurface, Display};

pub fn draw(
    list: &Vec<f64>,
    display: &Display<WindowSurface>,
    program: &glium::Program,
    buffers: (&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u16>),
    swap_index: usize,
) {
    let mut rect_vertices = vec![Vertex::new(0.0, 0.0, false)];
    let mut rect_indices = Vec::new();

    let size = list.len();

    let mut target = display.draw();
    target.clear_color(1.0, 1.0, 1.0, 1.0);
    target
        .draw(
            buffers.0,
            buffers.1,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();

    for (index, value) in list.iter().enumerate() {
        let left_vertex_x = -0.85 + 1.7 * (index as f64 / size as f64);
        let right_vertex_x = left_vertex_x + (0.75 / size.clone() as f64);

        let height = 0.9 * (value / 100.0);
        let mut is_red = false;

        if swap_index == index {
            is_red = true;
        }

        rect_vertices.push(Vertex::new(left_vertex_x, -0.88, is_red));
        rect_vertices.push(Vertex::new(right_vertex_x, -0.88, is_red));
        rect_vertices.push(Vertex::new(right_vertex_x, height, is_red));
        rect_vertices.push(Vertex::new(left_vertex_x, height, is_red));

        let multiplier: u16 = 4 * index as u16;

        for i in 1..4 {
            rect_indices.push(multiplier + i);
        }
        rect_indices.push(multiplier + 1);
        rect_indices.push(multiplier + 3);
        rect_indices.push(multiplier + 4);
    }

    let rect_pos = glium::VertexBuffer::new(display, &rect_vertices).unwrap();
    let rect_indices = glium::IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TrianglesList,
        &rect_indices,
    )
    .unwrap();

    target
        .draw(
            &rect_pos,
            &rect_indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
    target.finish().unwrap();
}
