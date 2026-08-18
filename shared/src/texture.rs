use abi_stable::std_types::{RHashMap, RString, RVec};
use rapidhash::fast::RandomState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum TextureId {
    Managed3d(u64),
    Shared(u64),
}

#[allow(clippy::derivable_impls)]
impl Default for TextureId {
    fn default() -> Self {
        Self::Managed3d(0)
    }
}

impl std::fmt::Display for TextureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureId::Managed3d(id) => write!(f, "Managed {id}"),
            TextureId::Shared(id) => write!(f, "Shared {id}"),
        }
    }
}

#[derive(Clone, Debug, Default)]
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
    pub fn get_texture(&self, name: impl AsRef<str>) -> Option<SizedTexture> {
        self.map.get(name.as_ref()).copied()
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
