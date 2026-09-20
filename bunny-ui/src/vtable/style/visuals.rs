use egui::Visuals;
use vtable::vtable;

#[vtable]
#[repr(C)]
pub struct VisualsFfiVTable {
    dark_mode: fn(VRef<VisualsFfiVTable>) -> bool,
}

impl VisualsFfi for Visuals {
    #[inline]
    fn dark_mode(&self) -> bool {
        self.dark_mode
    }
}

VisualsFfiVTable_static!(static VISUALSFFI_VT for Visuals);
