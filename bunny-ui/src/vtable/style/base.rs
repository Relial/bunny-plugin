use abi_stable::std_types::ROption;
use egui::Style;
use vtable::{VRef, VRefMut, vtable};

use crate::{
    Align,
    paint::text::{fonts::FontId, text_layout_types::TextWrapMode},
    style::{
        BunnyInteractionMut, BunnyInteractionRef, BunnySpacingMut, BunnySpacingRef,
        BunnyVisualsMut, BunnyVisualsRef, Interaction, ScrollAnimation, Spacing, TextStyle,
        Visuals,
    },
};

#[vtable]
#[repr(C)]
pub struct StyleFfiVTable {
    override_text_style: fn(VRef<StyleFfiVTable>) -> ROption<TextStyle>,
    override_font_id: fn(VRef<StyleFfiVTable>) -> ROption<FontId>,
    override_text_valign: fn(VRef<StyleFfiVTable>) -> ROption<Align>,
    drag_value_text_style: fn(VRef<StyleFfiVTable>) -> TextStyle,
    // number_formatter
    // wrap
    wrap_mode: fn(VRef<StyleFfiVTable>) -> ROption<TextWrapMode>,
    animation_time: fn(VRef<StyleFfiVTable>) -> f32,
    // debug
    explanation_tooltips: fn(VRef<StyleFfiVTable>) -> bool,
    // url_in_tooltip
    always_scroll_the_only_direction: fn(VRef<StyleFfiVTable>) -> bool,
    scroll_animation: fn(VRef<StyleFfiVTable>) -> ScrollAnimation,
    compact_menu_style: fn(VRef<StyleFfiVTable>) -> bool,

    set_override_text_style: fn(VRefMut<StyleFfiVTable>, text_style: ROption<TextStyle>),
    set_override_font_id: fn(VRefMut<StyleFfiVTable>, font_id: ROption<FontId>),
    set_override_text_valign: fn(VRefMut<StyleFfiVTable>, text_valign: ROption<Align>),
    set_drag_value_text_style: fn(VRefMut<StyleFfiVTable>, text_style: TextStyle),
    set_wrap_mode: fn(VRefMut<StyleFfiVTable>, wrap_mode: ROption<TextWrapMode>),
    set_animation_time: fn(VRefMut<StyleFfiVTable>, animation_time: f32),
    set_explanation_tooltips: fn(VRefMut<StyleFfiVTable>, explanation_tooltips: bool),
    set_always_scroll_the_only_direction:
        fn(VRefMut<StyleFfiVTable>, always_scroll_the_only_direction: bool),
    set_scroll_animation: fn(VRefMut<StyleFfiVTable>, scroll_animation: ScrollAnimation),
    set_compact_menu_style: fn(VRefMut<StyleFfiVTable>, compact_menu_style: bool),

    spacing: fn(VRef<StyleFfiVTable>) -> BunnySpacingRef,
    spacing_mut: fn(VRefMut<StyleFfiVTable>) -> BunnySpacingMut,
    spacing_clone: fn(VRef<StyleFfiVTable>) -> Spacing,
    set_spacing: fn(VRefMut<StyleFfiVTable>, spacing: &Spacing),
    interaction: fn(VRef<StyleFfiVTable>) -> BunnyInteractionRef,
    interaction_mut: fn(VRefMut<StyleFfiVTable>) -> BunnyInteractionMut,
    interaction_clone: fn(VRef<StyleFfiVTable>) -> Interaction,
    set_interaction: fn(VRefMut<StyleFfiVTable>, interaction: &Interaction),
    visuals: fn(VRef<StyleFfiVTable>) -> BunnyVisualsRef,
    visuals_mut: fn(VRefMut<StyleFfiVTable>) -> BunnyVisualsMut,
    visuals_clone: fn(VRef<StyleFfiVTable>) -> Visuals,
    set_visuals: fn(VRefMut<StyleFfiVTable>, visuals: &Visuals),
}

impl StyleFfi for Style {
    #[inline]
    fn override_text_style(&self) -> ROption<TextStyle> {
        self.override_text_style
            .as_ref()
            .map(|t| t.clone().into())
            .into()
    }

    #[inline]
    fn override_font_id(&self) -> ROption<FontId> {
        self.override_font_id
            .as_ref()
            .map(|f| f.clone().into())
            .into()
    }

