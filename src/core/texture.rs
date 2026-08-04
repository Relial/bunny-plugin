use abi_stable::std_types::RVec;
use image::DynamicImage;

use crate::backend::GpuColor;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(C)]
pub struct TextureId(u64);

impl TextureId {
    pub(crate) fn new(id: u64) -> Self {
        Self(id)
    }
}

impl std::fmt::Display for TextureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
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
    pub fn allocate<'a>(&mut self, texture: impl Into<TextureSource<'a>>) -> TextureId {
        let id = TextureId::new(self.next_id);
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
}

impl Textures {
    pub(crate) fn extract_allocations(&mut self) -> impl Iterator<Item = TextureAllocation> {
        self.allocations.drain(..)
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
