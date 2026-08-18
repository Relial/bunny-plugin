use abi_stable::std_types::ROption::{self, RNone};
use emath::Vec2;

use crate::{margin::Margin, vec2b::Vec2b};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum ScrollBarVisibility {
    AlwaysHidden,
    VisibleWhenNeeded,
    AlwaysVisible,
}

impl Default for ScrollBarVisibility {
    #[inline]
    fn default() -> Self {
        Self::VisibleWhenNeeded
    }
}

#[cfg(feature = "manager")]
impl From<ScrollBarVisibility> for egui::scroll_area::ScrollBarVisibility {
    fn from(value: ScrollBarVisibility) -> Self {
        match value {
            ScrollBarVisibility::AlwaysHidden => Self::AlwaysHidden,
            ScrollBarVisibility::VisibleWhenNeeded => Self::VisibleWhenNeeded,
            ScrollBarVisibility::AlwaysVisible => Self::AlwaysVisible,
        }
    }
}

#[repr(C)]
pub struct ScrollSource {
    pub scroll_bar: bool,
    pub drag: bool,
    pub mouse_wheel: bool,
}

impl Default for ScrollSource {
    fn default() -> Self {
        Self::ALL
    }
}

impl ScrollSource {
    pub const NONE: Self = Self {
        scroll_bar: false,
        drag: false,
        mouse_wheel: false,
    };
    pub const ALL: Self = Self {
        scroll_bar: true,
        drag: true,
        mouse_wheel: true,
    };
    pub const SCROLL_BAR: Self = Self {
        scroll_bar: true,
        drag: false,
        mouse_wheel: false,
    };
    pub const DRAG: Self = Self {
        scroll_bar: false,
        drag: true,
        mouse_wheel: false,
    };
    pub const MOUSE_WHEEL: Self = Self {
        scroll_bar: false,
        drag: false,
        mouse_wheel: true,
    };
}

#[repr(C)]
pub struct ScrollArea {
    pub(crate) max_size: Vec2,
    pub(crate) min_scrolled_size: Vec2,
    pub(crate) content_margin: ROption<Margin>,
    pub(crate) scroll_bar_visibility: ScrollBarVisibility,
    pub(crate) scroll_source: ScrollSource,
    pub(crate) direction_enabled: Vec2b,
    pub(crate) auto_shrink: Vec2b,
}

impl ScrollArea {
    #[inline]
    pub fn horizontal() -> Self {
        Self::new([true, false])
    }

    #[inline]
    pub fn vertical() -> Self {
        Self::new([false, true])
    }

    #[inline]
    pub fn both() -> Self {
        Self::new([true, true])
    }

    #[inline]
    pub fn neither() -> Self {
        Self::new([false, false])
    }

    pub fn new(direction_enabled: impl Into<Vec2b>) -> Self {
        Self {
            direction_enabled: direction_enabled.into(),
            auto_shrink: Vec2b::TRUE,
            max_size: Vec2::INFINITY,
            min_scrolled_size: Vec2::splat(64.0),
            scroll_bar_visibility: Default::default(),
            scroll_source: ScrollSource::default(),
            content_margin: RNone,
        }
    }

    #[inline]
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_size.x = max_width;
        self
    }

    #[inline]
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_size.y = max_height;
        self
    }

    #[inline]
    pub fn min_scrolled_width(mut self, min_scrolled_width: f32) -> Self {
        self.min_scrolled_size.x = min_scrolled_width;
        self
    }

    #[inline]
    pub fn min_scrolled_height(mut self, min_scrolled_height: f32) -> Self {
        self.min_scrolled_size.y = min_scrolled_height;
        self
    }

    #[inline]
    pub fn scroll_bar_visibility(mut self, scroll_bar_visibility: ScrollBarVisibility) -> Self {
        self.scroll_bar_visibility = scroll_bar_visibility;
        self
    }

    #[inline]
    pub fn hscroll(mut self, hscroll: bool) -> Self {
        self.direction_enabled[0] = hscroll;
        self
    }

    #[inline]
    pub fn vscroll(mut self, vscroll: bool) -> Self {
        self.direction_enabled[1] = vscroll;
        self
    }

    #[inline]
    pub fn scroll(mut self, direction_enabled: impl Into<Vec2b>) -> Self {
        self.direction_enabled = direction_enabled.into();
        self
    }

    #[inline]
    pub fn scroll_source(mut self, scroll_source: ScrollSource) -> Self {
        self.scroll_source = scroll_source;
        self
    }

    #[inline]
    pub fn auto_shrink(mut self, auto_shrink: impl Into<Vec2b>) -> Self {
        self.auto_shrink = auto_shrink.into();
        self
    }
}
