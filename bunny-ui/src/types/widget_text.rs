use std::borrow::Cow;

use abi_stable::std_types::{
    RBox, RCowStr,
    ROption::{self, RSome},
    RStr,
};
use ecolor::Color32;

use crate::{
    TextStyle,
    paint::{FontFamily, FontId},
};

#[derive(Clone)]
#[repr(C)]
pub enum WidgetText<'a> {
    Text(RCowStr<'a>),
    RichText(RBox<RichText<'a>>),
}

impl<'a> WidgetText<'a> {
    #[inline]
    pub fn text(&self) -> &str {
        match self {
            WidgetText::Text(rcow) => rcow,
            WidgetText::RichText(rich_text) => rich_text.text(),
        }
    }
}

impl Default for WidgetText<'_> {
    fn default() -> Self {
        Self::Text(RCowStr::Borrowed(RStr::empty()))
    }
}

impl<'a> From<&'a str> for WidgetText<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        Self::Text(value.into())
    }
}

impl<'a> From<&'a String> for WidgetText<'a> {
    #[inline]
    fn from(value: &'a String) -> Self {
        Self::Text(value.into())
    }
}

impl From<String> for WidgetText<'_> {
    #[inline]
    fn from(value: String) -> Self {
        Self::Text(value.into())
    }
}

impl<'a> From<Cow<'a, str>> for WidgetText<'a> {
    #[inline]
    fn from(value: Cow<'a, str>) -> Self {
        Self::Text(value.into())
    }
}

impl<'a> From<RichText<'a>> for WidgetText<'a> {
    #[inline]
    fn from(value: RichText<'a>) -> Self {
        Self::RichText(RBox::new(value))
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct RichText<'a> {
    family: ROption<FontFamily>,
    text: RCowStr<'a>,
    size: ROption<f32>,
    line_height: ROption<f32>,
    text_style: ROption<TextStyle>,
    text_color: ROption<Color32>,
    background_color: Color32,
    extra_letter_spacing: f32,
    code: bool,
    strong: bool,
    weak: bool,
    strikethrough: bool,
    underline: bool,
    italics: bool,
    raised: bool,
}

impl Default for RichText<'_> {
    fn default() -> Self {
        Self {
            family: Default::default(),
            text: RCowStr::Borrowed(RStr::empty()),
            size: Default::default(),
            line_height: Default::default(),
            text_style: Default::default(),
            text_color: Default::default(),
            background_color: Default::default(),
            extra_letter_spacing: Default::default(),
            code: Default::default(),
            strong: Default::default(),
            weak: Default::default(),
            strikethrough: Default::default(),
            underline: Default::default(),
            italics: Default::default(),
            raised: Default::default(),
        }
    }
}

impl<'a> RichText<'a> {
    pub fn new(text: impl Into<RCowStr<'a>>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[inline]
    pub fn size(mut self, size: f32) -> Self {
        self.size = RSome(size);
        self
    }

    #[inline]
    pub fn extra_letter_spacing(mut self, extra_letter_spacing: f32) -> Self {
        self.extra_letter_spacing = extra_letter_spacing;
        self
    }

    #[inline]
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = RSome(line_height);
        self
    }

    #[inline]
    pub fn family(mut self, family: FontFamily) -> Self {
        self.family = RSome(family);
        self
    }

    #[inline]
    pub fn font(mut self, font_id: FontId) -> Self {
        let FontId { family, size } = font_id;
        self.size = RSome(size);
        self.family = RSome(family);
        self
    }

    #[inline]
    pub fn text_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = RSome(text_style);
        self
    }

    #[inline]
    pub fn heading(self) -> Self {
        self.text_style(TextStyle::Heading)
    }

    #[inline]
    pub fn monospace(self) -> Self {
        self.text_style(TextStyle::Monospace)
    }

    #[inline]
    pub fn code(mut self) -> Self {
        self.code = true;
        self.text_style(TextStyle::Monospace)
    }

    #[inline]
    pub fn strong(mut self) -> Self {
        self.strong = true;
        self
    }

    #[inline]
    pub fn weak(mut self) -> Self {
        self.weak = true;
        self
    }

    #[inline]
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    #[inline]
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    #[inline]
    pub fn italics(mut self) -> Self {
        self.italics = true;
        self
    }

    #[inline]
    pub fn small(self) -> Self {
        self.text_style(TextStyle::Small)
    }

    #[inline]
    pub fn raised(mut self) -> Self {
        self.raised = true;
        self
    }

    #[inline]
    pub fn small_raised(self) -> Self {
        self.text_style(TextStyle::Small).raised()
    }

    #[inline]
    pub fn background_color(mut self, background_color: impl Into<Color32>) -> Self {
        self.background_color = background_color.into();
        self
    }

    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.text_color = RSome(color.into());
        self
    }
}

impl<'a> From<&'a str> for RichText<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        Self::new(value)
    }
}

impl<'a> From<&'a String> for RichText<'a> {
    #[inline]
    fn from(value: &'a String) -> Self {
        Self::new(value)
    }
}

impl<'a> From<&'a mut String> for RichText<'a> {
    #[inline]
    fn from(value: &'a mut String) -> Self {
        Self::new(value.as_str())
    }
}

impl From<String> for RichText<'_> {
    #[inline]
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl<'a> From<Cow<'a, str>> for RichText<'a> {
    #[inline]
    fn from(value: Cow<'a, str>) -> Self {
        Self::new(value)
    }
}

#[cfg(feature = "manager")]
impl From<RichText<'_>> for egui::RichText {
    fn from(value: RichText) -> Self {
        let RichText {
            family,
            text,
            size,
            line_height,
            text_style,
            text_color,
            background_color,
            extra_letter_spacing,
            code,
            strong,
            weak,
            strikethrough,
            underline,
            italics,
            raised,
        } = value;
        let mut rt = Self::new(text.to_string())
            .extra_letter_spacing(extra_letter_spacing)
            .line_height(line_height.into_option())
            .background_color(background_color);
        if let RSome(family) = family {
            rt = rt.family(family.into());
        }
        if let RSome(size) = size {
            rt = rt.size(size);
        }
        if let RSome(text_style) = text_style {
            rt = rt.text_style(text_style.into());
        }
        if let RSome(text_color) = text_color {
            rt = rt.color(text_color);
        }
        if code {
            rt = rt.code();
        }
        if strong {
            rt = rt.strong();
        }
        if weak {
            rt = rt.weak();
        }
        if strikethrough {
            rt = rt.strikethrough();
        }
        if underline {
            rt = rt.underline();
        }
        if italics {
            rt = rt.italics();
        }
        if raised {
            rt = rt.raised();
        }
        rt
    }
}

#[cfg(feature = "manager")]
impl From<WidgetText<'_>> for egui::WidgetText {
    fn from(value: WidgetText) -> Self {
        match value {
            WidgetText::Text(rcow) => Self::Text(rcow.to_string()),
            WidgetText::RichText(rich_text) => {
                let rt = RBox::into_inner(rich_text);
                Self::RichText(std::sync::Arc::new(rt.into()))
            }
        }
    }
}
