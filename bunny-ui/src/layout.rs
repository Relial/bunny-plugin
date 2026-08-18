use crate::{align::Align, direction::Direction};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Layout {
    pub main_dir: Direction,
    pub main_align: Align,
    pub cross_align: Align,
    pub cross_justify: bool,
    pub main_wrap: bool,
    pub main_justify: bool,
}

impl Default for Layout {
    fn default() -> Self {
        Self::top_down(Align::LEFT)
    }
}

impl Layout {
    #[inline(always)]
    pub fn left_to_right(valign: Align) -> Self {
        Self {
            main_dir: Direction::LeftToRight,
            main_wrap: false,
            main_align: Align::Center,
            main_justify: false,
            cross_align: valign,
            cross_justify: false,
        }
    }

    #[inline(always)]
    pub fn right_to_left(valign: Align) -> Self {
        Self {
            main_dir: Direction::RightToLeft,
            main_wrap: false,
            main_align: Align::Center,
            main_justify: false,
            cross_align: valign,
            cross_justify: false,
        }
    }

    #[inline(always)]
    pub fn top_down(halign: Align) -> Self {
        Self {
            main_dir: Direction::TopDown,
            main_wrap: false,
            main_align: Align::Center,
            main_justify: false,
            cross_align: halign,
            cross_justify: false,
        }
    }

    #[inline(always)]
    pub fn top_down_justified(halign: Align) -> Self {
        Self::top_down(halign).with_cross_justify(true)
    }

    #[inline(always)]
    pub fn bottom_up(halign: Align) -> Self {
        Self {
            main_dir: Direction::BottomUp,
            main_wrap: false,
            main_align: Align::Center,
            main_justify: false,
            cross_align: halign,
            cross_justify: false,
        }
    }

    #[inline(always)]
    pub fn from_main_dir_and_cross_align(main_dir: Direction, cross_align: Align) -> Self {
        Self {
            main_dir,
            main_wrap: false,
            main_align: Align::Center,
            main_justify: false,
            cross_align,
            cross_justify: false,
        }
    }

    #[inline(always)]
    pub fn centered_and_justified(main_dir: Direction) -> Self {
        Self {
            main_dir,
            main_wrap: false,
            main_align: Align::Center,
            main_justify: true,
            cross_align: Align::Center,
            cross_justify: true,
        }
    }

    #[inline(always)]
    pub fn with_main_wrap(self, main_wrap: bool) -> Self {
        Self { main_wrap, ..self }
    }

    #[inline(always)]
    pub fn with_main_align(self, main_align: Align) -> Self {
        Self { main_align, ..self }
    }

    #[inline(always)]
    pub fn with_cross_align(self, cross_align: Align) -> Self {
        Self {
            cross_align,
            ..self
        }
    }

    #[inline(always)]
    pub fn with_main_justify(self, main_justify: bool) -> Self {
        Self {
            main_justify,
            ..self
        }
    }

    #[inline(always)]
    pub fn with_cross_justify(self, cross_justify: bool) -> Self {
        Self {
            cross_justify,
            ..self
        }
    }
}

impl Layout {
    #[inline(always)]
    pub fn main_dir(&self) -> Direction {
        self.main_dir
    }

    #[inline(always)]
    pub fn main_wrap(&self) -> bool {
        self.main_wrap
    }

    #[inline(always)]
    pub fn cross_align(&self) -> Align {
        self.cross_align
    }

    #[inline(always)]
    pub fn cross_justify(&self) -> bool {
        self.cross_justify
    }

    #[inline(always)]
    pub fn is_horizontal(&self) -> bool {
        self.main_dir().is_horizontal()
    }

    #[inline(always)]
    pub fn is_vertical(&self) -> bool {
        self.main_dir().is_vertical()
    }

    pub fn prefer_right_to_left(&self) -> bool {
        self.main_dir == Direction::RightToLeft
            || self.main_dir.is_vertical() && self.cross_align == Align::Max
    }
}

impl From<Layout> for egui::Layout {
    fn from(value: Layout) -> Self {
        Self {
            main_dir: value.main_dir.into(),
            main_wrap: value.main_wrap,
            main_align: value.main_align.into(),
            main_justify: value.main_justify,
            cross_align: value.cross_align.into(),
            cross_justify: value.cross_justify,
        }
    }
}
