use std::hash::Hash;

use abi_stable::std_types::ROption::{self, RNone, RSome};
#[cfg(feature = "manager")]
use abi_stable::std_types::Tuple2;

#[cfg(feature = "manager")]
use crate::{BunnyResponse, closure::PluginClosure};
use crate::{
    Id, WidgetText, containers::PopupCloseBehavior, paint::TextWrapMode,
    response::BunnyInnerResponse, ui::BunnyUi,
};

#[repr(C)]
pub struct ComboBox<'a> {
    label: ROption<WidgetText<'a>>,
    selected_text: WidgetText<'a>,
    id_salt: Id,
    width: ROption<f32>,
    height: ROption<f32>,
    wrap_mode: ROption<TextWrapMode>,
    close_behavior: ROption<PopupCloseBehavior>,
}

impl<'a> ComboBox<'a> {
    #[inline]
    pub fn new(id_salt: impl Hash, label: impl Into<WidgetText<'a>>) -> Self {
        Self {
            id_salt: Id::new(id_salt),
            label: RSome(label.into()),
            selected_text: Default::default(),
            width: RNone,
            height: RNone,
            wrap_mode: RNone,
            close_behavior: RNone,
        }
    }

    /// A source for the Id.
    ///
    /// Note that this gets hashed twice, so the Id in the response won't match an Id created out of the same salt.
    #[inline]
    pub fn from_id_salt(id_salt: impl Hash) -> Self {
        Self {
            id_salt: Id::new(id_salt),
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
    pub fn selected_text(mut self, selected_text: impl Into<WidgetText<'a>>) -> Self {
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

#[cfg(feature = "manager")]
impl ComboBox<'_> {
    pub(crate) fn show_impl(
        self,
        ui: &mut egui::Ui,
        contents: PluginClosure,
    ) -> Tuple2<BunnyResponse, bool> {
        let ComboBox {
            label,
            selected_text,
            id_salt: id,
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
