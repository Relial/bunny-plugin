use abi_stable::std_types::{RHashMap, RString, RVec};
use image::DynamicImage;
use rapidhash::fast::RandomState;

use crate::backend::GpuColor;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum TextureId {
    Managed(u64),
    Shared(u64),
}

#[allow(clippy::derivable_impls)]
impl Default for TextureId {
    fn default() -> Self {
        Self::Managed(0)
    }
}

impl std::fmt::Display for TextureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureId::Managed(id) => write!(f, "Managed {id}"),
            TextureId::Shared(id) => write!(f, "Shared {id}"),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Textures {
    shared_textures: Option<SharedTextures>,
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

    pub(crate) fn add_shared(&mut self, shared: SharedTextures) {
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct SharedTextures {
    list: RVec<NamedTexture>,
    map: RHashMap<RString, SizedTexture, RandomState>,
}

impl SharedTextures {
    pub fn new(textures: impl IntoIterator<Item = (RString, SizedTexture)>) -> Self {
        let (map, list) = textures
            .into_iter()
            .map(|(name, tex)| ((name.clone(), tex), NamedTexture::new(name, tex)))
            .unzip();

        Self { list, map }
    }

    #[inline]
    pub fn get_texture(&self, name: impl AsRef<str>) -> Option<&SizedTexture> {
        self.map.get(name.as_ref())
    }

    #[inline]
    pub fn textures(&self) -> &[NamedTexture] {
        self.list.as_slice()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct SizedTexture {
    pub id: TextureId,
    pub size: [usize; 2],
}

impl SizedTexture {
    pub fn new(id: TextureId, size: [usize; 2]) -> Self {
        Self { id, size }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct NamedTexture {
    file_name: RString,
    texture: SizedTexture,
}

impl NamedTexture {
    pub fn new(file_name: impl Into<RString>, texture: SizedTexture) -> Self {
        Self {
            file_name: file_name.into(),
            texture,
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.file_name
    }

    #[inline]
    pub fn texture(&self) -> &SizedTexture {
        &self.texture
    }
}
