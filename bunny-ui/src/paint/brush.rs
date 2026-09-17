use emath::Rect;

use crate::paint::TextureId;

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Brush {
    pub fill_texture_id: TextureId,
    pub uv: Rect,
}

#[cfg(feature = "manager")]
impl From<Brush> for egui::epaint::Brush {
    #[inline]
    fn from(value: Brush) -> Self {
        let Brush {
            fill_texture_id,
            uv,
        } = value;
        Self {
            fill_texture_id: fill_texture_id.into(),
            uv,
        }
    }
}
