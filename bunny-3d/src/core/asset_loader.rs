use std::{
    ffi::OsString,
    hash::{BuildHasherDefault, Hash, Hasher},
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::{Path, PathBuf},
    thread,
};

use abi_stable::{
    external_types::RMutex,
    std_types::{
        RArc, RHashMap,
        RResult::{self, RErr, ROk},
        RString, RVec, Tuple2, Tuple3,
        map::REntry,
    },
};
use anyhow::{Result, anyhow};
use image::{DynamicImage, ImageReader};
use rapidhash::fast::RapidHasher;
use shared::texture::TextureId;

use crate::core::{
    mesh::{Mesh, UvOrigin},
    texture::{TextureData, Textures},
};

type BuildNoHash = BuildHasherDefault<NoHash>;

#[derive(Debug, Default)]
#[repr(C)]
struct NoHash(u64);

impl Hasher for NoHash {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _: &[u8]) {
        panic!("NoHash can't write byte slices")
    }

    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}

#[derive(Default)]
#[repr(C)]
pub struct AssetLoader {
    // Workaround for RHashMap::iter_mut() not working
    hash_texture_pairs: RVec<Tuple2<u64, TextureId>>,
    uninitialized_textures: RVec<Tuple2<u64, RVec<u16>>>,
    uninitialized_meshes: RVec<Tuple3<u64, RVec<u16>, UvOrigin>>,
    textures: RArc<RMutex<RHashMap<u64, TexturePollInner, BuildNoHash>>>,
    meshes: RArc<RMutex<RHashMap<u64, MeshPollInner, BuildNoHash>>>,
}

impl AssetLoader {
    pub fn load_texture(
        &mut self,
        textures: &mut Textures,
        path: impl AsRef<Path>,
    ) -> Result<TexturePoll> {
        let hash = hash_path(&path);
        let mut tex = self.textures.lock();
        if let REntry::Occupied(mut entry) = tex.entry(hash) {
            let entry = entry.get_mut();
            match entry {
                TexturePollInner::Ready(ROk(data)) => {
                    let data = std::mem::take(data);
                    let id = textures.advance_id();
                    textures.allocate_from_data(data, id);
                    *entry = TexturePollInner::Extracted(id);
                    self.hash_texture_pairs.push(Tuple2(hash, id));
                    Ok(TexturePoll::Ready(id))
                }
                TexturePollInner::Ready(RErr(e)) => Err(anyhow!(e.clone())),
                TexturePollInner::Extracted(texture_id) => Ok(TexturePoll::Ready(*texture_id)),
                TexturePollInner::Pending => Ok(TexturePoll::Pending),
            }
        } else {
            self.uninitialized_textures.push(Tuple2(
                hash,
                path.as_ref().as_os_str().encode_wide().collect(),
            ));
            tex.insert(hash, TexturePollInner::Pending);
            Ok(TexturePoll::Pending)
        }
    }

    pub fn load_obj(&mut self, path: impl AsRef<Path>, uv_origin: UvOrigin) -> Result<MeshPoll> {
        let hash = hash_path(&path);
        let mut meshes = self.meshes.lock();
        if let Some(entry) = meshes.get(&hash) {
            match entry {
                MeshPollInner::Ready(ROk(meshes)) => Ok(MeshPoll::Ready(meshes.clone())),
                MeshPollInner::Ready(RErr(e)) => Err(anyhow!(e.clone())),
                MeshPollInner::Pending => Ok(MeshPoll::Pending),
            }
        } else {
            self.uninitialized_meshes.push(Tuple3(
                hash,
                path.as_ref().as_os_str().encode_wide().collect(),
                uv_origin,
            ));
            meshes.insert(hash, MeshPollInner::Pending);
            Ok(MeshPoll::Pending)
        }
    }
}

