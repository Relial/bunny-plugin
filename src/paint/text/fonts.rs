use abi_stable::std_types::RString;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct FontId {
    pub size: f32,
    pub family: FontFamily,
}

impl Default for FontId {
    #[inline]
    fn default() -> Self {
        Self {
            size: 14.0,
            family: FontFamily::Proportional,
        }
    }
}

impl FontId {
    #[inline]
    pub const fn new(size: f32, family: FontFamily) -> Self {
        Self { size, family }
    }

    #[inline]
    pub const fn proportional(size: f32) -> Self {
        Self::new(size, FontFamily::Proportional)
    }

    pub const fn monospace(size: f32) -> Self {
        Self::new(size, FontFamily::Monospace)
    }
}

impl From<FontId> for egui::FontId {
    fn from(value: FontId) -> Self {
        let FontId { size, family } = value;
        Self {
            size,
            family: family.into(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum FontFamily {
    #[default]
    Proportional,
    Monospace,
    Name(RString),
}

impl From<FontFamily> for egui::FontFamily {
    fn from(value: FontFamily) -> Self {
        match value {
            FontFamily::Proportional => Self::Proportional,
            FontFamily::Monospace => Self::Monospace,
            FontFamily::Name(rstring) => Self::Name(rstring.as_str().into()),
        }
    }
}
