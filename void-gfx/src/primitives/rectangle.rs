use crate::{geometry::{Color, Vec2}, renderable::Renderable, renderer::Renderer};

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

impl Renderable for Rectangle {
    fn render<R>(&self, renderer: &mut R) -> Result<(), R::Error>
    where
            R: Renderer {
        renderer.render_rectangle(&self)
    }
}
