use vtable::{VRef, VRefMut};

use crate::{
    Align, BunnyInteractionMut, BunnyInteractionRef, BunnySpacingMut, BunnySpacingRef,
    BunnyVisualsMut, BunnyVisualsRef, FrameStyle, Interaction, ScrollAnimation, Spacing, TextStyle,
    Visuals,
    paint::{FontId, TextWrapMode},
    vtable::style::base::StyleFfiVTable,
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

#[cfg(feature = "manager")]
impl<'a> BunnyStyleRef<'a> {
    #[inline]
    pub fn new(style: &'a egui::Style) -> Self {
        Self {
            inner: VRef::new(style),
        }
    }
}

impl BunnyStyleRef<'_> {
    #[inline]
    pub fn spacing(&self) -> BunnySpacingRef<'_> {
        self.inner.spacing()
    }

    #[inline]
    pub fn spacing_clone(&self) -> Spacing {
        self.inner.spacing_clone()
    }

    #[inline]
    pub fn interaction(&self) -> BunnyInteractionRef<'_> {
        self.inner.interaction()
    }

    #[inline]
    pub fn interaction_clone(&self) -> Interaction {
        self.inner.interaction_clone()
    }

    #[inline]
    pub fn visuals(&self) -> BunnyVisualsRef<'_> {
        self.inner.visuals()
    }

    #[inline]
    pub fn visuals_clone(&self) -> Visuals {
        self.inner.visuals_clone()
    }
}

impl BunnyStyleRef<'_> {
    pub(crate) fn frame_style(&self) -> FrameStyle {
        self.inner.frame_style()
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

#[cfg(feature = "manager")]
impl<'a> BunnyStyleMut<'a> {
    #[inline]
    pub fn new(style: &'a mut egui::Style) -> Self {
        Self {
            inner: VRefMut::new(style),
        }
    }
}

impl BunnyStyleMut<'_> {
    #[inline]
    pub fn set_override_text_style(&mut self, text_style: Option<TextStyle>) {
        self.inner.set_override_text_style(text_style.into());
    }

    #[inline]
    pub fn set_override_font_id(&mut self, font_id: Option<FontId>) {
        self.inner.set_override_font_id(font_id.into());
    }

    #[inline]
    pub fn set_override_text_valign(&mut self, align: Option<Align>) {
        self.inner.set_override_text_valign(align.into());
    }

    #[inline]
    pub fn set_drag_value_text_style(&mut self, text_style: TextStyle) {
        self.inner.set_drag_value_text_style(text_style);
    }

    #[inline]
    pub fn set_wrap_mode(&mut self, wrap_mode: Option<TextWrapMode>) {
        self.inner.set_wrap_mode(wrap_mode.into());
    }

    #[inline]
    pub fn set_animation_time(&mut self, animation_time: f32) {
        self.inner.set_animation_time(animation_time);
    }

    #[inline]
    pub fn set_explanation_tooltips(&mut self, explanation_tooltips: bool) {
        self.inner.set_explanation_tooltips(explanation_tooltips);
    }

    #[inline]
    pub fn set_always_scroll_the_only_direction(&mut self, always_scroll_the_only_direction: bool) {
        self.inner
            .set_always_scroll_the_only_direction(always_scroll_the_only_direction);
    }

    #[inline]
    pub fn set_scroll_animation(&mut self, scroll_animation: ScrollAnimation) {
        self.inner.set_scroll_animation(scroll_animation);
    }

    #[inline]
    pub fn set_compact_menu_style(&mut self, compact_menu_style: bool) {
        self.inner.set_compact_menu_style(compact_menu_style);
    }

    #[inline]
    pub fn menu_style(&mut self) {
        self.inner.menu_style();
    }
}

impl BunnyStyleMut<'_> {
    #[inline]
    pub fn spacing(&self) -> BunnySpacingRef<'_> {
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
    pub fn interaction(&self) -> BunnyInteractionRef<'_> {
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
    pub fn visuals(&self) -> BunnyVisualsRef<'_> {
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
