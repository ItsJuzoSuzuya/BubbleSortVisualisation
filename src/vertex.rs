#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f64, f64),
    pub color: (f64, f64, f64),
}

implement_vertex!(Vertex, position, color);

impl Vertex {
    pub fn new(x: f64, y: f64, color_is_red: bool) -> Vertex {
        if color_is_red {
            Vertex {
                position: (x, y),
                color: (1.0, 0.0, 0.0),
            }
        } else {
            Vertex {
                position: (x, y),
                color: (0.0, 0.0, 0.0),
            }
        }
    }
}
