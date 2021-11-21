use super::{Color, Vec2};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    pub pos: Vec2,
    pub color: Color,
    pub texture_id: Option<usize>,
    pub uv: Option<Vec2>,
}

impl Vertex {
    pub fn new(pos: Vec2, color: Color, texture_id: Option<usize>, uv: Option<Vec2>) -> Self {
        Self {
            pos,
            color,
            texture_id,
            uv,
        }
    }

    pub fn new_colored(pos: Vec2, color: Color) -> Self {
        Self::new(pos, color, None, None)
    }

    pub fn new_textured(pos: Vec2, texture_id: usize, uv: Vec2) -> Self {
        Self::new(
            pos,
            Color::new_rgb(1.0, 1.0, 1.0),
            Some(texture_id),
            Some(uv),
        )
    }
}
