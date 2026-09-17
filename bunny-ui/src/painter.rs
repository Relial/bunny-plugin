use vtable::{VBox, VRef};

use crate::vtable::painter::PainterFfiVTable;

#[repr(transparent)]
pub struct BunnyPainterRef<'a> {
    inner: VRef<'a, PainterFfiVTable>,
}

impl<'a> BunnyPainterRef<'a> {
    #[inline]
    pub fn new(painter: &'a egui::Painter) -> Self {
        Self {
            inner: VRef::new(painter),
        }
    }
}

impl<'a> BunnyPainterRef<'a> {}

#[repr(transparent)]
pub struct BunnyPainter {
    inner: VBox<PainterFfiVTable>,
}

impl BunnyPainter {
    #[inline]
    pub fn new(painter: egui::Painter) -> Self {
        Self {
            inner: VBox::new(painter),
        }
    }
}

impl BunnyPainter {}
