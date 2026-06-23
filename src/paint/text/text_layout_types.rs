use std::{collections::BTreeMap, sync::Arc};

use abi_stable::{
    rvec,
    std_types::{
        ROption::{self, RNone, RSome},
        RString, RVec, Tuple2,
    },
};
use egui::{
    Color32,
    epaint::text::{IntoTag, Tag},
};

use crate::{
    align::Align,
    paint::{stroke::Stroke, text::fonts::FontId},
};

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutJob {
    pub text: RString,
    pub sections: RVec<LayoutSection>,
    pub wrap: TextWrapping,
    pub first_row_min_height: f32,
    pub break_on_newline: bool,
    pub halign: Align,
    pub justify: bool,
    pub round_output_to_gui: bool,
}

impl Default for LayoutJob {
    #[inline]
    fn default() -> Self {
        Self {
            text: Default::default(),
            sections: Default::default(),
            wrap: Default::default(),
            first_row_min_height: 0.0,
            break_on_newline: true,
            halign: Align::LEFT,
            justify: false,
            round_output_to_gui: true,
        }
    }
}

impl LayoutJob {
    #[inline]
    pub fn simple(
        text: impl Into<RString>,
        font_id: FontId,
        color: Color32,
        wrap_width: f32,
    ) -> Self {
        let text = text.into();
        Self {
            sections: rvec![LayoutSection {
                leading_space: 0.0,
                byte_range: [0, text.len()],
                format: TextFormat::simple(font_id, color),
            }],
            text,
            wrap: TextWrapping {
                max_width: wrap_width,
                ..Default::default()
            },
            break_on_newline: true,
            ..Default::default()
        }
    }

    #[inline]
    pub fn simple_format(text: impl Into<RString>, format: TextFormat) -> Self {
        let text = text.into();
        Self {
            sections: rvec![LayoutSection {
                leading_space: 0.0,
                byte_range: [0, text.len()],
                format
            }],
            text,
            break_on_newline: true,
            ..Default::default()
        }
    }

    #[inline]
    pub fn simple_singleline(text: impl Into<RString>, font_id: FontId, color: Color32) -> Self {
        let text = text.into();
        Self {
            sections: rvec![LayoutSection {
                leading_space: 0.0,
                byte_range: [0, text.len()],
                format: TextFormat::simple(font_id, color)
            }],
            text,
            wrap: Default::default(),
            break_on_newline: false,
            ..Default::default()
        }
    }

