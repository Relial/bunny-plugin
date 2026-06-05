#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CornerRadius {
    pub nw: u8,
    pub ne: u8,
    pub sw: u8,
    pub se: u8,
}

impl CornerRadius {
    pub const ZERO: Self = Self {
        nw: 0,
        ne: 0,
        sw: 0,
        se: 0,
    };

    pub const fn same(radius: u8) -> Self {
        Self {
            nw: radius,
            ne: radius,
            sw: radius,
            se: radius,
        }
    }
}

impl Default for CornerRadius {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<u8> for CornerRadius {
    fn from(value: u8) -> Self {
        Self::same(value)
    }
}

impl From<f32> for CornerRadius {
    fn from(value: f32) -> Self {
        Self::same(value.round() as u8)
    }
}

impl From<CornerRadius> for egui::CornerRadius {
    fn from(value: CornerRadius) -> Self {
        Self {
            nw: value.nw,
            ne: value.ne,
            sw: value.sw,
            se: value.se,
        }
    }
}

impl From<egui::CornerRadius> for CornerRadius {
    fn from(value: egui::CornerRadius) -> Self {
        Self {
            nw: value.nw,
            ne: value.ne,
            sw: value.sw,
            se: value.se,
        }
    }
}

impl std::ops::Add for CornerRadius {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            nw: self.nw.saturating_add(rhs.nw),
            ne: self.ne.saturating_add(rhs.ne),
            sw: self.sw.saturating_add(rhs.sw),
            se: self.se.saturating_add(rhs.se),
        }
    }
}

impl std::ops::Add<u8> for CornerRadius {
    type Output = Self;
    #[inline]
    fn add(self, rhs: u8) -> Self {
        Self {
            nw: self.nw.saturating_add(rhs),
            ne: self.ne.saturating_add(rhs),
            sw: self.sw.saturating_add(rhs),
            se: self.se.saturating_add(rhs),
        }
    }
}

impl std::ops::AddAssign for CornerRadius {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            nw: self.nw.saturating_add(rhs.nw),
            ne: self.ne.saturating_add(rhs.ne),
            sw: self.sw.saturating_add(rhs.sw),
            se: self.se.saturating_add(rhs.se),
        };
    }
}

impl std::ops::AddAssign<u8> for CornerRadius {
    #[inline]
    fn add_assign(&mut self, rhs: u8) {
        *self = Self {
            nw: self.nw.saturating_add(rhs),
            ne: self.ne.saturating_add(rhs),
            sw: self.sw.saturating_add(rhs),
            se: self.se.saturating_add(rhs),
        };
    }
}

impl std::ops::Sub for CornerRadius {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            nw: self.nw.saturating_sub(rhs.nw),
            ne: self.ne.saturating_sub(rhs.ne),
            sw: self.sw.saturating_sub(rhs.sw),
            se: self.se.saturating_sub(rhs.se),
        }
    }
}

impl std::ops::Sub<u8> for CornerRadius {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: u8) -> Self {
        Self {
            nw: self.nw.saturating_sub(rhs),
            ne: self.ne.saturating_sub(rhs),
            sw: self.sw.saturating_sub(rhs),
            se: self.se.saturating_sub(rhs),
        }
    }
}

impl std::ops::SubAssign for CornerRadius {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            nw: self.nw.saturating_sub(rhs.nw),
            ne: self.ne.saturating_sub(rhs.ne),
            sw: self.sw.saturating_sub(rhs.sw),
            se: self.se.saturating_sub(rhs.se),
        };
    }
}

impl std::ops::SubAssign<u8> for CornerRadius {
    #[inline]
    fn sub_assign(&mut self, rhs: u8) {
        *self = Self {
            nw: self.nw.saturating_sub(rhs),
            ne: self.ne.saturating_sub(rhs),
            sw: self.sw.saturating_sub(rhs),
            se: self.se.saturating_sub(rhs),
        };
    }
}

impl std::ops::Div<f32> for CornerRadius {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self {
            nw: (self.nw as f32 / rhs) as u8,
            ne: (self.ne as f32 / rhs) as u8,
            sw: (self.sw as f32 / rhs) as u8,
            se: (self.se as f32 / rhs) as u8,
        }
    }
}

impl std::ops::DivAssign<f32> for CornerRadius {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            nw: (self.nw as f32 / rhs) as u8,
            ne: (self.ne as f32 / rhs) as u8,
            sw: (self.sw as f32 / rhs) as u8,
            se: (self.se as f32 / rhs) as u8,
        };
    }
}

impl std::ops::Mul<f32> for CornerRadius {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self {
            nw: (self.nw as f32 * rhs) as u8,
            ne: (self.ne as f32 * rhs) as u8,
            sw: (self.sw as f32 * rhs) as u8,
            se: (self.se as f32 * rhs) as u8,
        }
    }
}

impl std::ops::MulAssign<f32> for CornerRadius {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            nw: (self.nw as f32 * rhs) as u8,
            ne: (self.ne as f32 * rhs) as u8,
            sw: (self.sw as f32 * rhs) as u8,
            se: (self.se as f32 * rhs) as u8,
        };
    }
}
