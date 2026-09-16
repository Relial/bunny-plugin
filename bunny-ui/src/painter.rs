use vtable::{VBox, VRef};

use crate::vtable::painter::PainterFfiVTable;

#[repr(transparent)]
pub struct BunnyPainter<'a> {
    inner: BunnyPainterImpl<'a>,
}

impl<'a> BunnyPainter<'a> {
    #[inline]
    pub fn new(painter: impl Into<BunnyPainter<'a>>) -> Self {
        painter.into()
    }
}

#[repr(C)]
enum BunnyPainterImpl<'a> {
    Borrowed(VRef<'a, PainterFfiVTable>),
    Owned(VBox<PainterFfiVTable>),
}

impl<'a> From<&'a egui::Painter> for BunnyPainter<'a> {
    fn from(value: &'a egui::Painter) -> Self {
        Self {
            inner: BunnyPainterImpl::Borrowed(VRef::new(value)),
        }
    }
}

impl From<egui::Painter> for BunnyPainter<'_> {
    fn from(value: egui::Painter) -> Self {
        Self {
            inner: BunnyPainterImpl::Owned(VBox::new(value)),
        }
    }
}
