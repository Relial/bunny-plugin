use abi_stable::std_types::{
    ROption::{self, RNone, RSome},
    Tuple2,
};
use egui::{Pos2, Rect, Vec2};

use crate::align::Align2;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Area {
    pub(crate) movable: bool,
    pub(crate) interactable: bool,
    pub(crate) enabled: bool,
    pub(crate) constrain: bool,
    pub(crate) constrain_rect: ROption<Rect>,
    pub(crate) default_pos: ROption<Pos2>,
    pub(crate) default_size: Vec2,
    pub(crate) pivot: Align2,
    pub(crate) anchor: ROption<Tuple2<Align2, Vec2>>,
    pub(crate) new_pos: ROption<Pos2>,
}

impl Default for Area {
    fn default() -> Self {
        Self {
            movable: true,
            interactable: true,
            enabled: true,
            constrain: true,
            constrain_rect: RNone,
            default_pos: RNone,
            default_size: Vec2::NAN,
            pivot: Align2::LEFT_TOP,
            anchor: RNone,
            new_pos: RNone,
        }
    }
}

impl Area {
    #[inline]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    #[inline]
    pub fn movable(mut self, movable: bool) -> Self {
        self.movable = movable;
        self.interactable |= movable;
        self
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_movable(&self) -> bool {
        self.movable && self.enabled
    }

    #[inline]
    pub fn interactable(mut self, interactable: bool) -> Self {
        self.interactable = interactable;
        self.movable &= interactable;
        self
    }

    #[inline]
    pub fn default_pos(mut self, default_pos: impl Into<Pos2>) -> Self {
        self.default_pos = RSome(default_pos.into());
        self
    }

    #[inline]
    pub fn default_size(mut self, default_size: impl Into<Vec2>) -> Self {
        self.default_size = default_size.into();
        self
    }

    #[inline]
    pub fn default_width(mut self, default_width: f32) -> Self {
        self.default_size.x = default_width;
        self
    }

    #[inline]
    pub fn default_height(mut self, default_height: f32) -> Self {
        self.default_size.y = default_height;
        self
    }

    #[inline]
    pub fn fixed_pos(mut self, fixed_pos: impl Into<Pos2>) -> Self {
        self.new_pos = RSome(fixed_pos.into());
        self.movable = false;
        self
    }

    #[inline]
    pub fn constrain(mut self, constrain: bool) -> Self {
        self.constrain = constrain;
        self
    }

    #[inline]
    pub fn constrain_to(mut self, constrain_rect: Rect) -> Self {
        self.constrain = true;
        self.constrain_rect = RSome(constrain_rect);
        self
    }

    #[inline]
    pub fn pivot(mut self, pivot: Align2) -> Self {
        self.pivot = pivot;
        self
    }

    #[inline]
    pub fn current_pos(mut self, current_pos: impl Into<Pos2>) -> Self {
        self.new_pos = RSome(current_pos.into());
        self
    }

    #[inline]
    pub fn anchor(mut self, align: Align2, offset: impl Into<Vec2>) -> Self {
        self.anchor = RSome(Tuple2(align, offset.into()));
        self.movable(false)
    }
}
