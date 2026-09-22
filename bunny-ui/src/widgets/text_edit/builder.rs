use abi_stable::std_types::ROption::{self, RNone, RSome};
use ecolor::Color32;
use egui::Id;
use emath::Vec2;
use mint::Vector2;

use crate::{
    Align, Align2, Margin, WidgetText,
    containers::Frame,
    paint::text::fonts::FontSelection,
    widgets::{Widget, text_edit::bunny_string::BunnyString},
};

#[repr(C)]
pub struct TextEdit<'t> {
    font_selection: FontSelection,
    prefix: WidgetText<'t>,
    suffix: WidgetText<'t>,
    hint_text: WidgetText<'t>,
    frame: ROption<Frame>,
    id: ROption<Id>,
    text: &'t mut BunnyString,
    text_color: ROption<Color32>,
    desired_width: ROption<f32>,
    desired_height_rows: usize,
    min_size: Vec2,
    align: Align2,
    char_limit: usize,
    background_color: ROption<Color32>,
    margin: Margin,
    password: bool,
    multiline: bool,
    interactive: bool,
    cursor_at_end: bool,
    clip_text: bool,
}

impl<'t> TextEdit<'t> {
    pub fn singleline(text: &'t mut BunnyString) -> Self {
        Self {
            desired_height_rows: 1,
            multiline: false,
            clip_text: true,
            ..Self::multiline(text)
        }
    }

    pub fn multiline(text: &'t mut BunnyString) -> Self {
        Self {
            text,
            prefix: Default::default(),
            suffix: Default::default(),
            hint_text: Default::default(),
            id: RNone,
            font_selection: Default::default(),
            text_color: RNone,
            password: false,
            frame: RNone,
            margin: Margin::symmetric(4, 2),
            multiline: true,
            interactive: true,
            desired_width: RNone,
            desired_height_rows: 4,
            cursor_at_end: true,
            min_size: Vec2::ZERO,
            align: Align2::LEFT_TOP,
            clip_text: false,
            char_limit: usize::MAX,
            background_color: RNone,
        }
    }

    #[inline]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = RSome(id.into());
        self
    }

    #[inline]
    pub fn hint_text(mut self, hint_text: impl Into<WidgetText<'t>>) -> Self {
        self.hint_text = hint_text.into();
        self
    }

    #[inline]
    pub fn prefix(mut self, prefix: impl Into<WidgetText<'t>>) -> Self {
        self.prefix = prefix.into();
        self
    }

    #[inline]
    pub fn suffix(mut self, suffix: impl Into<WidgetText<'t>>) -> Self {
        self.suffix = suffix.into();
        self
    }

    #[inline]
    pub fn background_color(mut self, color: Color32) -> Self {
        self.background_color = RSome(color);
        self
    }

    #[inline]
    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }

    #[inline]
    pub fn font(mut self, font_selection: impl Into<FontSelection>) -> Self {
        self.font_selection = font_selection.into();
        self
    }

    #[inline]
    pub fn text_color(mut self, text_color: Color32) -> Self {
        self.text_color = RSome(text_color);
        self
    }

    #[inline]
    pub fn text_color_opt(mut self, text_color: Option<Color32>) -> Self {
        self.text_color = text_color.into();
        self
    }

    #[inline]
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    #[inline]
    pub fn frame(mut self, frame: Frame) -> Self {
        self.frame = RSome(frame);
        self
    }

    #[inline]
    pub fn margin(mut self, margin: impl Into<Margin>) -> Self {
        self.margin = margin.into();
        self
    }

    #[inline]
    pub fn desired_width(mut self, desired_width: f32) -> Self {
        self.desired_width = RSome(desired_width);
        self
    }

    #[inline]
    pub fn desired_rows(mut self, desired_height_rows: usize) -> Self {
        self.desired_height_rows = desired_height_rows;
        self
    }

    #[inline]
    pub fn cursor_at_end(mut self, b: bool) -> Self {
        self.cursor_at_end = b;
        self
    }

    #[inline]
    pub fn clip_text(mut self, b: bool) -> Self {
        self.clip_text = b;
        self
    }

    #[inline]
    pub fn char_limit(mut self, limit: usize) -> Self {
        self.char_limit = limit;
        self
    }

    #[inline]
    pub fn horizontal_align(mut self, align: Align) -> Self {
        self.align.0[0] = align;
        self
    }

    #[inline]
    pub fn vertical_align(mut self, align: Align) -> Self {
        self.align.0[1] = align;
        self
    }

    #[inline]
    pub fn min_size(mut self, min_size: impl Into<Vector2<f32>>) -> Self {
        self.min_size = min_size.into().into();
        self
    }
}

#[cfg(feature = "manager")]
impl egui::Widget for TextEdit<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut text_edit = if self.multiline {
            egui::TextEdit::multiline(self.text)
        } else {
            egui::TextEdit::singleline(self.text)
        }
        .hint_text(self.hint_text)
        .prefix(self.prefix)
        .suffix(self.suffix)
        .password(self.password)
        .font(self.font_selection)
        .text_color_opt(self.text_color.into())
        .interactive(self.interactive)
        .margin(self.margin)
        .desired_rows(self.desired_height_rows)
        .cursor_at_end(self.cursor_at_end)
        .clip_text(self.clip_text)
        .char_limit(self.char_limit)
        .horizontal_align(self.align.0[0].into())
        .vertical_align(self.align.0[1].into())
        .min_size(self.min_size);

        if let RSome(id) = self.id {
            text_edit = text_edit.id(id);
        }
        if let RSome(frame) = self.frame {
            text_edit = text_edit.frame(frame.into());
        }
        if let RSome(width) = self.desired_width {
            text_edit = text_edit.desired_width(width);
        }
        if let RSome(color) = self.background_color {
            text_edit = text_edit.background_color(color);
        }

        text_edit.show(ui).response.response
    }
}

impl<'t> From<TextEdit<'t>> for Widget<'t> {
    #[inline]
    fn from(value: TextEdit<'t>) -> Self {
        Self::TextEdit(value)
    }
}
