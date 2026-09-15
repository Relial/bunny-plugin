use egui::Painter;
use vtable::vtable;

#[vtable]
#[repr(C)]
pub struct PainterFfiVTable {
    drop: fn(VRefMut<PainterFfiVTable>),
}

impl PainterFfi for Painter {}

PainterFfiVTable_static!(static PAINTERFFI_VT for Painter);
