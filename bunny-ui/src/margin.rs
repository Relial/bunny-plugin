use egui::Vec2;

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

impl From<Margin> for egui::Margin {
    fn from(value: Margin) -> Self {
        Self {
            left: value.left,
            right: value.right,
            top: value.top,
            bottom: value.bottom,
        }
    }
}

impl From<egui::Margin> for Margin {
    fn from(value: egui::Margin) -> Self {
        Self {
            left: value.left,
            right: value.right,
            top: value.top,
            bottom: value.bottom,
        }
    }
}
