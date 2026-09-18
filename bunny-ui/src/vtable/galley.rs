use std::sync::Arc;

use vtable::vtable;

#[vtable]
#[repr(C)]
pub struct GalleyFfiVTable {
    // job
    // rows
    elided: fn(VRef<GalleyFfiVTable>) -> bool,
    drop: fn(VRefMut<GalleyFfiVTable>),
}

impl GalleyFfi for Arc<egui::Galley> {
    fn elided(&self) -> bool {
        self.elided
    }
}

GalleyFfiVTable_static!(static GALLEYFFI_VT for Arc<egui::Galley>);
