#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vec2b {
    pub x: bool,
    pub y: bool,
}

impl Vec2b {
    pub const FALSE: Self = Self { x: false, y: false };
    pub const TRUE: Self = Self { x: true, y: true };

    #[inline]
    pub fn new(x: bool, y: bool) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn any(&self) -> bool {
        self.x || self.y
    }

    #[inline]
    pub fn all(&self) -> bool {
        self.x && self.y
    }
}

impl From<bool> for Vec2b {
    fn from(value: bool) -> Self {
        Self { x: value, y: value }
    }
}

impl From<[bool; 2]> for Vec2b {
    fn from([x, y]: [bool; 2]) -> Self {
        Self { x, y }
    }
}

impl std::ops::Index<usize> for Vec2b {
    type Output = bool;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vec2b index out of bounds: {index}"),
        }
    }
}

impl std::ops::IndexMut<usize> for Vec2b {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vec2b index out of bounds: {index}"),
        }
    }
}

#[cfg(feature = "manager")]
impl From<Vec2b> for egui::Vec2b {
    fn from(value: Vec2b) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
