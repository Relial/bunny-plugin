use std::{borrow::Cow, sync::Arc};

use abi_stable::std_types::{
    ROption::{self, RSome},
    RString,
};
use egui::Color32;

use crate::style::TextStyle;

#[repr(C)]
#[derive(Clone)]
pub enum WidgetText {
    Text(RString),
    RichText(RichText),
}

impl WidgetText {
    pub fn text(&self) -> &str {
        match self {
            WidgetText::Text(rstring) => rstring,
            WidgetText::RichText(rich_text) => rich_text.text(),
        }
    }
}

impl Default for WidgetText {
    fn default() -> Self {
        Self::Text(RString::new())
    }
}

impl From<&str> for WidgetText {
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}

impl From<&String> for WidgetText {
    fn from(value: &String) -> Self {
        Self::Text(value.clone().into())
    }
}

impl From<String> for WidgetText {
    fn from(value: String) -> Self {
        Self::Text(value.clone().into())
    }
}

impl From<Cow<'_, str>> for WidgetText {
    fn from(value: Cow<'_, str>) -> Self {
        Self::Text(value.into())
    }
}

impl From<RichText> for WidgetText {
    fn from(value: RichText) -> Self {
        Self::RichText(value)
    }
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct RichText {
    text: RString,
    size: ROption<f32>,
    text_style: ROption<TextStyle>,
    background_color: Color32,
    text_color: ROption<Color32>,
    code: bool,
    strong: bool,
    weak: bool,
    strikethrough: bool,
    underline: bool,
    italics: bool,
    raised: bool,
}

impl RichText {
    pub fn new(text: impl Into<RString>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = RSome(size);
        self
    }

    pub fn text_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = RSome(text_style);
        self
    }

    pub fn heading(self) -> Self {
        self.text_style(TextStyle::Heading)
    }

    pub fn monospace(self) -> Self {
        self.text_style(TextStyle::Monospace)
    }

    pub fn code(mut self) -> Self {
        self.code = true;
        self.text_style(TextStyle::Monospace)
    }

    pub fn strong(mut self) -> Self {
        self.strong = true;
        self
    }

    pub fn weak(mut self) -> Self {
        self.weak = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    pub fn italics(mut self) -> Self {
        self.italics = true;
        self
    }

    pub fn small(self) -> Self {
        self.text_style(TextStyle::Small)
    }

    pub fn raised(mut self) -> Self {
        self.raised = true;
        self
    }

    pub fn small_raised(self) -> Self {
        self.text_style(TextStyle::Small).raised()
    }

    pub fn background_color(mut self, background_color: impl Into<Color32>) -> Self {
        self.background_color = background_color.into();
        self
    }

    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.text_color = RSome(color.into());
        self
    }
}

impl From<&str> for RichText {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<&String> for RichText {
    fn from(value: &String) -> Self {
        Self::new(value.as_str())
    }
}

impl From<&mut String> for RichText {
    fn from(value: &mut String) -> Self {
        Self::new(value.as_str())
    }
}

impl From<String> for RichText {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<Cow<'_, str>> for RichText {
    fn from(value: Cow<'_, str>) -> Self {
        Self::new(value)
    }
}

impl From<WidgetText> for egui::WidgetText {
    fn from(value: WidgetText) -> Self {
        match value {
            WidgetText::Text(rstring) => egui::WidgetText::Text(rstring.into()),
            WidgetText::RichText(rich_text) => {
                let mut new = egui::RichText::new(rich_text.text)
                    .background_color(rich_text.background_color);
                if let RSome(size) = rich_text.size {
                    new = new.size(size);
                }
                if let RSome(text_style) = rich_text.text_style {
                    new = new.text_style(text_style.into());
                }
                if let RSome(text_color) = rich_text.text_color {
                    new = new.color(text_color);
                }
                if rich_text.code {
                    new = new.code();
                }
                if rich_text.strong {
                    new = new.strong();
                }
                if rich_text.weak {
                    new = new.weak();
                }
                if rich_text.strikethrough {
                    new = new.strikethrough();
                }
                if rich_text.underline {
                    new = new.underline();
                }
                if rich_text.italics {
                    new = new.italics();
                }
                if rich_text.raised {
                    new = new.raised();
                }
                egui::WidgetText::RichText(Arc::new(new))
            }
        }
    }
}
