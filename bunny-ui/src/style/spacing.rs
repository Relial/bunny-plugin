use vtable::{VRef, VRefMut};

use crate::vtable::spacing::SpacingFfiVTable;

#[repr(transparent)]
pub struct BunnySpacing<'a> {
    inner: VRef<'a, SpacingFfiVTable>,
}

impl<'a> BunnySpacing<'a> {
    #[inline]
    pub fn new(spacing: &'a egui::style::Spacing) -> Self {
        Self {
            inner: VRef::new(spacing),
        }
    }
}

#[repr(transparent)]
pub struct BunnySpacingMut<'a> {
    inner: VRefMut<'a, SpacingFfiVTable>,
}

impl<'a> BunnySpacingMut<'a> {
    #[inline]
    pub fn new(spacing: &'a mut egui::style::Spacing) -> Self {
        Self {
            inner: VRefMut::new(spacing),
        }
    }
}
