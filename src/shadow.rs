use egui::Color32;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Shadow {
    pub offset: [i8; 2],
    pub blur: u8,
    pub spread: u8,
    pub color: Color32,
}

impl Shadow {
    pub const NONE: Self = Self {
        offset: [0, 0],
        blur: 0,
        spread: 0,
        color: Color32::TRANSPARENT,
    };
}

impl From<Shadow> for egui::Shadow {
    fn from(value: Shadow) -> Self {
        Self {
            offset: value.offset,
            blur: value.blur,
            spread: value.spread,
            color: value.color,
        }
    }
}
