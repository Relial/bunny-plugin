use vtable::{VRef, VRefMut};

use crate::{
    Align,
    paint::text::{fonts::FontId, text_layout_types::TextWrapMode},
    style::{
        BunnyInteraction, BunnyInteractionMut, BunnySpacing, BunnySpacingMut, BunnyVisuals,
        BunnyVisualsMut, Interaction, ScrollAnimation, Spacing, TextStyle, Visuals,
    },
    vtable::style::StyleFfiVTable,
};

pub trait BunnyStyle {
    fn as_ref(&self) -> VRef<'_, StyleFfiVTable>;

    #[inline]
    fn override_text_style(&self) -> Option<TextStyle> {
        self.as_ref().override_text_style().into_option()
    }

    #[inline]
    fn override_font_id(&self) -> Option<FontId> {
        self.as_ref().override_font_id().into_option()
    }

    #[inline]
    fn override_text_valign(&self) -> Option<Align> {
        self.as_ref().override_text_valign().into_option()
    }

    #[inline]
    fn drag_value_text_style(&self) -> TextStyle {
        self.as_ref().drag_value_text_style()
    }

    #[inline]
    fn wrap_mode(&self) -> Option<TextWrapMode> {
        self.as_ref().wrap_mode().into_option()
    }

    #[inline]
    fn animation_time(&self) -> f32 {
        self.as_ref().animation_time()
    }

    #[inline]
    fn explanation_tooltips(&self) -> bool {
        self.as_ref().explanation_tooltips()
    }

    #[inline]
    fn always_scroll_the_only_direction(&self) -> bool {
        self.as_ref().always_scroll_the_only_direction()
    }

    #[inline]
    fn scroll_animation(&self) -> ScrollAnimation {
        self.as_ref().scroll_animation()
    }

    #[inline]
    fn compact_menu_style(&self) -> bool {
        self.as_ref().compact_menu_style()
    }
}

#[repr(transparent)]
pub struct BunnyStyleRef<'a> {
    inner: VRef<'a, StyleFfiVTable>,
}

impl<'a> BunnyStyleRef<'a> {
    #[inline]
    pub fn new(style: &'a egui::Style) -> Self {
        Self {
            inner: VRef::new(style),
        }
    }

    #[inline]
    pub fn spacing(&self) -> BunnySpacing<'_> {
        self.inner.spacing()
    }

    #[inline]
    pub fn spacing_clone(&self) -> Spacing {
        self.inner.spacing_clone()
    }

    #[inline]
    pub fn interaction(&self) -> BunnyInteraction<'_> {
        self.inner.interaction()
    }

    #[inline]
    pub fn interaction_clone(&self) -> Interaction {
        self.inner.interaction_clone()
    }

    #[inline]
    pub fn visuals(&self) -> BunnyVisuals<'_> {
        self.inner.visuals()
    }

    #[inline]
    pub fn visuals_clone(&self) -> Visuals {
        self.inner.visuals_clone()
    }
}

impl BunnyStyle for BunnyStyleRef<'_> {
    fn as_ref(&self) -> VRef<'_, StyleFfiVTable> {
        self.inner
    }
}

#[repr(transparent)]
pub struct BunnyStyleMut<'a> {
    inner: VRefMut<'a, StyleFfiVTable>,
}

impl<'a> BunnyStyleMut<'a> {
    #[inline]
    pub fn new(style: &'a mut egui::Style) -> Self {
        Self {
            inner: VRefMut::new(style),
        }
    }

    #[inline]
    pub fn spacing(&self) -> BunnySpacing<'_> {
        self.inner.spacing()
    }

    #[inline]
    pub fn spacing_mut(&mut self) -> BunnySpacingMut<'_> {
        self.inner.spacing_mut()
    }

    #[inline]
    pub fn spacing_clone(&self) -> Spacing {
        self.inner.spacing_clone()
    }

    #[inline]
    pub fn set_spacing(&mut self, spacing: &Spacing) {
        self.inner.set_spacing(spacing);
    }

    #[inline]
    pub fn interaction(&self) -> BunnyInteraction<'_> {
        self.inner.interaction()
    }

    #[inline]
    pub fn interaction_mut(&mut self) -> BunnyInteractionMut<'_> {
        self.inner.interaction_mut()
    }

    #[inline]
    pub fn interaction_clone(&self) -> Interaction {
        self.inner.interaction_clone()
    }

    #[inline]
    pub fn set_interaction(&mut self, interaction: &Interaction) {
        self.inner.set_interaction(interaction);
    }

    #[inline]
    pub fn visuals(&self) -> BunnyVisuals<'_> {
        self.inner.visuals()
    }

    #[inline]
    pub fn visuals_mut(&mut self) -> BunnyVisualsMut<'_> {
        self.inner.visuals_mut()
    }

    #[inline]
    pub fn visuals_clone(&self) -> Visuals {
        self.inner.visuals_clone()
    }

    #[inline]
    pub fn set_visuals(&mut self, visuals: &Visuals) {
        self.inner.set_visuals(visuals);
    }
}

impl BunnyStyle for BunnyStyleMut<'_> {
    fn as_ref(&self) -> VRef<'_, StyleFfiVTable> {
        self.inner.borrow()
    }
}
