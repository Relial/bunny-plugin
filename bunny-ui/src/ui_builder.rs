use abi_stable::std_types::ROption::{self, RSome};
use egui::{Rect, Sense};

use crate::layout::Layout;

#[derive(Default)]
#[repr(C)]
pub struct UiBuilder {
    pub layout: ROption<Layout>,
    pub max_rect: ROption<Rect>,
    pub sense: ROption<Sense>,
    pub disabled: bool,
    pub invisible: bool,
    pub global_scope: bool,
}

impl UiBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn global_scope(mut self, global_scope: bool) -> Self {
        self.global_scope = global_scope;
        self
    }

    #[inline]
    pub fn max_rect(mut self, max_rect: Rect) -> Self {
        self.max_rect = RSome(max_rect);
        self
    }

    #[inline]
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = RSome(layout);
        self
    }

    #[inline]
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    #[inline]
    pub fn invisible(mut self) -> Self {
        self.invisible = true;
        self.disabled = true;
        self
    }

    #[inline]
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = RSome(sense);
        self
    }
}

impl From<UiBuilder> for egui::UiBuilder {
    fn from(value: UiBuilder) -> Self {
        Self {
            global_scope: value.global_scope,
            max_rect: value.max_rect.into(),
            layout: value.layout.map(|l| l.into()).into(),
            disabled: value.disabled,
            invisible: value.invisible,
            sense: value.sense.into(),
            ..Default::default()
        }
    }
}
