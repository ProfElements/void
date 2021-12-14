use alloc::vec;
use alloc::vec::Vec;

use crate::{
    geometry::{Color, Vec2, Vertex},
    renderable::Renderable,
    renderer::{DrawHint, Renderer},
};

pub struct Rectangle {
    pub top_left: Vec2,
    pub size: Vec2,
    pub color: Color,
}

impl Rectangle {
    pub fn new(top_left: Vec2, size: Vec2, color: Color) -> Self {
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
        R: Renderer,
    {
        let vertexes = self.build_vertices();
        let draw_hint = self.get_draw_hint();
        renderer.render_vertex_list(&vertexes, draw_hint)
    }
    fn build_vertices(&self) -> Vec<Vertex> {
        vec![
            Vertex::new_colored(self.top_left, self.color),
            Vertex::new_colored(
                Vec2::new(self.top_left.x, self.top_left.y + self.size.y),
                self.color,
            ),
            Vertex::new_colored(
                Vec2::new(self.top_left.x + self.size.x, self.top_left.y + self.size.y),
                self.color,
            ),
            Vertex::new_colored(
                Vec2::new(self.top_left.x + self.size.x, self.top_left.y),
                self.color,
            ),
        ]
    }
    fn get_draw_hint(&self) -> DrawHint {
        DrawHint::Quads
    }
}
