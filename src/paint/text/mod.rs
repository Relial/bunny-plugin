pub mod fonts;
pub mod text_layout_types;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextOptions {
    pub max_texture_side: usize,
    pub alpha_from_coverage: AlphaFromCoverage,
    pub font_hinting: bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AlphaFromCoverage {
    Linear,
    Gamma(f32),
    #[default]
    TwoCoverageMinusCoverageSq,
}

impl From<TextOptions> for egui::epaint::text::TextOptions {
    fn from(value: TextOptions) -> Self {
        Self {
            max_texture_side: value.max_texture_side,
            alpha_from_coverage: value.alpha_from_coverage.into(),
            font_hinting: value.font_hinting,
        }
    }
}

impl From<egui::epaint::text::TextOptions> for TextOptions {
    fn from(value: egui::epaint::text::TextOptions) -> Self {
        Self {
            max_texture_side: value.max_texture_side,
            alpha_from_coverage: value.alpha_from_coverage.into(),
            font_hinting: value.font_hinting,
        }
    }
}

impl From<AlphaFromCoverage> for egui::epaint::image::AlphaFromCoverage {
    fn from(value: AlphaFromCoverage) -> Self {
        match value {
            AlphaFromCoverage::Linear => Self::Linear,
            AlphaFromCoverage::Gamma(gamma) => Self::Gamma(gamma),
            AlphaFromCoverage::TwoCoverageMinusCoverageSq => Self::TwoCoverageMinusCoverageSq,
        }
    }
}

impl From<egui::epaint::image::AlphaFromCoverage> for AlphaFromCoverage {
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
