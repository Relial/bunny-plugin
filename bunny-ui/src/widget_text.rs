use std::borrow::Cow;

use abi_stable::std_types::{
    RBox,
    ROption::{self, RSome},
    RString,
};
use ecolor::Color32;

use crate::style::TextStyle;

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum WidgetText {
    Text(RString),
    RichText(RBox<RichText>),
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
        Self::RichText(RBox::new(value))
    }
}

#[derive(Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct RichText {
    text: RString,
    size: ROption<f32>,
    text_style: ROption<TextStyle>,
    text_color: ROption<Color32>,
    background_color: Color32,
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

#[cfg(feature = "manager")]
impl From<WidgetText> for egui::WidgetText {
    fn from(value: WidgetText) -> Self {
        match value {
            WidgetText::Text(rstring) => egui::WidgetText::Text(rstring.into()),
            WidgetText::RichText(rich_text) => {
                let rt = RBox::into_inner(rich_text);
                let mut new = egui::RichText::new(rt.text).background_color(rt.background_color);
                if let RSome(size) = rt.size {
                    new = new.size(size);
                }
                if let RSome(text_style) = rt.text_style {
                    new = new.text_style(text_style.into());
                }
                if let RSome(text_color) = rt.text_color {
                    new = new.color(text_color);
                }
                if rt.code {
                    new = new.code();
                }
                if rt.strong {
                    new = new.strong();
                }
                if rt.weak {
                    new = new.weak();
                }
                if rt.strikethrough {
                    new = new.strikethrough();
                }
                if rt.underline {
                    new = new.underline();
                }
                if rt.italics {
                    new = new.italics();
                }
                if rt.raised {
                    new = new.raised();
                }
                egui::WidgetText::RichText(std::sync::Arc::new(new))
            }
        }
    }
}
