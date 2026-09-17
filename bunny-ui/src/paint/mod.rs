use egui::Vec2;
use mint::Vector2;
use shared::texture::{SharedSizedTexture, SharedTextureId};

pub mod brush;
pub mod color;
pub mod corner_radius;
pub mod mesh;
// pub mod paintlist;
pub mod shape_transform;
pub mod shapes;
pub mod stroke;
pub mod text;
pub mod textures;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum TextureId {
    Managed(u64),
    User(u64),
}

impl Default for TextureId {
    fn default() -> Self {
        Self::Managed(0)
    }
}

impl std::fmt::Display for TextureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureId::Managed(id) => write!(f, "Managed {id}"),
            TextureId::User(id) => write!(f, "User {id}"),
        }
    }
}

impl From<SharedTextureId> for TextureId {
    fn from(value: SharedTextureId) -> Self {
        Self::User(value.inner())
    }
}

#[cfg(feature = "manager")]
impl From<TextureId> for egui::TextureId {
    fn from(value: TextureId) -> Self {
        match value {
            TextureId::Managed(id) => Self::Managed(id),
            TextureId::User(id) => Self::User(id),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct SizedTexture {
    pub id: TextureId,
    pub size: Vec2,
}

impl SizedTexture {
    #[inline]
    pub fn new(id: impl Into<TextureId>, size: impl Into<Vector2<f32>>) -> Self {
        Self {
            id: id.into(),
            size: size.into().into(),
        }
    }

    #[inline]
    pub fn from_shared(shared: SharedSizedTexture) -> Self {
        shared.into()
    }
}

impl From<SharedSizedTexture> for SizedTexture {
    #[inline]
    fn from(value: SharedSizedTexture) -> Self {
        let SharedSizedTexture { id, size } = value;
        Self {
            id: id.into(),
            size: Vec2 {
                x: size[0] as f32,
                y: size[1] as f32,
            },
        }
    }
}

#[cfg(feature = "manager")]
impl From<SizedTexture> for egui::load::SizedTexture {
    fn from(value: SizedTexture) -> Self {
        let SizedTexture { id, size } = value;
        Self {
            id: id.into(),
            size,
        }
    }
}
