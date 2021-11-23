use crate::{geometry::{Color, Vec2}, renderable::Renderable, renderer::Renderer};

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

impl Renderable for Line {
    fn render<R>(&self, renderer: &mut R) -> Result<(), R::Error>
    where
            R: Renderer {
        renderer.render_line(&self)
    }
}
