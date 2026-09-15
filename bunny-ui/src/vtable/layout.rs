use vtable::vtable;

#[vtable]
#[repr(C)]
pub struct LayoutFfiVTable {
    // main_dir
    main_wrap: fn(VRef<LayoutFfiVTable>) -> bool,
}

impl LayoutFfi for egui::Layout {
    #[inline]
    fn main_wrap(&self) -> bool {
        self.main_wrap
    }
}

LayoutFfiVTable_static!(static LAYOUTFFI_VT for egui::Layout);
