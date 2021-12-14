use core::f32::consts::PI;

use alloc::vec::Vec;

use crate::{
    geometry::{Vec2, Vertex},
    primitives::{Ellipse, Line, Polyline, Rectangle, Triangle},
};

#[cfg(feature = "text")]
use crate::{geometry::Color, primitives::Text};

pub enum DrawHint {
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriStrip,
    TriFan,
    Quads,
}

//TODO: Try to minimize allocations during drawing.
pub trait Renderer {
    type Image;
    type Error;

    fn render_vertex_list(
        &mut self,
        vertices: &[Vertex],
        draw_hint: DrawHint,
    ) -> Result<(), Self::Error>;

    fn render_line(&mut self, line: &Line) -> Result<(), Self::Error> {
        self.render_vertex_list(
            &[
                Vertex::new_colored(line.start, line.color),
                Vertex::new_colored(line.end, line.color),
            ],
            DrawHint::Lines,
        )
    }

    fn render_polyline(&mut self, poly: &Polyline) -> Result<(), Self::Error> {
        let mut vertexes = Vec::with_capacity(poly.vertices.len());

        for vertex in poly.vertices {
            vertexes.push(Vertex::new_colored(*vertex, poly.color));
        }

        self.render_vertex_list(&vertexes, DrawHint::LineStrip)
    }

    fn render_triangle(&mut self, tri: &Triangle) -> Result<(), Self::Error> {
        let mut vertexes = Vec::with_capacity(tri.vertices.len());

        for vertex in tri.vertices {
            vertexes.push(Vertex::new_colored(vertex, tri.color));
        }

        self.render_vertex_list(&vertexes, DrawHint::Triangles)
    }

    fn render_rectangle(&mut self, rect: &Rectangle) -> Result<(), Self::Error> {
        self.render_vertex_list(
            &[
                Vertex::new_colored(rect.top_left, rect.color),
                Vertex::new_colored(
                    Vec2::new(rect.top_left.x, rect.top_left.y + rect.size.y),
                    rect.color,
                ),
                Vertex::new_colored(
                    Vec2::new(rect.top_left.x + rect.size.x, rect.top_left.y + rect.size.y),
                    rect.color,
                ),
                Vertex::new_colored(
                    Vec2::new(rect.top_left.x + rect.size.x, rect.top_left.y),
                    rect.color,
                ),
            ],
            DrawHint::Quads,
        )
    }

    fn render_ellipse(&mut self, ellipse: &Ellipse) -> Result<(), Self::Error> {
        use libm::{cosf, sinf};

        let mut vertexes = Vec::new();

        let center = Vec2::new(
            ellipse.top_left.x + (ellipse.size.x * 0.5),
            ellipse.top_left.y + (ellipse.size.y * 0.5),
        );

        vertexes.push(Vertex::new_colored(center, ellipse.color));

        const STEPS: u16 = 32;
        const VERT_COUNT: u16 = STEPS + 2;
        let x_radius = ellipse.size.x * 0.5;
        let y_radius = ellipse.size.y * 0.5;

        for n in 0..VERT_COUNT {
            let curr_step = n as f32 * 2.0 * PI / STEPS as f32;
            let x = center.x + (x_radius * cosf(curr_step));
            let y = center.y + (y_radius * sinf(curr_step));

            vertexes.push(Vertex::new_colored(Vec2::new(x, y), ellipse.color));
        }

        self.render_vertex_list(&vertexes, DrawHint::TriFan)
    }

    #[cfg(feature = "text")]
    fn render_text(&mut self, text: &Text) -> Result<(), Self::Error> {
        use rusttype::{point, Font, Scale};
        let mut vertexes = Vec::new();

        const THRESHOLD: f32 = 0.2;

        let ttf = Font::try_from_bytes(include_bytes!("../assets/font.ttf")).unwrap();
        let scale = Scale::uniform(text.px_size);
        let v_metrics = ttf.v_metrics(scale);

        let glyphs: Vec<_> = ttf
            .layout(text.text, scale, point(20.0, 20.0 + v_metrics.ascent))
            .collect();

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    if v > THRESHOLD {
                        vertexes.push(Vertex::new_colored(
                            Vec2::new(
                                text.top_left.x + x as f32 + bounding_box.min.x as f32,
                                text.top_left.y + y as f32 + bounding_box.min.y as f32,
                            ),
                            Color::new(text.color.r, text.color.g, text.color.b, v),
                        ));
                    }
                });
            }
        }

        self.render_vertex_list(&vertexes, DrawHint::Points)
    }
}
