use crate::geometry::{Color, Vec2};

pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
    pub thickness: u16,
    pub color: Color,
}

impl Line {
    fn new(start: Vec2, end: Vec2, thickness: u16, color: Color) -> Self {
        Self {
            start,
            end,
            thickness,
            color,
        }
    }
}
