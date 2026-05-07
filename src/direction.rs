#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
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

impl From<Direction> for egui::Direction {
    fn from(value: Direction) -> Self {
        match value {
            Direction::LeftToRight => Self::LeftToRight,
            Direction::RightToLeft => Self::RightToLeft,
            Direction::TopDown => Self::TopDown,
            Direction::BottomUp => Self::BottomUp,
        }
    }
}
