use crate::{geometry::{Color, Vec2}, renderable::Renderable, renderer::Renderer};

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

impl Renderable for Text {
    fn render<R>(&self, renderer: &mut R) -> Result<(), R::Error>
    where
            R: Renderer {
        renderer.render_text(&self)
    }
}
