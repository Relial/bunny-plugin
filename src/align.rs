use egui::{Pos2, Rect, Vec2, pos2};

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

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
}

impl From<Align2> for egui::Align2 {
    fn from(value: Align2) -> Self {
        Self([value.x().into(), value.y().into()])
    }
}
