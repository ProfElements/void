use crate::geometry::{Color, Vec2};

pub struct Triangle {
    pub vertices: [Vec2; 3],
    pub color: Color,
}

impl Triangle {
    fn new(vertices: [Vec2; 3], color: Color) -> Self {
        Self { vertices, color }
    }
}
