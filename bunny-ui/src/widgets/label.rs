use abi_stable::std_types::ROption::{self, RNone, RSome};
use egui::Sense;

use crate::{Align, WidgetText, paint::text::text_layout_types::TextWrapMode, widgets::Widget};

#[repr(C)]
pub struct Label<'a> {
    text: WidgetText<'a>,
    wrap_mode: ROption<TextWrapMode>,
    halign: ROption<Align>,
    sense: ROption<Sense>,
    selectable: ROption<bool>,
    show_tooltip_when_elided: bool,
}

impl<'a> Label<'a> {
    #[inline]
    pub fn new(text: impl Into<WidgetText<'a>>) -> Self {
        Self {
            text: text.into(),
            wrap_mode: RNone,
            sense: RNone,
            selectable: RNone,
            halign: RNone,
            show_tooltip_when_elided: true,
        }
    }

    #[inline]
    pub fn wrap_mode(mut self, wrap_mode: TextWrapMode) -> Self {
        self.wrap_mode = RSome(wrap_mode);
        self
    }

    #[inline]
    pub fn wrap(mut self) -> Self {
        self.wrap_mode = RSome(TextWrapMode::Wrap);
        self
    }

    #[inline]
    pub fn truncate(mut self) -> Self {
        self.wrap_mode = RSome(TextWrapMode::Truncate);
        self
    }

    #[inline]
    pub fn extend(mut self) -> Self {
        self.wrap_mode = RSome(TextWrapMode::Extend);
        self
    }

    #[inline]
    pub fn halign(mut self, align: Align) -> Self {
        self.halign = RSome(align);
        self
    }

    #[inline]
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = RSome(selectable);
        self
    }

    #[inline]
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = RSome(sense);
        self
    }

    #[inline]
    pub fn show_tooltip_when_elided(mut self, show: bool) -> Self {
        self.show_tooltip_when_elided = show;
        self
    }
}

#[cfg(feature = "manager")]
impl egui::Widget for Label<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut label =
            egui::Label::new(self.text).show_tooltip_when_elided(self.show_tooltip_when_elided);
        if let RSome(wrap_mode) = self.wrap_mode {
            label = label.wrap_mode(wrap_mode.into());
        }
        if let RSome(sense) = self.sense {
            label = label.sense(sense);
        }
        if let RSome(selectable) = self.selectable {
            label = label.selectable(selectable);
        }
        if let RSome(halign) = self.halign {
            label = label.halign(halign.into());
        }

        label.ui(ui)
    }
}

impl<'a> From<Label<'a>> for Widget<'a> {
    #[inline]
    fn from(value: Label<'a>) -> Self {
        Self::Label(value)
    }
}
