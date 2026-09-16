use vtable::{VRef, VRefMut};

use crate::vtable::interaction::InteractionFfiVTable;

#[repr(transparent)]
pub struct BunnyInteraction<'a> {
    inner: VRef<'a, InteractionFfiVTable>,
}

impl<'a> BunnyInteraction<'a> {
    #[inline]
    pub fn new(interaction: &'a egui::style::Interaction) -> Self {
        Self {
            inner: VRef::new(interaction),
        }
    }
}

#[repr(transparent)]
pub struct BunnyInteractionMut<'a> {
    inner: VRefMut<'a, InteractionFfiVTable>,
}

impl<'a> BunnyInteractionMut<'a> {
    #[inline]
    pub fn new(interaction: &'a mut egui::style::Interaction) -> Self {
        Self {
            inner: VRefMut::new(interaction),
        }
    }
}
