use vtable::{VRef, VRefMut};

use crate::vtable::visuals::VisualsFfiVTable;

#[repr(transparent)]
pub struct BunnyVisuals<'a> {
    inner: VRef<'a, VisualsFfiVTable>,
}

impl<'a> BunnyVisuals<'a> {
    #[inline]
    pub fn new(visuals: &'a egui::style::Visuals) -> Self {
        Self {
            inner: VRef::new(visuals),
        }
    }
}

#[repr(transparent)]
pub struct BunnyVisualsMut<'a> {
    inner: VRefMut<'a, VisualsFfiVTable>,
}

impl<'a> BunnyVisualsMut<'a> {
    #[inline]
    pub fn new(visuals: &'a mut egui::style::Visuals) -> Self {
        Self {
            inner: VRefMut::new(visuals),
        }
    }
}
