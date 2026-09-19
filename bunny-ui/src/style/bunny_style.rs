use vtable::{VRef, VRefMut};

use crate::vtable::style::StyleFfiVTable;

pub trait BunnyStyle {
    fn as_ref(&self) -> VRef<'_, StyleFfiVTable>;
}

#[repr(transparent)]
pub struct BunnyStyleRef<'a> {
    inner: VRef<'a, StyleFfiVTable>,
}

impl<'a> BunnyStyleRef<'a> {
    #[inline]
    pub fn new(style: &'a egui::Style) -> Self {
        Self {
            inner: VRef::new(style),
        }
    }
}

impl BunnyStyle for BunnyStyleRef<'_> {
    fn as_ref(&self) -> VRef<'_, StyleFfiVTable> {
        self.inner
    }
}

#[repr(transparent)]
pub struct BunnyStyleMut<'a> {
    inner: VRefMut<'a, StyleFfiVTable>,
}

impl<'a> BunnyStyleMut<'a> {
    #[inline]
    pub fn new(style: &'a mut egui::Style) -> Self {
        Self {
            inner: VRefMut::new(style),
        }
    }
}

impl BunnyStyle for BunnyStyleMut<'_> {
    fn as_ref(&self) -> VRef<'_, StyleFfiVTable> {
        self.inner.borrow()
    }
}
