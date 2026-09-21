use abi_stable::std_types::{
    ROption::{self, RNone, RSome},
    Tuple2,
};
use egui::Id;

use crate::{
    WidgetText,
    closure::PluginClosure,
    containers::PopupCloseBehavior,
    paint::text::text_layout_types::TextWrapMode,
    response::{BunnyInnerResponse, BunnyResponse},
    ui::BunnyUi,
};

#[repr(C)]
pub struct ComboBox {
    label: ROption<WidgetText>,
    selected_text: WidgetText,
    id: Id,
    width: ROption<f32>,
    height: ROption<f32>,
    wrap_mode: ROption<TextWrapMode>,
    close_behavior: ROption<PopupCloseBehavior>,
}

impl ComboBox {
    #[inline]
    pub fn new(id: impl Into<Id>, label: impl Into<WidgetText>) -> Self {
        Self {
            id: id.into(),
            label: RSome(label.into()),
            selected_text: Default::default(),
            width: RNone,
            height: RNone,
            wrap_mode: RNone,
            close_behavior: RNone,
        }
    }

    #[inline]
    pub fn from_id(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            label: RNone,
            selected_text: Default::default(),
            width: RNone,
            height: RNone,
            wrap_mode: RNone,
            close_behavior: RNone,
        }
    }

    #[inline]
    pub fn width(mut self, width: f32) -> Self {
        self.width = RSome(width);
        self
    }

    #[inline]
    pub fn height(mut self, height: f32) -> Self {
        self.height = RSome(height);
        self
    }

    #[inline]
    pub fn selected_text(mut self, selected_text: impl Into<WidgetText>) -> Self {
        self.selected_text = selected_text.into();
        self
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
    pub fn close_behavior(mut self, close_behavior: PopupCloseBehavior) -> Self {
        self.close_behavior = RSome(close_behavior);
        self
    }

    #[inline]
    pub fn show<R>(
        self,
        ui: &mut BunnyUi,
        add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyInnerResponse<Option<R>> {
        ui.combo_box_show(self, add_contents)
    }
}

impl ComboBox {
    pub(crate) fn show_impl(
        self,
        ui: &mut egui::Ui,
        contents: PluginClosure,
    ) -> Tuple2<BunnyResponse, bool> {
        let ComboBox {
            label,
            selected_text,
            id,
            width,
            height,
            wrap_mode,
            close_behavior,
        } = self;
        let mut combo_box = if let RSome(label) = label {
            egui::ComboBox::new(id, label).selected_text(selected_text)
        } else {
            egui::ComboBox::from_id_salt(id).selected_text(selected_text)
        };
        if let RSome(width) = width {
            combo_box = combo_box.width(width);
        }
        if let RSome(height) = height {
            combo_box = combo_box.height(height);
        }
        if let RSome(wrap_mode) = wrap_mode {
            combo_box = combo_box.wrap_mode(wrap_mode.into());
        }
        if let RSome(close_behavior) = close_behavior {
            combo_box = combo_box.close_behavior(close_behavior.into());
        }

        let inner_response = combo_box.show_ui(ui, |ui| {
            let mut b = BunnyUi::new(ui);
            contents.call(&mut b);
        });
        Tuple2(
            BunnyResponse::new(inner_response.response),
            inner_response.inner.is_some(),
        )
    }
}
