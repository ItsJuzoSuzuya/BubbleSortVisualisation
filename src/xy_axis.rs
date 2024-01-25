use crate::vertex::Vertex;

pub const VERTICES: [Vertex; 8] = [
    Vertex {
        position: (0.0, 0.0),
    },
    Vertex {
        position: (-0.9, -0.9),
    },
    Vertex {
        position: (-0.9, -0.88),
    },
    Vertex {
        position: (0.9, -0.88),
    },
    Vertex {
        position: (0.9, -0.9),
    },
    Vertex {
        position: (0.9, 0.9),
    },
    Vertex {
        position: (0.88, 0.9),
    },
    Vertex {
        position: (0.88, -0.88),
    },
];

pub const INDICES: [u16; 12] = [1, 2, 3, 1, 3, 4, 3, 5, 6, 6, 3, 7];
