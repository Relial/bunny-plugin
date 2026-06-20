use egui::{Rangef, emath::fast_midpoint};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    Min,
    Center,
    Max,
}

impl Align {
    pub const LEFT: Align = Self::Min;
    pub const RIGHT: Align = Self::Max;
    pub const TOP: Align = Self::Min;
    pub const BOTTOM: Align = Self::Max;

    #[inline(always)]
    pub fn to_factor(self) -> f32 {
        match self {
            Align::Min => 0.0,
            Align::Center => 0.5,
            Align::Max => 1.0,
        }
    }

    #[inline(always)]
    pub fn to_sign(self) -> f32 {
        match self {
            Align::Min => -1.0,
            Align::Center => 0.0,
            Align::Max => 1.0,
        }
    }

    pub fn flip(self) -> Self {
        match self {
            Align::Min => Self::Max,
            Align::Center => Self::Center,
            Align::Max => Self::Min,
        }
    }

    #[inline]
    pub fn align_size_within_range(self, size: f32, range: impl Into<Rangef>) -> Rangef {
        let range = range.into();
        let Rangef { min, max } = range;

        if max - min == f32::INFINITY && size == f32::INFINITY {
            return range;
        }

        match self {
            Align::Min => Rangef::new(min, min + size),
            Align::Center => {
                if size == f32::INFINITY {
                    Rangef::new(f32::NEG_INFINITY, f32::INFINITY)
                } else {
                    let left = fast_midpoint(min, max) - size / 2.0;
                    Rangef::new(left, left + size)
                }
            }
            Align::Max => Rangef::new(max - size, max),
        }
    }
}

impl From<Align> for egui::Align {
    fn from(value: Align) -> Self {
        match value {
            Align::Min => Self::Min,
            Align::Center => Self::Center,
            Align::Max => Self::Max,
        }
    }
}

impl From<egui::Align> for Align {
    fn from(value: egui::Align) -> Self {
        match value {
            egui::Align::Min => Self::Min,
            egui::Align::Center => Self::Center,
            egui::Align::Max => Self::Max,
        }
    }
}
