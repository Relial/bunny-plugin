use vtable::vtable;

#[vtable]
#[repr(C)]
pub struct ScrollStyleFfiVTable {
    floating: fn(VRef<ScrollStyleFfiVTable>) -> bool,
}

impl ScrollStyleFfi for egui::style::ScrollStyle {
    fn floating(&self) -> bool {
        todo!()
    }
}

ScrollStyleFfiVTable_static!(static SCROLLSTYLEFFI_VT for egui::style::ScrollStyle);
