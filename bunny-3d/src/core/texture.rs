use abi_stable::std_types::{
    RArc,
    ROption::{self, RSome},
    RVec,
};
use image::DynamicImage;
use shared::texture::{NamedTexture, SharedTextures, SizedTexture, TextureId};

use crate::backend::GpuColor;

#[derive(Debug)]
#[repr(C)]
pub struct Textures {
    shared_textures: ROption<RArc<SharedTextures>>,
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

    pub fn allocate<'a>(&mut self, texture: impl Into<TextureSource<'a>>) -> TextureId {
        let id = self.advance_id();

        let data = match texture.into() {
            TextureSource::Image(image) => image.into(),
            TextureSource::RawPixels { size, pixels } => TextureData {
                pixels: pixels.into(),
                size,
            },
        };
        self.allocate_from_data(data, id);
        id
    }

    #[inline]
    pub fn allocations_len(&self) -> usize {
        self.allocations.len()
    }

    #[inline]
    pub fn get_texture(&self, texture_file_name: impl AsRef<str>) -> Option<SizedTexture> {
        self.shared_textures
            .as_ref()
            .into_option()
            .and_then(|s| s.get_texture(texture_file_name))
    }

    #[inline]
    pub fn textures(&self) -> &[NamedTexture] {
        self.shared_textures
            .as_ref()
            .map(|s| s.textures())
            .unwrap_or_default()
    }
}

impl Textures {
    pub fn extract_allocations(&mut self) -> impl Iterator<Item = TextureAllocation> {
        self.allocations.drain(..)
    }

    pub fn add_shared(&mut self, shared: RArc<SharedTextures>) {
        self.shared_textures = RSome(shared);
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct TextureAllocation {
    pub data: TextureData,
    pub id: TextureId,
}

pub enum TextureSource<'a> {
    Image(&'a DynamicImage),
    RawPixels {
        size: [u32; 2],
        pixels: Vec<GpuColor>,
    },
}

impl<'a> From<&'a DynamicImage> for TextureSource<'a> {
    fn from(value: &'a DynamicImage) -> Self {
        Self::Image(value)
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct TextureData {
    pub pixels: RVec<GpuColor>,
    pub size: [u32; 2],
}

impl From<&DynamicImage> for TextureData {
    fn from(image: &DynamicImage) -> Self {
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
