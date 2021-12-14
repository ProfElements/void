use crate::{
    geometry::Vertex,
    renderer::{DrawHint, Renderer},
};
use alloc::vec::Vec;

pub trait Renderable {
    fn render<R>(&self, renderer: &mut R) -> Result<(), R::Error>
    where
        R: Renderer;
    fn build_vertices(&self) -> Vec<Vertex>;
    fn get_draw_hint(&self) -> DrawHint;
}
