use vtable::{VBox, VRef};

use crate::vtable::painter::PainterFfiVTable;

#[repr(transparent)]
pub struct BunnyPainter<'a>(BunnyPainterImpl<'a>);

#[repr(C)]
pub enum BunnyPainterImpl<'a> {
    Borrowed(VRef<'a, PainterFfiVTable>),
    Owned(VBox<PainterFfiVTable>),
}

impl<'a> From<VRef<'a, PainterFfiVTable>> for BunnyPainter<'a> {
    fn from(value: VRef<'a, PainterFfiVTable>) -> Self {
        Self(BunnyPainterImpl::Borrowed(value))
    }
}

impl From<VBox<PainterFfiVTable>> for BunnyPainter<'_> {
    fn from(value: VBox<PainterFfiVTable>) -> Self {
        Self(BunnyPainterImpl::Owned(value))
    }
}
