use vtable::VRefMut;

use crate::vtable::input::InputStateFfiVTable;

#[repr(C)]
pub struct BunnyInputState<'a> {
    inner: VRefMut<'a, InputStateFfiVTable>,
}

impl<'a> BunnyInputState<'a> {
    #[inline]
    pub fn new(input_state: &'a mut egui::InputState) -> Self {
        Self {
            inner: VRefMut::new(input_state),
        }
    }
}
