use std::hash::Hash;

use abi_stable::std_types::ROption::{self, RNone, RSome};
use emath::{Rect, Vec2};
use mint::Vector2;

use crate::{Id, Margin, Vec2b, ui::BunnyUi};
#[cfg(feature = "manager")]
use crate::{
    closure::{PluginClosure, ScrollAreaRowsClosure},
    vtable::ui::ScrollAreaFfiOutput,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    #[inline]
    fn from(value: ScrollBarVisibility) -> Self {
        match value {
            ScrollBarVisibility::AlwaysHidden => Self::AlwaysHidden,
            ScrollBarVisibility::VisibleWhenNeeded => Self::VisibleWhenNeeded,
            ScrollBarVisibility::AlwaysVisible => Self::AlwaysVisible,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[cfg(feature = "manager")]
impl From<ScrollSource> for egui::scroll_area::ScrollSource {
    #[inline]
    fn from(value: ScrollSource) -> Self {
        let ScrollSource {
            scroll_bar,
            drag,
            mouse_wheel,
        } = value;
        Self {
            scroll_bar,
            drag,
            mouse_wheel,
        }
    }
}

#[repr(C)]
pub struct ScrollArea {
    scroll_bar_rect: ROption<Rect>,
    id: ROption<Id>,
    offset_x: ROption<f32>,
    offset_y: ROption<f32>,
    max_size: Vec2,
    wheel_scroll_multiplier: Vec2,
    min_scrolled_size: Vec2,
    content_margin: ROption<Margin>,
    pub(crate) scroll_bar_visibility: ScrollBarVisibility,
    pub(crate) scroll_source: ScrollSource,
    pub(crate) direction_enabled: Vec2b,
    stick_to_end: Vec2b,
    auto_shrink: Vec2b,
    animated: bool,
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
            scroll_bar_rect: RNone,
            id: RNone,
            offset_x: RNone,
            offset_y: RNone,
            wheel_scroll_multiplier: Vec2::splat(1.0),
            stick_to_end: Vec2b::FALSE,
            animated: true,
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
    pub fn scroll_bar_rect(mut self, scroll_bar_rect: Rect) -> Self {
        self.scroll_bar_rect = RSome(scroll_bar_rect);
        self
    }

    /// A source for the Id.
    ///
    /// Note that this gets hashed twice, so the Id in the response won't match an Id created out of the same salt.
    #[inline]
    pub fn id_salt(mut self, id_salt: impl Hash) -> Self {
        self.id = RSome(Id::new(id_salt));
        self
    }

    #[inline]
    pub fn scroll_offset(mut self, offset: impl Into<Vector2<f32>>) -> Self {
        let offset = offset.into();
        self.offset_x = RSome(offset.x);
        self.offset_y = RSome(offset.y);
        self
    }

    #[inline]
    pub fn vertical_scroll_offset(mut self, offset: f32) -> Self {
        self.offset_y = RSome(offset);
        self
    }

    #[inline]
    pub fn horizontal_scroll_offset(mut self, offset: f32) -> Self {
        self.offset_x = RSome(offset);
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
    pub fn wheel_scroll_multiplier(mut self, multiplier: impl Into<Vector2<f32>>) -> Self {
        self.wheel_scroll_multiplier = multiplier.into().into();
        self
    }

    #[inline]
    pub fn auto_shrink(mut self, auto_shrink: impl Into<Vec2b>) -> Self {
        self.auto_shrink = auto_shrink.into();
        self
    }

    #[inline]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    #[inline]
    pub fn content_margin(mut self, margin: impl Into<Margin>) -> Self {
        self.content_margin = RSome(margin.into());
        self
    }

    #[inline]
    pub fn stick_to_right(mut self, stick: bool) -> Self {
        self.stick_to_end.x = stick;
        self
    }

    #[inline]
    pub fn stick_to_bottom(mut self, stick: bool) -> Self {
        self.stick_to_end.y = stick;
        self
    }

    #[inline]
    pub fn show<R>(
        self,
        ui: &mut BunnyUi,
        add_contents: impl FnMut(&mut BunnyUi) -> R,
    ) -> BunnyScrollAreaOutput<R> {
        ui.scroll_area_show(self, add_contents)
    }

    #[inline]
    pub fn show_rows<R>(
        self,
        ui: &mut BunnyUi,
        row_height_sans_spacing: f32,
        total_rows: usize,
        add_contents: impl FnMut(&mut BunnyUi, std::ops::Range<usize>) -> R,
    ) -> BunnyScrollAreaOutput<R> {
        ui.scroll_area_show_rows(self, row_height_sans_spacing, total_rows, add_contents)
    }
}

#[cfg(feature = "manager")]
impl From<ScrollArea> for egui::ScrollArea {
    fn from(value: ScrollArea) -> Self {
        let ScrollArea {
            scroll_bar_rect,
            id,
            offset_x,
            offset_y,
            max_size,
            wheel_scroll_multiplier,
            min_scrolled_size,
            content_margin,
            scroll_bar_visibility,
            scroll_source,
            direction_enabled,
            stick_to_end,
            auto_shrink,
            animated,
        } = value;
        let mut scroll_area = egui::ScrollArea::new(direction_enabled)
            .max_width(max_size.x)
            .max_height(max_size.y)
            .wheel_scroll_multiplier(wheel_scroll_multiplier)
            .min_scrolled_width(min_scrolled_size.x)
            .min_scrolled_height(min_scrolled_size.y)
            .scroll_bar_visibility(scroll_bar_visibility.into())
            .scroll_source(scroll_source.into())
            .stick_to_bottom(stick_to_end.x)
            .stick_to_right(stick_to_end.y)
            .auto_shrink(auto_shrink)
            .animated(animated);
        if let RSome(scroll_bar_rect) = scroll_bar_rect {
            scroll_area = scroll_area.scroll_bar_rect(scroll_bar_rect);
        }
        if let RSome(id) = id {
            scroll_area = scroll_area.id_salt(id);
        }
        if let RSome(offset) = offset_x {
            scroll_area = scroll_area.horizontal_scroll_offset(offset);
        }
        if let RSome(offset) = offset_y {
            scroll_area = scroll_area.vertical_scroll_offset(offset);
        }
        if let RSome(margin) = content_margin {
            scroll_area = scroll_area.content_margin(margin);
        }

        scroll_area
    }
}

#[cfg(feature = "manager")]
impl ScrollArea {
    #[inline]
    pub(crate) fn show_impl(
        self,
        ui: &mut egui::Ui,
        contents: PluginClosure,
    ) -> ScrollAreaFfiOutput {
        let scroll_area: egui::ScrollArea = self.into();
        let output = scroll_area.show(ui, |ui| {
            let mut b = BunnyUi::new(ui);
            contents.call(&mut b);
        });
        ScrollAreaFfiOutput::new(output)
    }

    #[inline]
    pub(crate) fn show_rows_impl(
        self,
        ui: &mut egui::Ui,
        row_height_sans_spacing: f32,
        total_rows: usize,
        contents: ScrollAreaRowsClosure,
    ) -> ScrollAreaFfiOutput {
        let scroll_area: egui::ScrollArea = self.into();
        let output =
            scroll_area.show_rows(ui, row_height_sans_spacing, total_rows, |ui, row_range| {
                let mut b = BunnyUi::new(ui);
                contents.call(&mut b, &[row_range.start, row_range.end]);
            });
        ScrollAreaFfiOutput::new(output)
    }
}

pub struct BunnyScrollAreaOutput<R> {
    pub inner: R,
    pub id: Id,
    pub offset: Vec2,
    pub velocity: Vec2,
    pub content_size: Vec2,
    pub inner_rect: Rect,
}
