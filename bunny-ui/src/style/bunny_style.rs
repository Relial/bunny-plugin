use std::ops::{Deref, DerefMut};

use vtable::{VRef, VRefMut};

use crate::vtable::style::StyleFfiVTable;

#[repr(transparent)]
pub struct BunnyStyle<'a> {
    inner: VRef<'a, StyleFfiVTable>,
}

impl<'a> BunnyStyle<'a> {
    #[inline]
    pub fn new(style: &'a egui::Style) -> Self {
        Self {
            inner: VRef::new(style),
        }
    }
}

impl<'a> Deref for BunnyStyle<'a> {
    type Target = VRef<'a, StyleFfiVTable>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
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

impl<'a> Deref for BunnyStyleMut<'a> {
    type Target = VRefMut<'a, StyleFfiVTable>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for BunnyStyleMut<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
