use egui::Style;
use vtable::{VRef, VRefMut, vtable};

use crate::vtable::{
    interaction::InteractionFfiVTable, spacing::SpacingFfiVTable, visuals::VisualsFfiVTable,
};

#[vtable]
#[repr(C)]
pub struct StyleFfiVTable {
    spacing: fn(VRef<StyleFfiVTable>) -> VRef<SpacingFfiVTable>,
    spacing_mut: fn(VRefMut<StyleFfiVTable>) -> VRefMut<SpacingFfiVTable>,
    interaction: fn(VRef<StyleFfiVTable>) -> VRef<InteractionFfiVTable>,
    interaction_mut: fn(VRefMut<StyleFfiVTable>) -> VRefMut<InteractionFfiVTable>,
    visuals: fn(VRef<StyleFfiVTable>) -> VRef<VisualsFfiVTable>,
    visuals_mut: fn(VRefMut<StyleFfiVTable>) -> VRefMut<VisualsFfiVTable>,
}

impl StyleFfi for Style {
    #[inline]
    fn spacing(&self) -> VRef<'_, SpacingFfiVTable> {
        VRef::new(&self.spacing)
    }

    #[inline]
    fn spacing_mut(&mut self) -> VRefMut<'_, SpacingFfiVTable> {
        VRefMut::new(&mut self.spacing)
    }

    #[inline]
    fn interaction(&self) -> VRef<'_, InteractionFfiVTable> {
        VRef::new(&self.interaction)
    }

    #[inline]
    fn interaction_mut(&mut self) -> VRefMut<'_, InteractionFfiVTable> {
        VRefMut::new(&mut self.interaction)
    }

    #[inline]
    fn visuals(&self) -> VRef<'_, VisualsFfiVTable> {
        VRef::new(&self.visuals)
    }

    #[inline]
    fn visuals_mut(&mut self) -> VRefMut<'_, VisualsFfiVTable> {
        VRefMut::new(&mut self.visuals)
    }
}

StyleFfiVTable_static!(static STYLEFFI_VT for Style);
