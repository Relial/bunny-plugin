use vtable::{VRef, VRefMut};

use crate::vtable::style::interaction::InteractionFfiVTable;

pub trait BunnyInteraction {
    fn as_ref(&self) -> VRef<'_, InteractionFfiVTable>;

    #[inline]
    fn interact_radius(&self) -> f32 {
        self.as_ref().interact_radius()
    }

    #[inline]
    fn resize_grab_radius_side(&self) -> f32 {
        self.as_ref().resize_grab_radius_side()
    }

    #[inline]
    fn resize_grab_radius_corner(&self) -> f32 {
        self.as_ref().resize_grab_radius_corner()
    }

    #[inline]
    fn show_tooltips_only_when_still(&self) -> bool {
        self.as_ref().show_tooltips_only_when_still()
    }

    #[inline]
    fn tooltip_delay(&self) -> f32 {
        self.as_ref().tooltip_delay()
    }

    #[inline]
    fn tooltip_grace_time(&self) -> f32 {
        self.as_ref().tooltip_grace_time()
    }

    #[inline]
    fn selectable_label(&self) -> bool {
        self.as_ref().selectable_labels()
    }

    #[inline]
    fn multi_widget_text_select(&self) -> bool {
        self.as_ref().multi_widget_text_select()
    }
}

#[repr(transparent)]
pub struct BunnyInteractionRef<'a> {
    inner: VRef<'a, InteractionFfiVTable>,
}

impl<'a> BunnyInteractionRef<'a> {
    #[inline]
    pub fn new(interaction: &'a egui::style::Interaction) -> Self {
        Self {
            inner: VRef::new(interaction),
        }
    }
}

impl BunnyInteraction for BunnyInteractionRef<'_> {
    fn as_ref(&self) -> VRef<'_, InteractionFfiVTable> {
        self.inner
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

impl BunnyInteractionMut<'_> {
    #[inline]
    pub fn set_interact_radius(&mut self, interact_radius: f32) {
        self.inner.set_interact_radius(interact_radius);
    }

    #[inline]
    pub fn set_resize_grab_radius_side(&mut self, resize_grab_radius_side: f32) {
        self.inner
            .set_resize_grab_radius_side(resize_grab_radius_side);
    }

    #[inline]
    pub fn set_resize_grab_radius_corner(&mut self, resize_grab_radius_corner: f32) {
        self.inner
            .set_resize_grab_radius_corner(resize_grab_radius_corner);
    }

    #[inline]
    pub fn set_show_tooltips_only_when_still(&mut self, show_tooltips_only_when_still: bool) {
        self.inner
            .set_show_tooltips_only_when_still(show_tooltips_only_when_still);
    }

    #[inline]
    pub fn set_tooltip_delay(&mut self, tooltip_delay: f32) {
        self.inner.set_tooltip_delay(tooltip_delay);
    }

    #[inline]
    pub fn set_tooltip_grace_time(&mut self, tooltip_grace_time: f32) {
        self.inner.set_tooltip_grace_time(tooltip_grace_time);
    }

    #[inline]
    pub fn set_selectable_labels(&mut self, selectable_labels: bool) {
        self.inner.set_selectable_labels(selectable_labels);
    }

    #[inline]
    pub fn set_multi_widget_text_select(&mut self, multi_widget_text_select: bool) {
        self.inner
            .set_multi_widget_text_select(multi_widget_text_select);
    }
}

impl BunnyInteraction for BunnyInteractionMut<'_> {
    fn as_ref(&self) -> VRef<'_, InteractionFfiVTable> {
        self.inner.borrow()
    }
}
