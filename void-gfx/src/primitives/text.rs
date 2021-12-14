use crate::{
    geometry::{Color, Vec2, Vertex},
    renderable::Renderable,
    renderer::{DrawHint, Renderer},
};
use alloc::vec::Vec;

pub struct Text {
    pub top_left: Vec2,
    pub text: &'static str,
    pub px_size: f32,
    pub color: Color,
}

impl Text {
    pub fn new(top_left: Vec2, text: &'static str, px_size: f32, color: Color) -> Self {
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
        R: Renderer,
    {
        let vertexes = self.build_vertices();
        let draw_hint = self.get_draw_hint();
        renderer.render_vertex_list(&vertexes, draw_hint)
    }

    fn build_vertices(&self) -> Vec<Vertex> {
        use rusttype::{point, Font, Scale};
        let mut vertexes = Vec::new();

        const THRESHOLD: f32 = 0.2;

        let ttf = Font::try_from_bytes(include_bytes!("../../assets/font.ttf")).unwrap();
        let scale = Scale::uniform(self.px_size);
        let v_metrics = ttf.v_metrics(scale);

        let glyphs: Vec<_> = ttf
            .layout(self.text, scale, point(20.0, 20.0 + v_metrics.ascent))
            .collect();

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    if v > THRESHOLD {
                        vertexes.push(Vertex::new_colored(
                            Vec2::new(
                                self.top_left.x + x as f32 + bounding_box.min.x as f32,
                                self.top_left.y + y as f32 + bounding_box.min.y as f32,
                            ),
                            Color::new(self.color.r, self.color.g, self.color.b, v),
                        ));
                    }
                });
            }
        }

        vertexes
    }

    fn get_draw_hint(&self) -> DrawHint {
        DrawHint::Points
    }
}
