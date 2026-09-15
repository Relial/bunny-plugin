use egui::{Spacing, Vec2};
use vtable::vtable;

#[vtable]
#[repr(C)]
pub struct SpacingFfiVTable {
    item_spacing: fn(VRef<SpacingFfiVTable>) -> Vec2,
}

impl SpacingFfi for Spacing {
    #[inline]
    fn item_spacing(&self) -> Vec2 {
        self.item_spacing
    }
}

SpacingFfiVTable_static!(static SPACINGFFI_VT for Spacing);
