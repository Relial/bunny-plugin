use abi_stable::std_types::{RHashMap, RString, RVec};
use rapidhash::fast::RandomState;

use crate::image_source::{ImageSource, SizedTexture};

#[derive(Debug)]
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


