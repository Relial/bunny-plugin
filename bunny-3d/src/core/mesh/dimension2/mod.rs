mod annulus;
mod capsule2d;
mod circle;
mod convex_polygon;
mod ellipse;
mod rectangle;
mod rhombus;
mod triangle2d;

pub use annulus::*;
pub use capsule2d::*;
pub use circle::*;
pub use convex_polygon::*;
pub use ellipse::*;
pub use rectangle::*;
pub use rhombus::*;
pub use triangle2d::*;

pub trait Primitive2d {}
