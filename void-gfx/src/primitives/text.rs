use crate::geometry::{Color, Vec2};

pub struct Text {
    pub top_left: Vec2,
    pub text: &'static str,
    pub px_size: f32,
    pub color: Color,
}

impl Text {
    fn new(top_left: Vec2, text: &'static str, px_size: f32, color: Color) -> Self {
        Self {
            top_left,
            text,
            px_size,
            color,
        }
    }
}