    #[inline]
    pub fn single_section(text: impl Into<RString>, format: TextFormat) -> Self {
        let text = text.into();
        Self {
            sections: rvec![LayoutSection {
                leading_space: 0.0,
                byte_range: [0, text.len()],
                format
            }],
            text,
            wrap: Default::default(),
            break_on_newline: true,
            ..Default::default()
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.sections.is_empty()
    }

    pub fn append(&mut self, text: &str, leading_space: f32, format: TextFormat) {
        let start = self.text.len();
        self.text.push_str(text);
        let byte_range = [start, self.text.len()];
        self.sections.push(LayoutSection {
            leading_space,
            byte_range,
            format,
        });
    }

    pub fn effective_wrap_width(&self) -> f32 {
        if self.round_output_to_gui {
            self.wrap.max_width + 0.5
        } else {
            self.wrap.max_width
        }
    }
}

impl LayoutJob {
    pub(crate) fn convert_to_egui(
        self,
        font_data: &BTreeMap<String, Arc<egui::FontData>>,
    ) -> egui::epaint::text::LayoutJob {
        let LayoutJob {
            text,
            sections,
            wrap,
            first_row_min_height,
            break_on_newline,
            halign,
            justify,
            round_output_to_gui,
        } = self;
        egui::epaint::text::LayoutJob {
            text: text.into(),
            sections: sections
                .into_iter()
                .map(|section| section.convert_to_egui(font_data))
                .collect(),
            wrap: wrap.into(),
            first_row_min_height,
            break_on_newline,
            halign: halign.into(),
            justify,
            round_output_to_gui,
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutSection {
    pub leading_space: f32,
    pub byte_range: [usize; 2],
    pub format: TextFormat,
}

impl LayoutSection {
    pub(crate) fn convert_to_egui(
        self,
        font_data: &BTreeMap<String, Arc<egui::FontData>>,
    ) -> egui::epaint::text::LayoutSection {
        let LayoutSection {
            leading_space,
            byte_range,
            format,
        } = self;
        egui::epaint::text::LayoutSection {
            leading_space,
            byte_range: byte_range[0]..byte_range[1],
            format: format.convert_to_egui(font_data),
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct VariationCoords(RVec<Tuple2<Tag, f32>>);

impl VariationCoords {
    pub fn new<T: IntoTag>(values: impl IntoIterator<Item = (T, f32)>) -> Self {
        Self(
            values
                .into_iter()
                .map(|(t, c)| Tuple2(t.into_tag(), c))
                .collect(),
        )
    }

    #[inline(always)]
    pub fn push(&mut self, tag: impl IntoTag, coord: f32) {
        self.0.push(Tuple2(tag.into_tag(), coord));
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl From<VariationCoords> for egui::epaint::text::VariationCoords {
    fn from(value: VariationCoords) -> Self {
        Self::new(value.0.into_iter().map(|t| t.into()))
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct TextFormat {
    pub font_id: FontId,
    pub extra_letter_spacing: f32,
    pub line_height: ROption<f32>,
    pub color: Color32,
    pub background: Color32,
    pub expand_bg: f32,
    pub coords: VariationCoords,
    pub italics: bool,
    pub underline: Stroke,
    pub strikethrough: Stroke,
    pub valign: Align,
}

impl Default for TextFormat {
    fn default() -> Self {
        Self {
            font_id: FontId::default(),
            extra_letter_spacing: 0.0,
            line_height: RNone,
            color: Color32::GRAY,
            background: Color32::TRANSPARENT,
            expand_bg: 1.0,
            coords: VariationCoords::default(),
            italics: false,
            underline: Stroke::NONE,
            strikethrough: Stroke::NONE,
            valign: Align::BOTTOM,
        }
    }
}

impl TextFormat {
    #[inline]
    pub fn simple(font_id: FontId, color: Color32) -> Self {
        Self {
            font_id,
            color,
            ..Default::default()
        }
    }
}

impl TextFormat {
    pub(crate) fn convert_to_egui(
        self,
        font_data: &BTreeMap<String, Arc<egui::FontData>>,
    ) -> egui::epaint::text::TextFormat {
        let TextFormat {
            font_id,
            extra_letter_spacing,
            line_height,
            color,
            background,
            expand_bg,
            coords,
            italics,
            underline,
            strikethrough,
            valign,
        } = self;
        egui::epaint::text::TextFormat {
            font_id: font_id.convert_to_egui(font_data),
            extra_letter_spacing,
            line_height: line_height.into(),
            color,
            background,
            expand_bg,
            coords: coords.into(),
            italics,
            underline: underline.into(),
            strikethrough: strikethrough.into(),
            valign: valign.into(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextWrapMode {
    Extend,
    Wrap,
    Truncate,
}

impl From<TextWrapMode> for egui::TextWrapMode {
    fn from(value: TextWrapMode) -> Self {
        match value {
            TextWrapMode::Extend => Self::Extend,
            TextWrapMode::Wrap => Self::Wrap,
            TextWrapMode::Truncate => Self::Truncate,
        }
    }
}

impl From<egui::TextWrapMode> for TextWrapMode {
    fn from(value: egui::TextWrapMode) -> Self {
        match value {
            egui::TextWrapMode::Extend => Self::Extend,
            egui::TextWrapMode::Wrap => Self::Wrap,
            egui::TextWrapMode::Truncate => Self::Truncate,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextWrapping {
    pub max_width: f32,
    pub max_rows: usize,
    pub break_anywhere: bool,
    pub overflow_character: ROption<char>,
}

impl Default for TextWrapping {
    fn default() -> Self {
        Self {
            max_width: f32::INFINITY,
            max_rows: usize::MAX,
            break_anywhere: false,
            overflow_character: RSome('…'),
        }
    }
}

impl TextWrapping {
    pub fn from_wrap_mode_and_width(mode: TextWrapMode, max_width: f32) -> Self {
        match mode {
            TextWrapMode::Extend => Self::no_max_width(),
            TextWrapMode::Wrap => Self::wrap_at_width(max_width),
            TextWrapMode::Truncate => Self::truncate_at_width(max_width),
        }
    }

    pub fn no_max_width() -> Self {
        Self {
            max_width: f32::INFINITY,
            ..Default::default()
        }
    }

    pub fn wrap_at_width(max_width: f32) -> Self {
        Self {
            max_width,
            ..Default::default()
        }
    }

    pub fn truncate_at_width(max_width: f32) -> Self {
        Self {
            max_width,
            max_rows: 1,
            break_anywhere: true,
            ..Default::default()
        }
    }
}

impl From<TextWrapping> for egui::epaint::text::TextWrapping {
    fn from(value: TextWrapping) -> Self {
        let TextWrapping {
            max_width,
            max_rows,
            break_anywhere,
            overflow_character,
        } = value;
        Self {
            max_width,
            max_rows,
            break_anywhere,
            overflow_character: overflow_character.into(),
        }
    }
}
