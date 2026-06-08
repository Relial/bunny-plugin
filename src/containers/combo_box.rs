use std::hash::Hash;

use abi_stable::std_types::{
    RHashMap,
    ROption::{self, RNone, RSome},
};
use egui::{Id, Ui};

use crate::{
    containers::popup::PopupCloseBehavior, elements::{Container, UiContainer}, input_state::Input, layout::Layout, paint::text::text_layout_types::TextWrapMode, response::{InnerResponse, Response}, ui::BunnyUi, widget_text::WidgetText
};

#[repr(C)]
pub struct ComboBox {
    id: Id,
    label: ROption<WidgetText>,
    selected_text: WidgetText,
    width: ROption<f32>,
    height: ROption<f32>,
    wrap_mode: ROption<TextWrapMode>,
    close_behavior: ROption<PopupCloseBehavior>,
}

impl ComboBox {
    pub fn new(id_salt: impl Hash, label: impl Into<WidgetText>) -> Self {
        Self {
            id: Id::new(id_salt),
            label: RSome(label.into()),
            selected_text: Default::default(),
            width: RNone,
            height: RNone,
            wrap_mode: RNone,
            close_behavior: RNone,
        }
    }

    pub fn from_id_salt(id_salt: impl Hash) -> Self {
        Self {
            id: Id::new(id_salt),
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

    pub fn show_ui<'a, R>(
        self,
        ui: &mut BunnyUi<'a>,
        menu_contents: impl FnOnce(&mut BunnyUi<'a>) -> R,
    ) -> InnerResponse<R> {
        let mut new = ui.new_child(Some(Layout::default()));
        let ret = menu_contents(&mut new);
        let response = ui.add_component_auto_id(Container::ComboBox(ComboBoxComponent {
            contents: new,
            combo_box: self,
        }));
        InnerResponse::new(ret, response)
    }
}

#[repr(C)]
pub struct ComboBoxComponent<'a> {
    contents: BunnyUi<'a>,
    combo_box: ComboBox,
}

impl UiContainer for ComboBoxComponent<'_> {
    fn ui(
        self,
        ui: &mut Ui,
        responses: &mut RHashMap<Id, Response, rapidhash::fast::RandomState>,
        input: Input,
        id: Id,
    ) -> Response {
        let mut combo_box = if let RSome(label) = self.combo_box.label {
            egui::ComboBox::new(self.combo_box.id, label)
        } else {
            egui::ComboBox::from_id_salt(self.combo_box.id)
                .selected_text(self.combo_box.selected_text)
        };
        if let RSome(width) = self.combo_box.width {
            combo_box = combo_box.width(width);
        }
        if let RSome(height) = self.combo_box.height {
            combo_box = combo_box.height(height);
        }
        if let RSome(wrap_mode) = self.combo_box.wrap_mode {
            combo_box = combo_box.wrap_mode(wrap_mode.into());
        }
        if let RSome(close_behavior) = self.combo_box.close_behavior {
            combo_box = combo_box.close_behavior(close_behavior.into());
        }

        let resp = combo_box
            .show_ui(ui, |ui| {
                self.contents.ui(ui, responses, input.clone());
            })
            .response;
        Response::new(id, resp, input)
    }
}

impl<'a> From<ComboBoxComponent<'a>> for Container<'a> {
    fn from(value: ComboBoxComponent<'a>) -> Self {
        Self::ComboBox(value)
    }
}
