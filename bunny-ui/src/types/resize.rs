use emath::{Vec2, vec2};
use mint::Vector2;

use crate::types::vec2b::Vec2b;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Resize {
    pub(crate) min_size: Vec2,
    pub(crate) max_size: Vec2,
    pub(crate) default_size: Vec2,
    pub(crate) resizable: Vec2b,
}

impl Default for Resize {
    fn default() -> Self {
        Self {
            resizable: Vec2b::TRUE,
            min_size: Vec2::splat(16.0),
            max_size: Vec2::splat(f32::INFINITY),
            default_size: vec2(320.0, 128.0),
        }
    }
}

impl Resize {
    #[inline]
    pub fn default_width(mut self, width: f32) -> Self {
        self.default_size.x = width;
        self
    }

    #[inline]
    pub fn default_height(mut self, height: f32) -> Self {
        self.default_size.y = height;
        self
    }

    #[inline]
    pub fn default_size(mut self, default_size: impl Into<Vector2<f32>>) -> Self {
        self.default_size = default_size.into().into();
        self
    }

    #[inline]
    pub fn min_width(mut self, min_width: f32) -> Self {
        self.min_size.x = min_width;
        self
    }

    #[inline]
    pub fn min_height(mut self, min_height: f32) -> Self {
        self.min_size.y = min_height;
        self
    }

    #[inline]
    pub fn min_size(mut self, min_size: impl Into<Vector2<f32>>) -> Self {
        self.min_size = min_size.into().into();
        self
    }

    #[inline]
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_size.x = max_width;
        self
    }

    #[inline]
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_size.y = max_height;
        self
    }

    #[inline]
    pub fn max_size(mut self, max_size: impl Into<Vector2<f32>>) -> Self {
        self.max_size = max_size.into().into();
        self
    }

    #[inline]
    pub fn resizable(mut self, resizable: impl Into<Vec2b>) -> Self {
        self.resizable = resizable.into();
        self
    }

    #[inline]
    pub fn is_resizable(&self) -> Vec2b {
        self.resizable
    }

    #[inline]
    pub fn auto_sized(self) -> Self {
        self.min_size(Vec2::ZERO)
            .default_size(Vec2::splat(f32::INFINITY))
            .resizable(false)
    }

    #[inline]
    pub fn fixed_size(mut self, size: impl Into<Vector2<f32>>) -> Self {
        let size: Vec2 = size.into().into();
        self.default_size = size;
        self.min_size = size;
        self.max_size = size;
        self.resizable = Vec2b::FALSE;
        self
    }
}
