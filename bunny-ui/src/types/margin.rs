use emath::{Rect, Vec2, vec2};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Margin {
    pub left: i8,
    pub right: i8,
    pub top: i8,
    pub bottom: i8,
}

impl Margin {
    pub const ZERO: Self = Self {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    };

    #[inline]
    pub const fn same(margin: i8) -> Self {
        Self {
            left: margin,
            right: margin,
            top: margin,
            bottom: margin,
        }
    }

    #[inline]
    pub const fn symmetric(x: i8, y: i8) -> Self {
        Self {
            left: x,
            right: x,
            top: y,
            bottom: y,
        }
    }

    #[inline]
    pub const fn leftf(self) -> f32 {
        self.left as f32
    }

    #[inline]
    pub const fn rightf(self) -> f32 {
        self.right as f32
    }

    #[inline]
    pub const fn topf(self) -> f32 {
        self.top as f32
    }

    #[inline]
    pub const fn bottomf(self) -> f32 {
        self.bottom as f32
    }

    #[inline]
    pub fn sum(self) -> Vec2 {
        vec2(self.leftf() + self.rightf(), self.topf() + self.bottomf())
    }

    #[inline]
    pub const fn left_top(self) -> Vec2 {
        vec2(self.leftf(), self.topf())
    }

    #[inline]
    pub const fn right_bottom(self) -> Vec2 {
        vec2(self.rightf(), self.bottomf())
    }

    #[inline]
    pub const fn is_same(self) -> bool {
        self.left == self.right && self.left == self.top && self.left == self.bottom
    }
}

impl From<i8> for Margin {
    #[inline]
    fn from(value: i8) -> Self {
        Self::same(value)
    }
}

impl From<f32> for Margin {
    #[inline]
    fn from(value: f32) -> Self {
        Self::same(value.round() as _)
    }
}

impl From<Vec2> for Margin {
    #[inline]
    fn from(value: Vec2) -> Self {
        Self::symmetric(value.x.round() as _, value.y.round() as _)
    }
}

impl std::ops::Add<Margin> for Rect {
    type Output = Self;

    #[inline]
    fn add(self, margin: Margin) -> Self::Output {
        Self::from_min_max(
            self.min - margin.left_top(),
            self.max + margin.right_bottom(),
        )
    }
}

impl std::ops::AddAssign<Margin> for Rect {
    #[inline]
    fn add_assign(&mut self, margin: Margin) {
        *self = *self + margin
    }
}

impl std::ops::Sub<Margin> for Rect {
    type Output = Self;

    #[inline]
    fn sub(self, margin: Margin) -> Self::Output {
        Self::from_min_max(
            self.min + margin.left_top(),
            self.max - margin.right_bottom(),
        )
    }
}

impl std::ops::SubAssign<Margin> for Rect {
    #[inline]
    fn sub_assign(&mut self, margin: Margin) {
        *self = *self - margin
    }
}

#[cfg(feature = "manager")]
impl From<Margin> for egui::Margin {
    #[inline]
    fn from(value: Margin) -> Self {
        Self {
            left: value.left,
            right: value.right,
            top: value.top,
            bottom: value.bottom,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::Margin> for Margin {
    #[inline]
    fn from(value: egui::Margin) -> Self {
        Self {
            left: value.left,
            right: value.right,
            top: value.top,
            bottom: value.bottom,
        }
    }
}