impl AssetLoader {
    pub(crate) fn initialize_loads(&mut self) {
        // We do the work on the manager side so that plugins don't all individually compile the image/tobj loading process
        // This saves around 1.8MB of file size for a plugin that loads both an obj and a texture

        for Tuple2(texture_hash, path_wide_bytes) in self.uninitialized_textures.drain(..) {
            thread::spawn({
                let tex = self.textures.clone();
                let path_str = OsString::from_wide(&path_wide_bytes);
                let path: PathBuf = path_str.into();
                move || {
                    let res: RResult<TextureData, RString> = load_image_from_path(path)
                        .map(|image| TextureData::from(&image))
                        .map_err(|e| e.to_string().into())
                        .into();
                    let mut tex = tex.lock();
                    tex.entry(texture_hash)
                        .and_modify(|p| *p = TexturePollInner::Ready(res));
                }
            });
        }

        #[cfg(feature = "tobj")]
        for Tuple3(meshes_hash, path_wide_bytes, uv_origin) in self.uninitialized_meshes.drain(..) {
            thread::spawn({
                let meshes = self.meshes.clone();
                let path_str = OsString::from_wide(&path_wide_bytes);
                let path: PathBuf = path_str.into();
                move || {
                    let res: RResult<LoadedMeshes, RString> = load_obj_from_path(path, uv_origin)
                        .map(LoadedMeshes::new)
                        .map_err(|e| e.to_string().into())
                        .into();
                    let mut meshes = meshes.lock();
                    meshes
                        .entry(meshes_hash)
                        .and_modify(|p| *p = MeshPollInner::Ready(res));
                }
            });
        }
    }

    pub(crate) fn free_texture(&mut self, texture: TextureId) {
        // This sucks, but we only free textures when a plugin unloads, so this shouldn't cause issues unless a plugin allocates a massive number of textures
        // and even then only when a user disables such a plugin
        if let Some(i) = self
            .hash_texture_pairs
            .iter()
            .position(|Tuple2(_, id)| *id == texture)
        {
            let Tuple2(hash, _) = self.hash_texture_pairs.remove(i);
            let mut tex = self.textures.lock();
            tex.remove(&hash);
        }
    }
}

pub enum TexturePoll {
    Ready(TextureId),
    Pending,
}

#[derive(Debug)]
#[repr(C)]
enum TexturePollInner {
    Ready(RResult<TextureData, RString>),
    Extracted(TextureId),
    Pending,
}

pub enum MeshPoll {
    Ready(LoadedMeshes),
    Pending,
}

#[allow(unused)]
#[derive(Clone, Debug)]
#[repr(C)]
enum MeshPollInner {
    Ready(RResult<LoadedMeshes, RString>),
    Pending,
}

#[derive(Clone, Debug)]
pub struct LoadedMeshes(RArc<RVec<Mesh>>);

impl LoadedMeshes {
    #[cfg(feature = "tobj")]
    fn new(meshes: RVec<Mesh>) -> Self {
        Self(RArc::new(meshes))
    }

    pub fn meshes(&self) -> &[Mesh] {
        self.0.as_slice()
    }
}

fn hash_path(path: impl AsRef<Path>) -> u64 {
    let path = path.as_ref();
    let mut hasher = RapidHasher::new(0);
    path.hash(&mut hasher);
    hasher.finish()
}

fn load_image_from_path(path: impl AsRef<Path>) -> Result<DynamicImage> {
    let image = ImageReader::open(path)?.decode()?;
    Ok(image)
}

#[cfg(feature = "tobj")]
fn load_obj_from_path(path: impl AsRef<Path>, uv_origin: UvOrigin) -> Result<RVec<Mesh>> {
    tobj::load_obj(path.as_ref(), &tobj::GPU_LOAD_OPTIONS)?
        .0
        .into_iter()
        .map(|model| {
            use crate::core::mesh::TobjMesh;

            let t = TobjMesh::new(model.mesh, uv_origin);
            t.try_into()
        })
        .collect()
}

impl std::fmt::Debug for AssetLoader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AssetLoader").finish_non_exhaustive()
    }
}
