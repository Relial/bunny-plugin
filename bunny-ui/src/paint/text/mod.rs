pub mod fonts;
pub mod text_layout_types;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct TextOptions {
    pub max_texture_side: usize,
    pub alpha_from_coverage: AlphaFromCoverage,
    pub font_hinting: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum AlphaFromCoverage {
    Linear,
    Gamma(f32),
    #[default]
    TwoCoverageMinusCoverageSq,
}

#[cfg(feature = "manager")]
impl From<TextOptions> for egui::epaint::text::TextOptions {
    #[inline]
    fn from(value: TextOptions) -> Self {
        Self {
            max_texture_side: value.max_texture_side,
            alpha_from_coverage: value.alpha_from_coverage.into(),
            font_hinting: value.font_hinting,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::epaint::text::TextOptions> for TextOptions {
    #[inline]
    fn from(value: egui::epaint::text::TextOptions) -> Self {
        Self {
            max_texture_side: value.max_texture_side,
            alpha_from_coverage: value.alpha_from_coverage.into(),
            font_hinting: value.font_hinting,
        }
    }
}

#[cfg(feature = "manager")]
impl From<AlphaFromCoverage> for egui::epaint::image::AlphaFromCoverage {
    #[inline]
    fn from(value: AlphaFromCoverage) -> Self {
        match value {
            AlphaFromCoverage::Linear => Self::Linear,
            AlphaFromCoverage::Gamma(gamma) => Self::Gamma(gamma),
            AlphaFromCoverage::TwoCoverageMinusCoverageSq => Self::TwoCoverageMinusCoverageSq,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::epaint::image::AlphaFromCoverage> for AlphaFromCoverage {
    #[inline]
    fn from(value: egui::epaint::image::AlphaFromCoverage) -> Self {
        match value {
            egui::epaint::AlphaFromCoverage::Linear => Self::Linear,
            egui::epaint::AlphaFromCoverage::Gamma(gamma) => Self::Gamma(gamma),
            egui::epaint::AlphaFromCoverage::TwoCoverageMinusCoverageSq => {
                Self::TwoCoverageMinusCoverageSq
            }
        }
    }
}
