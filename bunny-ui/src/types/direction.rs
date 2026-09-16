#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
    TopDown,
    BottomUp,
}

impl Direction {
    #[inline(always)]
    pub fn is_horizontal(self) -> bool {
        match self {
            Self::LeftToRight | Self::RightToLeft => true,
            Self::TopDown | Self::BottomUp => false,
        }
    }

    #[inline(always)]
    pub fn is_vertical(&self) -> bool {
        match self {
            Self::LeftToRight | Self::RightToLeft => false,
            Self::TopDown | Self::BottomUp => true,
        }
    }
}

#[cfg(feature = "manager")]
impl From<Direction> for egui::Direction {
    #[inline]
    fn from(value: Direction) -> Self {
        match value {
            Direction::LeftToRight => Self::LeftToRight,
            Direction::RightToLeft => Self::RightToLeft,
            Direction::TopDown => Self::TopDown,
            Direction::BottomUp => Self::BottomUp,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::Direction> for Direction {
    #[inline]
    fn from(value: egui::Direction) -> Self {
        match value {
            egui::Direction::LeftToRight => Self::LeftToRight,
            egui::Direction::RightToLeft => Self::RightToLeft,
            egui::Direction::TopDown => Self::TopDown,
            egui::Direction::BottomUp => Self::BottomUp,
        }
    }
}
