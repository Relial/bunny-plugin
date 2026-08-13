mod annulus;
mod capsule2d;
mod circle;
mod convex_polygon;
mod ellipse;
mod inset;
mod rectangle;
mod rhombus;
mod ring;
mod triangle2d;

pub use annulus::*;
pub use capsule2d::*;
pub use circle::*;
pub use convex_polygon::*;
pub use ellipse::*;
pub use inset::*;
pub use rectangle::*;
pub use rhombus::*;
pub use ring::*;
pub use triangle2d::*;

pub trait Primitive2d {}
