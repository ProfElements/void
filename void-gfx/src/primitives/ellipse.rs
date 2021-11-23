use crate::{geometry::{Color, Vec2}, renderable::Renderable, renderer::Renderer};

pub struct Ellipse {
    pub top_left: Vec2,
    pub size: Vec2,
    pub color: Color,
}

impl Ellipse {
    fn new(top_left: Vec2, size: Vec2, color: Color) -> Self {
        Self {
            top_left,
            size,
            color,
        }
    }

    fn new_equal(top_left: Vec2, radius: f32, color: Color) -> Self {
        Self::new(top_left, Vec2::new(radius * 2.0, radius * 2.0), color)
    }
}

impl Renderable for Ellipse {
    fn render<R>(&self, renderer: &mut R) -> Result<(), R::Error>
    where
            R: Renderer {
        renderer.render_ellipse(self)
    }
}
