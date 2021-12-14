use crate::{
    geometry::{Color, Vec2, Vertex},
    renderable::Renderable,
    renderer::{DrawHint, Renderer},
};
use alloc::vec::Vec;

pub struct Triangle {
    pub vertices: [Vec2; 3],
    pub color: Color,
}

impl Triangle {
    pub fn new(vertices: [Vec2; 3], color: Color) -> Self {
        Self { vertices, color }
    }
}

impl Renderable for Triangle {
    fn render<R>(&self, renderer: &mut R) -> Result<(), R::Error>
    where
        R: Renderer,
    {
        let vertexes = self.build_vertices();
        let draw_hint = self.get_draw_hint();
        renderer.render_vertex_list(&vertexes, draw_hint)
    }

    fn build_vertices(&self) -> Vec<Vertex> {
        let mut vertexes = Vec::with_capacity(self.vertices.len());

        for vertex in self.vertices {
            vertexes.push(Vertex::new_colored(vertex, self.color));
        }

        vertexes
    }

    fn get_draw_hint(&self) -> DrawHint {
        DrawHint::Triangles
    }
}
