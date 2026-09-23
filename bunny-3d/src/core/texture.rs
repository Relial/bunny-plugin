use abi_stable::std_types::RVec;
use shared::texture::SharedTextureId;

use crate::GpuColor;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum TextureId3d {
    Managed(u64),
    User(u64),
}

impl Default for TextureId3d {
    fn default() -> Self {
        Self::Managed(0)
    }
}

impl From<SharedTextureId> for TextureId3d {
    fn from(value: SharedTextureId) -> Self {
        Self::User(value.inner())
    }
}

impl std::fmt::Display for TextureId3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureId3d::Managed(id) => write!(f, "Managed {id}"),
            TextureId3d::User(id) => write!(f, "User {id}"),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Textures {
    allocations: RVec<TextureAllocation>,
    next_id: u64,
}

impl Default for Textures {
    fn default() -> Self {
        Self {
            allocations: Default::default(),
            next_id: 1,
        }
    }
}

impl Textures {
    #[inline]
    pub fn advance_id(&mut self) -> TextureId3d {
        let id = TextureId3d::Managed(self.next_id);
        self.next_id += 1;
        id
    }

    #[inline]
    pub fn allocate_from_data(&mut self, data: TextureData, id: TextureId3d) {
        let allocation = TextureAllocation { data, id };
        self.allocations.push(allocation);
    }

    pub fn allocate(&mut self, texture: impl Into<TextureData>) -> TextureId3d {
        let id = self.advance_id();
        let data = texture.into();
        self.allocate_from_data(data, id);
        id
    }

    #[inline]
    pub fn allocations_len(&self) -> usize {
        self.allocations.len()
    }
}

impl Textures {
    pub fn extract_allocations(&mut self) -> impl Iterator<Item = TextureAllocation> {
        self.allocations.drain(..)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct TextureAllocation {
    pub data: TextureData,
    pub id: TextureId3d,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct TextureData {
    pub pixels: RVec<GpuColor>,
    pub size: [u32; 2],
}

impl TextureData {
    pub fn from_raw_pixels(
        pixels: impl IntoIterator<Item = GpuColor>,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            pixels: pixels.into_iter().collect(),
            size: [width, height],
        }
    }
}

#[cfg(feature = "image")]
impl From<&image::DynamicImage> for TextureData {
    fn from(image: &image::DynamicImage) -> Self {
        let size = [image.width(), image.height()];
        let rgba = image.to_rgba8().into_flat_samples();
        let (chunks, _) = rgba.as_slice().as_chunks::<4>();
        let pixels: RVec<GpuColor> = chunks
            .iter()
            .map(|c| GpuColor::from_rgba_bytes(c.as_slice()))
            .collect();
        Self { pixels, size }
    }
}
