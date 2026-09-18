#![feature(phantom_variance_markers)]

pub mod containers;
pub mod paint;
mod types;
pub use types::*;
pub mod galley;
pub mod num;
pub mod painter;
pub mod response;
pub mod style;
pub mod ui;
pub mod widgets;
pub mod input;

pub mod closure;
pub mod vtable;

pub(crate) mod id;

pub use ecolor::Color32;
pub use emath::{Pos2, Rangef, Rect, RectTransform, TSTransform, Vec2, pos2, vec2};
