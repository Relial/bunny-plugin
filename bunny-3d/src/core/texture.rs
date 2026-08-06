use abi_stable::std_types::{RArc, RVec};
use image::DynamicImage;
use shared_textures::{NamedTexture, SharedTextures, SizedTexture, TextureId};

use crate::backend::GpuColor;

#[derive(Debug)]
#[repr(C)]
pub struct Textures {
    shared_textures: Option<RArc<SharedTextures>>,
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
    pub fn allocate<'a>(&mut self, texture: impl Into<TextureSource<'a>>) -> TextureId {
        let id = TextureId::Managed(self.next_id);
        self.next_id += 1;

        let (size, pixels) = match texture.into() {
            TextureSource::Image(image) => {
                let size = [image.width(), image.height()];
                let rgba = image.to_rgba8().into_flat_samples();
                let (chunks, _) = rgba.as_slice().as_chunks::<4>();
                let pixels: RVec<GpuColor> = chunks
                    .iter()
                    .map(|c| GpuColor::from_rgba_bytes(c.as_slice()))
                    .collect();
                (size, pixels)
            }
            TextureSource::RawPixels { size, data } => (size, data.into()),
        };
        let allocation = TextureAllocation { pixels, size, id };
        self.allocations.push(allocation);
        id
    }

    #[inline]
    pub fn allocations_len(&self) -> usize {
        self.allocations.len()
    }

    #[inline]
    pub fn get_texture(&self, texture_file_name: impl AsRef<str>) -> Option<&SizedTexture> {
        self.shared_textures
            .as_ref()
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
    pub(crate) fn extract_allocations(&mut self) -> impl Iterator<Item = TextureAllocation> {
        self.allocations.drain(..)
    }

    pub(crate) fn add_shared(&mut self, shared: RArc<SharedTextures>) {
        self.shared_textures = Some(shared);
    }
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct TextureAllocation {
    pub(crate) pixels: RVec<GpuColor>,
    pub(crate) size: [u32; 2],
    pub(crate) id: TextureId,
}

pub enum TextureSource<'a> {
    Image(&'a DynamicImage),
    RawPixels { size: [u32; 2], data: Vec<GpuColor> },
}

impl<'a> From<&'a DynamicImage> for TextureSource<'a> {
    fn from(value: &'a DynamicImage) -> Self {
        Self::Image(value)
    }
}
