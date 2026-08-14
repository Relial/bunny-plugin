use std::path::Path;

use abi_stable::std_types::RArc;
use anyhow::Result;
use glam::{Quat, Vec3};
use shared::{
    camera::Camera,
    texture::{NamedTexture, SharedTextures, SizedTexture, TextureId},
};

use crate::{
    backend::GpuColor,
    core::{
        asset_loader::{AssetLoader, MeshPoll, TexturePoll},
        draw_list::DrawList,
        texture::{TextureAllocation, TextureSource, Textures},
    },
    mesh::{Mesh, UvOrigin},
};

pub mod asset_loader;
pub(crate) mod draw_list;
pub mod mesh;
pub(crate) mod texture;

mod transform;
pub use transform::*;

#[derive(Debug, Default)]
#[repr(C)]
pub struct Bunny3d {
    asset_loader: AssetLoader,
    pub(crate) normal_draws: DrawList,
    pub(crate) no_depth_buffer_draws: DrawList,
    textures: Textures,
    camera: RArc<Camera>,
}

impl Bunny3d {
    pub fn add(&mut self, mesh: &Mesh, draw_options: &DrawOptions) {
        if draw_options.draw_on_top {
            self.no_depth_buffer_draws.add(mesh, draw_options);
        } else {
            self.normal_draws.add(mesh, draw_options);
        }
    }

    pub fn allocate_texture<'a>(&mut self, texture: impl Into<TextureSource<'a>>) -> TextureId {
        self.textures.allocate(texture)
    }

    #[inline]
    pub fn allocations_len(&self) -> usize {
        self.textures.allocations_len()
    }

    /// Get a texture loaded from the bunny_textures directory.
    /// The textures are loaded asynchronously, so you should not assume this returns what you want at startup.
    #[inline]
    pub fn get_shared_texture(&self, texture_file_name: impl AsRef<str>) -> Option<SizedTexture> {
        self.textures.get_texture(texture_file_name)
    }

    /// Textures loaded from the bunny_textures directory.
    /// The textures are loaded asynchronously, so you should not assume this returns what you want at startup.
    #[inline]
    pub fn shared_textures(&self) -> &[NamedTexture] {
        self.textures.textures()
    }

    /// Asynchronously load a texture from an image file path
    ///
    /// Loads are cached, so you can safely call this every frame
    pub fn load_texture(&mut self, path: impl AsRef<Path>) -> Result<TexturePoll> {
        self.asset_loader.load_texture(&mut self.textures, path)
    }

    /// Asynchronously load mesh(es) from a .obj file path
    ///
    /// Loads are cached, so you can safely call this every frame
    pub fn load_obj(&mut self, path: impl AsRef<Path>, uv_origin: UvOrigin) -> Result<MeshPoll> {
        self.asset_loader.load_obj(path, uv_origin)
    }

    #[inline]
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}

impl Bunny3d {
    pub(crate) fn start_frame(&mut self, camera: RArc<Camera>) {
        self.normal_draws.start_frame();
        self.no_depth_buffer_draws.start_frame();
        self.asset_loader.initialize_loads();
        self.camera = camera;
    }

    pub(crate) fn extract_allocations(&mut self) -> impl Iterator<Item = TextureAllocation> {
        self.textures.extract_allocations()
    }

    pub(crate) fn add_shared(&mut self, shared: RArc<SharedTextures>) {
        self.textures.add_shared(shared);
    }

    pub(crate) fn free_texture(&mut self, texture: TextureId) {
        self.asset_loader.free_texture(texture);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum FillMode {
    Wireframe = 2,
    Solid = 3,
}

#[derive(Debug)]
pub struct DrawOptions {
    pub draw_on_top: bool,
    pub fill: FillMode,
    pub transform: Transform,
    pub texture: Option<TextureId>,
    pub color: GpuColor,
}

impl Default for DrawOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl DrawOptions {
    pub const fn new() -> Self {
        Self {
            draw_on_top: false,
            fill: FillMode::Wireframe,
            transform: Transform::IDENTITY,
            texture: None,
            color: GpuColor::WHITE,
        }
    }

    #[inline]
    pub fn draw_on_top(mut self, draw_on_top: bool) -> Self {
        self.draw_on_top = draw_on_top;
        self
    }

    #[inline]
    pub fn fill(mut self, fill: FillMode) -> Self {
        self.fill = fill;
        self
    }

    #[inline]
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    #[inline]
    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.transform = self.transform.with_translation(translation);
        self
    }

    #[inline]
    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.transform = self.transform.with_rotation(rotation);
        self
    }

    #[inline]
    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.transform = self.transform.with_scale(scale);
        self
    }

    #[inline]
    pub fn texture(mut self, texture: TextureId) -> Self {
        self.texture = Some(texture);
        self.fill = FillMode::Solid;
        self
    }

    #[inline]
    pub fn color(mut self, color: impl Into<GpuColor>) -> Self {
        self.color = color.into();
        self
    }
}
