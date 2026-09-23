#![feature(phantom_variance_markers)]

mod containers;
mod id;
mod types;
mod ui;
mod widgets;
mod style;
mod response;
mod painter;
mod num;
mod galley;
mod input;

pub use containers::*;
pub use id::*;
pub use types::*;
pub use ui::*;
pub use widgets::*;
pub use style::*;
pub use response::*;
pub use painter::*;
pub use num::*;
pub use galley::*;
pub use input::*;

pub mod paint;

pub(crate) mod closure;
pub(crate) mod vtable;

pub use ecolor;

pub use ecolor::{Color32, Rgba};
pub use emath::{Pos2, Rangef, Rect, RectTransform, TSTransform, Vec2, lerp, pos2, vec2};
