use crate::{geometry::{Color, Vec2}, renderable::Renderable, renderer::Renderer};

pub struct Triangle {
    pub vertices: [Vec2; 3],
    pub color: Color,
}

impl Triangle {
    fn new(vertices: [Vec2; 3], color: Color) -> Self {
        Self { vertices, color }
    }
}

impl Renderable for Triangle {
    fn render<R>(&self, renderer: &mut R) -> Result<(), R::Error> where R: Renderer {
        renderer.render_triangle(&self)
    }
}
