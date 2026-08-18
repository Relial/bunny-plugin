mod core;
pub use core::*;

pub(crate) const VERTEX_SIZE: u32 = 24;

#[cfg(feature = "backend")]
pub mod backend;

#[cfg(feature = "bevy")]
pub use bevy_mesh;
pub use ecolor::Color32;
pub use glam;
#[cfg(feature = "image")]
pub use image;
#[cfg(feature = "tobj")]
pub use tobj;
pub use windows_numerics::Matrix4x4;
