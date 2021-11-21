use crate::geometry::{Color, Vec2};

pub struct Polyline<'a> {
    pub vertices: &'a [Vec2],
    pub color: Color,
}

impl<'a> Polyline<'a> {
    fn new(vertices: &'a [Vec2], color: Color) -> Self {
        Self { vertices, color }
    }
}
