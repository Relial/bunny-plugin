use std::path::Path;

use abi_stable::std_types::RArc;
use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use ecolor::Color32;
use glam::{Quat, Vec3};
#[cfg(feature = "backend")]
use shared::texture::SharedTextures;
use shared::{
    camera::Camera,
    texture::{SharedTextures, TextureId},
};

#[cfg(feature = "backend")]
use crate::texture::TextureAllocation;
use crate::{
    core::{
        asset_loader::{AssetLoader, MeshPoll, TexturePoll},
        draw_list::DrawList,
        texture::Textures,
    },
    mesh::{Mesh, UvOrigin},
    texture::TextureData,
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
    /// Convert the mesh into a GPU friendly format and add it to this frame's draw list.
    pub fn draw(&mut self, mesh: &Mesh, draw_options: &DrawOptions) {
        if draw_options.draw_on_top {
            self.no_depth_buffer_draws.add(mesh, draw_options);
        } else {
            self.normal_draws.add(mesh, draw_options);
        }
    }

    /// Allocate a texture on the GPU. Every call to this allocates, so this should only be called once per texture.
    ///
    /// If you want cached texture loading, use load_texture instead
    pub fn allocate_texture(&mut self, texture: impl Into<TextureData>) -> TextureId {
        self.textures.allocate(texture)
    }

    /// Textures allocated this frame
    #[inline]
    pub fn allocations_len(&self) -> usize {
        self.textures.allocations_len()
    }

    #[inline]
    pub fn shared_textures(&self) -> Option<SharedTextures> {
        self.textures.shared_textures()
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

    /// Get information about the game camera
    #[inline]
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}

#[cfg(feature = "backend")]
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

    pub(crate) fn add_shared(&mut self, shared: SharedTextures) {
        self.textures.add_shared(shared);
    }

    pub(crate) fn free_texture(&mut self, texture: TextureId) {
        self.asset_loader.free_texture(texture);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum FillMode {
    Wireframe = 2,
    Solid = 3,
}

#[derive(Clone, Copy, Debug)]
pub struct DrawOptions {
    pub draw_on_top: bool,
    pub fill: FillMode,
    pub transform: Transform,
    pub texture: Option<TextureId>,
    pub override_vertex_color: Option<GpuColor>,
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
            override_vertex_color: None,
        }
    }

    /// Draw on top of all game visuals. BunnyUi draws will still happen on top of this.
    #[inline]
    pub fn draw_on_top(mut self, draw_on_top: bool) -> Self {
        self.draw_on_top = draw_on_top;
        self
    }

    /// Choose between wireframe and solid.
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

    /// Override all vertex colors
    #[inline]
    pub fn color(mut self, color: impl Into<GpuColor>) -> Self {
        self.override_vertex_color = Some(color.into());
        self
    }
}

/// GBRA
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GpuColor([u8; 4]);

impl GpuColor {
    pub const WHITE: Self = Self::from_rgb(255, 255, 255);

    #[inline]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self([b, g, r, 255])
    }

    #[inline]
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([b, g, r, a])
    }

    #[inline]
    pub const fn from_rgba_float(r: f32, g: f32, b: f32, a: f32) -> Self {
        let r = (r * u8::MAX as f32).round() as u8;
        let g = (g * u8::MAX as f32).round() as u8;
        let b = (b * u8::MAX as f32).round() as u8;
        let a = (a * u8::MAX as f32).round() as u8;
        Self::from_rgba(r, g, b, a)
    }

    #[inline]
    pub const fn from_rgba_bytes(bytes: &[u8]) -> Self {
        Self([bytes[2], bytes[1], bytes[0], bytes[3]])
    }

    #[inline]
    pub const fn from_bgra_bytes(bytes: &[u8]) -> Self {
        Self([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Color32> for GpuColor {
    fn from(value: Color32) -> Self {
        let cols = value.to_array();
        Self([cols[2], cols[1], cols[0], cols[3]])
    }
}
