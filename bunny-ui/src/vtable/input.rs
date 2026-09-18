use egui::Vec2;
use vtable::vtable;

#[vtable]
#[repr(C)]
pub struct InputStateFfiVTable {
    // raw
    // pointer
    smooth_scroll_delta: fn(VRef<InputStateFfiVTable>) -> Vec2,
}

impl InputStateFfi for egui::InputState {
    fn smooth_scroll_delta(&self) -> Vec2 {
        todo!()
    }
}

InputStateFfiVTable_static!(static INPUTSTATEFFI_VT for egui::InputState);
