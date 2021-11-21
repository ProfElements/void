mod ellipse;
mod line;
mod polyline;
mod triangle;
mod rectangle;

#[cfg(feature = "text")]
mod text;

#[cfg(feature = "text")]
pub use text::Text;

pub use ellipse::Ellipse;
pub use line::Line;
pub use polyline::Polyline;
pub use rectangle::Rectangle;
pub use triangle::Triangle;

pub trait Primitive {
    fn outline(&self) -> Polyline;
    fn bounds(&self) -> Rectangle;
}
