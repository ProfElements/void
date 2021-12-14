use crate::{
    geometry::{Color, Vec2, Vertex},
    renderable::Renderable,
    renderer::{DrawHint, Renderer},
};
use alloc::vec::Vec;

pub struct Ellipse {
    pub top_left: Vec2,
    pub size: Vec2,
    pub color: Color,
}

impl Ellipse {
    pub fn new(top_left: Vec2, size: Vec2, color: Color) -> Self {
        Self {
            top_left,
            size,
            color,
        }
    }

    pub fn new_equal(top_left: Vec2, radius: f32, color: Color) -> Self {
        Self::new(top_left, Vec2::new(radius * 2.0, radius * 2.0), color)
    }
}

impl Renderable for Ellipse {
    fn render<R>(&self, renderer: &mut R) -> Result<(), R::Error>
    where
        R: Renderer,
    {
        let vertexes = self.build_vertices();
        let draw_hint = self.get_draw_hint();
        renderer.render_vertex_list(&vertexes, draw_hint)
    }
    fn build_vertices(&self) -> Vec<Vertex> {
        use core::f32::consts::PI;
        use libm::{cosf, sinf};

        let mut vertexes = Vec::new();

        let center = Vec2::new(
            self.top_left.x + (self.size.x * 0.5),
            self.top_left.y + (self.size.y * 0.5),
        );

        vertexes.push(Vertex::new_colored(center, self.color));

        const STEPS: u16 = 32;
        const VERT_COUNT: u16 = STEPS + 2;
        let x_radius = self.size.x * 0.5;
        let y_radius = self.size.y * 0.5;

        for n in 0..VERT_COUNT {
            let curr_step = n as f32 * 2.0 * PI / STEPS as f32;
            let x = center.x + (x_radius * cosf(curr_step));
            let y = center.y + (y_radius * sinf(curr_step));

            vertexes.push(Vertex::new_colored(Vec2::new(x, y), self.color));
        }

        vertexes
    }

    fn get_draw_hint(&self) -> DrawHint {
        DrawHint::TriFan
    }
}
