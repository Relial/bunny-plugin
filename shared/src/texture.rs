use abi_stable::std_types::{RArc, RHashMap, RString, RVec};
use rapidhash::fast::RandomState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SharedTextureId(u64);

impl SharedTextureId {
    #[inline]
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    #[inline]
    pub fn inner(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for SharedTextureId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Shared {}", self.0)
    }
}

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct SharedTextures(RArc<SharedTexturesImpl>);

impl SharedTextures {
    pub fn new(textures: impl IntoIterator<Item = (RString, SharedSizedTexture)>) -> Self {
        Self(RArc::new(SharedTexturesImpl::new(textures)))
    }

    /// Get a texture loaded by the manager by its filename
    ///
    /// The textures are loaded asynchronously, so you should not assume this returns what you want at startup
    #[inline]
    pub fn get_texture(&self, name: impl AsRef<str>) -> Option<SharedSizedTexture> {
        self.0.get_texture(name)
    }

    /// Textures loaded by the manager
    ///
    /// The textures are loaded asynchronously, so you should not assume this returns what you want at startup
    #[inline]
    pub fn textures(&self) -> &[SharedTexture] {
        self.0.textures()
    }
}

#[derive(Clone, Debug, Default)]
#[repr(C)]
struct SharedTexturesImpl {
    list: RVec<SharedTexture>,
    map: RHashMap<RString, SharedSizedTexture, RandomState>,
}

impl SharedTexturesImpl {
    fn new(textures: impl IntoIterator<Item = (RString, SharedSizedTexture)>) -> Self {
        let (map, list) = textures
            .into_iter()
            .map(|(name, tex)| ((name.clone(), tex), SharedTexture::new(name, tex)))
            .unzip();

        Self { list, map }
    }

    #[inline]
    fn get_texture(&self, name: impl AsRef<str>) -> Option<SharedSizedTexture> {
        self.map.get(name.as_ref()).copied()
    }

    #[inline]
    fn textures(&self) -> &[SharedTexture] {
        self.list.as_slice()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct SharedSizedTexture {
    pub id: SharedTextureId,
    pub size: [usize; 2],
}

impl SharedSizedTexture {
    pub fn new(id: SharedTextureId, size: [usize; 2]) -> Self {
        Self { id, size }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct SharedTexture {
    file_name: RString,
    texture: SharedSizedTexture,
}

impl SharedTexture {
    pub fn new(file_name: impl Into<RString>, texture: SharedSizedTexture) -> Self {
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
    pub fn texture(&self) -> &SharedSizedTexture {
        &self.texture
    }
}
