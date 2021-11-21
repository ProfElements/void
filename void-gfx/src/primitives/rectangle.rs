use crate::geometry::{Color, Vec2};

pub struct Rectangle {
    pub top_left: Vec2,
    pub size: Vec2,
    pub color: Color,
}

impl Rectangle {
    fn new(top_left: Vec2, size: Vec2, color: Color) -> Self {
        Self {
            top_left,
            size,
            color,
        }
    }
}
