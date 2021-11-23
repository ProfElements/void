use crate::{geometry::{Color, Vec2}, renderable::Renderable, renderer::Renderer};

pub struct Polyline<'a> {
    pub vertices: &'a [Vec2],
    pub color: Color,
}

impl<'a> Polyline<'a> {
    fn new(vertices: &'a [Vec2], color: Color) -> Self {
        Self { vertices, color }
    }
}

impl<'a> Renderable for Polyline<'a> {
    fn render<R>(&self, renderer: &mut R) -> Result<(), R::Error>
    where
            R: Renderer {
        renderer.render_polyline(&self)
    }
}
