use vtable::{VRef, VRefMut};

use crate::vtable::style::scroll_style::ScrollStyleFfiVTable;

#[repr(C)]
pub struct BunnyScrollStyleRef<'a> {
    inner: VRef<'a, ScrollStyleFfiVTable>,
}

impl<'a> BunnyScrollStyleRef<'a> {
    #[inline]
    pub fn new(scroll_style: &'a egui::style::ScrollStyle) -> Self {
        Self {
            inner: VRef::new(scroll_style),
        }
    }
}

#[repr(C)]
pub struct BunnyScrollStyleMut<'a> {
    inner: VRefMut<'a, ScrollStyleFfiVTable>,
}

impl<'a> BunnyScrollStyleMut<'a> {
    #[inline]
    pub fn new(scroll_style: &'a mut egui::style::ScrollStyle) -> Self {
        Self {
            inner: VRefMut::new(scroll_style),
        }
    }
}
