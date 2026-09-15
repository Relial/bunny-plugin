use egui::style::Interaction;
use vtable::{VRef, vtable};

#[vtable]
#[repr(C)]
pub struct InteractionFfiVTable {
    interact_radius: fn(VRef<InteractionFfiVTable>) -> f32,
}

impl InteractionFfi for Interaction {
    #[inline]
    fn interact_radius(&self) -> f32 {
        self.interact_radius
    }
}

InteractionFfiVTable_static!(static INTERACTIONFFI_VT for Interaction);
