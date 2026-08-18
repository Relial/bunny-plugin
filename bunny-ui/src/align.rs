use emath::{Pos2, Rangef, Rect, Vec2, fast_midpoint, pos2, vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
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

#[cfg(feature = "manager")]
impl From<Align> for egui::Align {
    fn from(value: Align) -> Self {
        match value {
            Align::Min => Self::Min,
            Align::Center => Self::Center,
            Align::Max => Self::Max,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::Align> for Align {
    fn from(value: egui::Align) -> Self {
        match value {
            egui::Align::Min => Self::Min,
            egui::Align::Center => Self::Center,
            egui::Align::Max => Self::Max,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Align2(pub [Align; 2]);

impl Align2 {
    pub const LEFT_BOTTOM: Self = Self([Align::Min, Align::Max]);
    pub const LEFT_CENTER: Self = Self([Align::Min, Align::Center]);
    pub const LEFT_TOP: Self = Self([Align::Min, Align::Min]);
    pub const CENTER_BOTTOM: Self = Self([Align::Center, Align::Max]);
    pub const CENTER_CENTER: Self = Self([Align::Center, Align::Center]);
    pub const CENTER_TOP: Self = Self([Align::Center, Align::Min]);
    pub const RIGHT_BOTTOM: Self = Self([Align::Max, Align::Max]);
    pub const RIGHT_CENTER: Self = Self([Align::Max, Align::Center]);
    pub const RIGHT_TOP: Self = Self([Align::Max, Align::Min]);
}

impl Align2 {
    #[inline(always)]
    pub fn x(self) -> Align {
        self.0[0]
    }

    #[inline(always)]
    pub fn y(self) -> Align {
        self.0[1]
    }

    pub fn to_sign(self) -> Vec2 {
        vec2(self.x().to_sign(), self.y().to_sign())
    }

    pub fn flip_x(self) -> Self {
        Self([self.x().flip(), self.y()])
    }

    pub fn flip_y(self) -> Self {
        Self([self.x(), self.y().flip()])
    }

    pub fn flip(self) -> Self {
        Self([self.x().flip(), self.y().flip()])
    }

    pub fn anchor_rect(self, rect: Rect) -> Rect {
        let x = match self.x() {
            Align::Min => rect.left(),
            Align::Center => rect.left() - 0.5 * rect.width(),
            Align::Max => rect.left() - rect.width(),
        };
        let y = match self.y() {
            Align::Min => rect.top(),
            Align::Center => rect.top() - 0.5 * rect.height(),
            Align::Max => rect.top() - rect.height(),
        };
        Rect::from_min_size(pos2(x, y), rect.size())
    }

    pub fn anchor_size(self, pos: Pos2, size: Vec2) -> Rect {
        let x = match self.x() {
            Align::Min => pos.x,
            Align::Center => pos.x - 0.5 * size.x,
            Align::Max => pos.x - size.x,
        };
        let y = match self.y() {
            Align::Min => pos.y,
            Align::Center => pos.y - 0.5 * size.y,
            Align::Max => pos.y - size.y,
        };
        Rect::from_min_size(pos2(x, y), size)
    }

    pub fn align_size_within_rect(self, size: Vec2, frame: Rect) -> Rect {
        let x_range = self.x().align_size_within_range(size.x, frame.x_range());
        let y_range = self.y().align_size_within_range(size.y, frame.y_range());
        Rect::from_x_y_ranges(x_range, y_range)
    }

    pub fn pos_in_rect(self, frame: &Rect) -> Pos2 {
        let x = match self.x() {
            Align::Min => frame.left(),
            Align::Center => frame.center().x,
            Align::Max => frame.right(),
        };
        let y = match self.y() {
            Align::Min => frame.top(),
            Align::Center => frame.center().y,
            Align::Max => frame.bottom(),
        };
        pos2(x, y)
    }
}

#[cfg(feature = "manager")]
impl From<Align2> for egui::Align2 {
    fn from(value: Align2) -> Self {
        Self([value.x().into(), value.y().into()])
    }
}
