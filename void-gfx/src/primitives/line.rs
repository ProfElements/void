use crate::{
    geometry::{Color, Vec2, Vertex},
    renderable::Renderable,
    renderer::{DrawHint, Renderer},
};
use alloc::vec;
use alloc::vec::Vec;

pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
    pub color: Color,
}

impl Line {
    pub fn new(start: Vec2, end: Vec2, color: Color) -> Self {
        Self { start, end, color }
    }
}

impl Renderable for Line {
    fn render<R>(&self, renderer: &mut R) -> Result<(), R::Error>
    where
        R: Renderer,
    {
        let vertexes = self.build_vertices();
        let draw_hint = self.get_draw_hint();
        renderer.render_vertex_list(&vertexes, draw_hint)
    }
    fn build_vertices(&self) -> Vec<Vertex> {
        vec![
            Vertex::new_colored(self.start, self.color),
            Vertex::new_colored(self.end, self.color),
        ]
    }

    fn get_draw_hint(&self) -> DrawHint {
        DrawHint::Lines
    }
}