    #[inline]
    fn override_text_valign(&self) -> ROption<Align> {
        self.override_text_valign
            .as_ref()
            .map(|a| (*a).into())
            .into()
    }

    #[inline]
    fn drag_value_text_style(&self) -> TextStyle {
        self.drag_value_text_style.clone().into()
    }

    #[inline]
    fn wrap_mode(&self) -> ROption<TextWrapMode> {
        self.wrap_mode.as_ref().map(|w| (*w).into()).into()
    }

    #[inline]
    fn animation_time(&self) -> f32 {
        self.animation_time
    }

    #[inline]
    fn explanation_tooltips(&self) -> bool {
        self.explanation_tooltips
    }

    #[inline]
    fn always_scroll_the_only_direction(&self) -> bool {
        self.always_scroll_the_only_direction
    }

    #[inline]
    fn scroll_animation(&self) -> ScrollAnimation {
        self.scroll_animation.into()
    }

    #[inline]
    fn compact_menu_style(&self) -> bool {
        self.compact_menu_style
    }

    #[inline]
    fn set_override_text_style(&mut self, text_style: ROption<TextStyle>) {
        self.override_text_style = text_style.map(|t| t.into()).into_option()
    }

    #[inline]
    fn set_override_font_id(&mut self, font_id: ROption<FontId>) {
        self.override_font_id = font_id.map(|f| f.into()).into_option()
    }

    #[inline]
    fn set_override_text_valign(&mut self, align: ROption<Align>) {
        self.override_text_valign = align.map(|a| a.into()).into_option()
    }

    #[inline]
    fn set_drag_value_text_style(&mut self, text_style: TextStyle) {
        self.drag_value_text_style = text_style.into()
    }

    #[inline]
    fn set_wrap_mode(&mut self, wrap_mode: ROption<TextWrapMode>) {
        self.wrap_mode = wrap_mode.map(|w| w.into()).into_option()
    }

    #[inline]
    fn set_animation_time(&mut self, animation_time: f32) {
        self.animation_time = animation_time
    }

    #[inline]
    fn set_explanation_tooltips(&mut self, explanation_tooltips: bool) {
        self.explanation_tooltips = explanation_tooltips
    }

    #[inline]
    fn set_always_scroll_the_only_direction(&mut self, always_scroll_the_only_direction: bool) {
        self.always_scroll_the_only_direction = always_scroll_the_only_direction
    }

    #[inline]
    fn set_scroll_animation(&mut self, scroll_animation: ScrollAnimation) {
        self.scroll_animation = scroll_animation.into()
    }

    #[inline]
    fn set_compact_menu_style(&mut self, compact_menu_style: bool) {
        self.compact_menu_style = compact_menu_style
    }

    #[inline]
    fn spacing<'a>(&'a self) -> BunnySpacingRef<'a> {
        BunnySpacingRef::new(&self.spacing)
    }

    #[inline]
    fn spacing_mut(&mut self) -> BunnySpacingMut<'_> {
        BunnySpacingMut::new(&mut self.spacing)
    }

    #[inline]
    fn spacing_clone(&self) -> Spacing {
        self.spacing.clone().into()
    }

    #[inline]
    fn set_spacing(&mut self, spacing: &Spacing) {
        self.spacing = spacing.into()
    }

    #[inline]
    fn interaction(&self) -> BunnyInteractionRef<'_> {
        BunnyInteractionRef::new(&self.interaction)
    }

    #[inline]
    fn interaction_mut(&mut self) -> BunnyInteractionMut<'_> {
        BunnyInteractionMut::new(&mut self.interaction)
    }

    #[inline]
    fn interaction_clone(&self) -> Interaction {
        self.interaction.clone().into()
    }

    #[inline]
    fn set_interaction(&mut self, interaction: &Interaction) {
        self.interaction = interaction.into()
    }

    #[inline]
    fn visuals(&self) -> BunnyVisualsRef<'_> {
        BunnyVisualsRef::new(&self.visuals)
    }

    #[inline]
    fn visuals_mut(&mut self) -> BunnyVisualsMut<'_> {
        BunnyVisualsMut::new(&mut self.visuals)
    }

    #[inline]
    fn visuals_clone(&self) -> Visuals {
        self.visuals.clone().into()
    }

    #[inline]
    fn set_visuals(&mut self, visuals: &Visuals) {
        self.visuals = visuals.into()
    }
}

StyleFfiVTable_static!(static STYLEFFI_VT for Style);
