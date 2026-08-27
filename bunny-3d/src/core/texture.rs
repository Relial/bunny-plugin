use abi_stable::std_types::{
    ROption::{self, RSome},
    RVec,
};
use shared::texture::{SharedTextures, TextureId};

use crate::GpuColor;

#[derive(Debug)]
#[repr(C)]
pub struct Textures {
    shared_textures: ROption<SharedTextures>,
    allocations: RVec<TextureAllocation>,
    next_id: u64,
}

impl Default for Textures {
    fn default() -> Self {
        Self {
            allocations: Default::default(),
            next_id: 1,
            shared_textures: Default::default(),
        }
    }
}

impl Textures {
    #[inline]
    pub fn advance_id(&mut self) -> TextureId {
        let id = TextureId::Managed3d(self.next_id);
        self.next_id += 1;
        id
    }

    #[inline]
    pub fn allocate_from_data(&mut self, data: TextureData, id: TextureId) {
        let allocation = TextureAllocation { data, id };
        self.allocations.push(allocation);
    }

    pub fn allocate(&mut self, texture: impl Into<TextureData>) -> TextureId {
        let id = self.advance_id();
        let data = texture.into();
        self.allocate_from_data(data, id);
        id
    }

    #[inline]
    pub fn allocations_len(&self) -> usize {
        self.allocations.len()
    }

    /// Textures loaded by the manager
    #[inline]
    pub fn shared_textures(&self) -> Option<SharedTextures> {
        self.shared_textures.clone().into_option()
    }
}

impl Textures {
    pub fn extract_allocations(&mut self) -> impl Iterator<Item = TextureAllocation> {
        self.allocations.drain(..)
    }

    pub fn add_shared(&mut self, shared: SharedTextures) {
        self.shared_textures = RSome(shared);
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct TextureAllocation {
    pub data: TextureData,
    pub id: TextureId,
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
