use egui::style::Interaction;
use vtable::{VRef, vtable};

#[vtable]
#[repr(C)]
pub struct InteractionFfiVTable {
    interact_radius: fn(VRef<InteractionFfiVTable>) -> f32,
    resize_grab_radius_side: fn(VRef<InteractionFfiVTable>) -> f32,
    resize_grab_radius_corner: fn(VRef<InteractionFfiVTable>) -> f32,
    show_tooltips_only_when_still: fn(VRef<InteractionFfiVTable>) -> bool,
    tooltip_delay: fn(VRef<InteractionFfiVTable>) -> f32,
    tooltip_grace_time: fn(VRef<InteractionFfiVTable>) -> f32,
    selectable_labels: fn(VRef<InteractionFfiVTable>) -> bool,
    multi_widget_text_select: fn(VRef<InteractionFfiVTable>) -> bool,

    set_interact_radius: fn(VRefMut<InteractionFfiVTable>, interact_radius: f32),
    set_resize_grab_radius_side: fn(VRefMut<InteractionFfiVTable>, resize_grab_radius_side: f32),
    set_resize_grab_radius_corner:
        fn(VRefMut<InteractionFfiVTable>, resize_grab_radius_corner: f32),
    set_show_tooltips_only_when_still:
        fn(VRefMut<InteractionFfiVTable>, show_tooltips_only_when_still: bool),
    set_tooltip_delay: fn(VRefMut<InteractionFfiVTable>, tooltip_delay: f32),
    set_tooltip_grace_time: fn(VRefMut<InteractionFfiVTable>, tooltip_grace_time: f32),
    set_selectable_labels: fn(VRefMut<InteractionFfiVTable>, selectable_labels: bool),
    set_multi_widget_text_select: fn(VRefMut<InteractionFfiVTable>, multi_widget_text_select: bool),
}

impl InteractionFfi for Interaction {
    #[inline]
    fn interact_radius(&self) -> f32 {
        self.interact_radius
    }

    #[inline]
    fn resize_grab_radius_side(&self) -> f32 {
        self.resize_grab_radius_side
    }

    #[inline]
    fn resize_grab_radius_corner(&self) -> f32 {
        self.resize_grab_radius_corner
    }

    #[inline]
    fn show_tooltips_only_when_still(&self) -> bool {
        self.show_tooltips_only_when_still
    }

    #[inline]
    fn tooltip_delay(&self) -> f32 {
        self.tooltip_delay
    }

    #[inline]
    fn tooltip_grace_time(&self) -> f32 {
        self.tooltip_grace_time
    }

    #[inline]
    fn selectable_labels(&self) -> bool {
        self.selectable_labels
    }

    #[inline]
    fn multi_widget_text_select(&self) -> bool {
        self.multi_widget_text_select
    }

    #[inline]
    fn set_interact_radius(&mut self, interact_radius: f32) {
        self.interact_radius = interact_radius
    }

    #[inline]
    fn set_resize_grab_radius_side(&mut self, resize_grab_radius_side: f32) {
        self.resize_grab_radius_side = resize_grab_radius_side
    }

    #[inline]
    fn set_resize_grab_radius_corner(&mut self, resize_grab_radius_corner: f32) {
        self.resize_grab_radius_corner = resize_grab_radius_corner
    }

    #[inline]
    fn set_show_tooltips_only_when_still(&mut self, show_tooltips_only_when_still: bool) {
        self.show_tooltips_only_when_still = show_tooltips_only_when_still
    }

    #[inline]
    fn set_tooltip_delay(&mut self, tooltip_delay: f32) {
        self.tooltip_delay = tooltip_delay
    }

    #[inline]
    fn set_tooltip_grace_time(&mut self, tooltip_grace_time: f32) {
        self.tooltip_grace_time = tooltip_grace_time
    }

    #[inline]
    fn set_selectable_labels(&mut self, selectable_labels: bool) {
        self.selectable_labels = selectable_labels
    }

    #[inline]
    fn set_multi_widget_text_select(&mut self, multi_widget_text_select: bool) {
        self.multi_widget_text_select = multi_widget_text_select
    }
}

InteractionFfiVTable_static!(static INTERACTIONFFI_VT for Interaction);
