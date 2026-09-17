#![feature(phantom_variance_markers)]

pub mod containers;
pub mod paint;
mod types;
pub use types::*;
pub mod ui;
pub mod widgets;
pub mod style;
pub mod response;
pub mod painter;
pub mod num;

pub mod closure;
pub mod vtable;

pub(crate) mod id;

pub use ecolor::Color32;
pub use emath::{Pos2, Rangef, Rect, RectTransform, TSTransform, Vec2, pos2, vec2};
