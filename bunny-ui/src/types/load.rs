use std::ops::Deref;

use abi_stable::std_types::{RCowSlice, ROption};
use egui::Vec2;

use crate::paint::SizedTexture;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum SizeHint {
    Scale(f32),
    Width(u32),
    Height(u32),
    Size {
        width: u32,
        height: u32,
        maintain_aspect_ratio: bool,
    },
}

impl Default for SizeHint {
    fn default() -> Self {
        Self::Scale(1.0)
    }
}

impl SizeHint {
    pub fn scale_by(self, factor: f32) -> Self {
        match self {
            SizeHint::Scale(scale) => Self::Scale(factor * scale),
            SizeHint::Width(width) => Self::Width((factor * width as f32).round() as _),
            SizeHint::Height(height) => Self::Height((factor * height as f32).round() as _),
            SizeHint::Size {
                width,
                height,
                maintain_aspect_ratio,
            } => Self::Size {
                width: (factor * width as f32).round() as _,
                height: (factor * height as f32).round() as _,
                maintain_aspect_ratio,
            },
        }
    }
}

#[cfg(feature = "manager")]
impl From<SizeHint> for egui::SizeHint {
    #[inline(always)]
    fn from(value: SizeHint) -> Self {
        use egui::emath::Float as _;

        match value {
            SizeHint::Scale(scale) => Self::Scale(scale.ord()),
            SizeHint::Width(width) => Self::Width(width),
            SizeHint::Height(height) => Self::Height(height),
            SizeHint::Size {
                width,
                height,
                maintain_aspect_ratio,
            } => Self::Size {
                width,
                height,
                maintain_aspect_ratio,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Bytes(RCowSlice<'static, u8>);

impl Bytes {
    #[inline]
    pub fn into_inner(self) -> RCowSlice<'static, u8> {
        self.0
    }
}

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl From<&'static [u8]> for Bytes {
    #[inline]
    fn from(value: &'static [u8]) -> Self {
        Self(value.into())
    }
}

impl<const N: usize> From<&'static [u8; N]> for Bytes {
    #[inline]
    fn from(value: &'static [u8; N]) -> Self {
        Self(RCowSlice::from_slice(value))
    }
}

impl From<Vec<u8>> for Bytes {
    #[inline]
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}

impl AsRef<[u8]> for Bytes {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        match &self.0 {
            abi_stable::std_types::RCow::Borrowed(bytes) => bytes,
            abi_stable::std_types::RCow::Owned(bytes) => bytes,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum TexturePoll {
    Pending { size: ROption<Vec2> },
    Ready { texture: SizedTexture },
}

#[cfg(feature = "manager")]
impl From<egui::load::TexturePoll> for TexturePoll {
    fn from(value: egui::load::TexturePoll) -> Self {
        match value {
            egui::load::TexturePoll::Pending { size } => Self::Pending { size: size.into() },
            egui::load::TexturePoll::Ready { texture } => Self::Ready {
                texture: texture.into(),
            },
        }
    }
}
