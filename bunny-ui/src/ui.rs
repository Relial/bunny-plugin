use std::ops::{Deref, DerefMut};

use vtable::{VRef, VRefMut};

use crate::{
    WidgetText,
    closure::PluginClosure,
    containers::collapsing_header::CollapsingHeader,
    response::BunnyResponse,
    vtable::{style::StyleFfiVTable, ui::UiFfiVTable},
};

#[repr(C)]
pub struct BunnyUi<'a>(VRefMut<'a, UiFfiVTable>);

impl<'a> BunnyUi<'a> {
    #[inline]
    pub fn new(ui: &'a mut egui::Ui) -> Self {
        Self(VRefMut::new(ui))
    }

    #[inline]
    pub fn style(&self) -> BunnyStyle<'_> {
        self.0.style().into()
    }

    #[inline]
    pub fn style_mut(&mut self) -> BunnyStyleMut<'_> {
        self.0.style_mut().into()
    }
}

impl<'a> BunnyUi<'a> {
    #[inline]
    pub fn label(&mut self, text: impl Into<WidgetText>) -> BunnyResponse {
        self.0.label(text.into()).into()
    }
}

impl<'a> BunnyUi<'a> {
    #[inline]
    pub(crate) fn collapsing_header_show(
        &mut self,
        collapsing_header: CollapsingHeader,
        mut add_contents: impl FnMut(&mut BunnyUi),
    ) {
        let closure = PluginClosure::new(&mut add_contents);
        self.0.collapsing_header_show(collapsing_header, closure);
    }
}

#[repr(C)]
pub struct BunnyStyle<'a>(VRef<'a, StyleFfiVTable>);

impl<'a> Deref for BunnyStyle<'a> {
    type Target = VRef<'a, StyleFfiVTable>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<VRef<'a, StyleFfiVTable>> for BunnyStyle<'a> {
    #[inline]
    fn from(value: VRef<'a, StyleFfiVTable>) -> Self {
        Self(value)
    }
}

#[repr(C)]
pub struct BunnyStyleMut<'a>(VRefMut<'a, StyleFfiVTable>);

impl<'a> Deref for BunnyStyleMut<'a> {
    type Target = VRefMut<'a, StyleFfiVTable>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BunnyStyleMut<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<VRefMut<'a, StyleFfiVTable>> for BunnyStyleMut<'a> {
    #[inline]
    fn from(value: VRefMut<'a, StyleFfiVTable>) -> Self {
        Self(value)
    }
}
