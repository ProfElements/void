use crate::renderer::Renderer;

pub trait Renderable {
    fn render<R>(&self, renderer: &mut R) -> Result<(), R::Error>
    where
        R: Renderer;
}
