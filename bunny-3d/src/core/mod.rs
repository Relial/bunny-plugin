use glam::{Quat, Vec3};
use shared_textures::{NamedTexture, SharedTextures, SizedTexture, TextureId};

use crate::{
    backend::GpuColor,
    core::{
        draw_list::DrawList,
        mesh::Mesh,
        texture::{TextureAllocation, TextureSource, Textures},
    },
};

pub(crate) mod draw_list;
pub mod mesh;
pub mod texture;

#[derive(Debug, Default)]
#[repr(C)]
pub struct Bunny3d {
    pub(crate) normal_draws: DrawList,
    pub(crate) no_depth_buffer_draws: DrawList,
    textures: Textures,
}

impl Bunny3d {
    pub fn add(&mut self, component: &Bunny3dComponent) {
        if component.draw_on_top {
            self.no_depth_buffer_draws.add(component);
        } else {
            self.normal_draws.add(component);
        }
    }

    pub fn allocate_texture<'a>(&mut self, texture: impl Into<TextureSource<'a>>) -> TextureId {
        self.textures.allocate(texture)
    }

    #[inline]
    pub fn allocations_len(&self) -> usize {
        self.textures.allocations_len()
    }

    /// Get a texture loaded by the manager by its filename
    /// The textures are loaded asynchronously, so you should not assume this returns what you want at startup
    #[inline]
    pub fn get_shared_texture(&self, texture_file_name: impl AsRef<str>) -> Option<&SizedTexture> {
        self.textures.get_texture(texture_file_name)
    }

    /// Textures loaded by the manager
    /// The textures are loaded asynchronously, so you should not assume this returns what you want at startup
    #[inline]
    pub fn shared_textures(&self) -> &[NamedTexture] {
        self.textures.textures()
    }
}

impl Bunny3d {
    pub(crate) fn start_frame(&mut self) {
        self.normal_draws.start_frame();
        self.no_depth_buffer_draws.start_frame();
    }

    pub(crate) fn extract_allocations(&mut self) -> impl Iterator<Item = TextureAllocation> {
        self.textures.extract_allocations()
    }

    pub(crate) fn add_shared(&mut self, shared: SharedTextures) {
        self.textures.add_shared(shared);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum FillMode {
    Wireframe = 2,
    Solid = 3,
}

#[derive(Debug)]
pub struct Bunny3dComponent {
    mesh: Mesh,
    draw_on_top: bool,
    fill: FillMode,
    scale: Vec3,
    rotation: Quat,
    translation: Vec3,
    texture: Option<TextureId>,
}

impl Bunny3dComponent {
    pub fn new(mesh: impl Into<Mesh>) -> Self {
        Self {
            mesh: mesh.into(),
            draw_on_top: false,
            fill: FillMode::Wireframe,
            scale: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            rotation: Quat::IDENTITY,
            translation: Vec3::ZERO,
            texture: None,
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
    pub fn scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    #[inline]
    pub fn rotate(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self
    }

    #[inline]
    pub fn translate(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        self
    }

    #[inline]
    pub fn color(mut self, color: impl Into<GpuColor>) -> Self {
        self.mesh = self.mesh.color(color);
        self
    }

    #[inline]
    pub fn texture(mut self, texture: TextureId) -> Self {
        self.texture = Some(texture);
        self.fill = FillMode::Solid;
        self
    }
}

impl Bunny3dComponent {
    fn vertex_count(&self) -> usize {
        self.mesh.vertex_count()
    }

    fn index_count(&self) -> usize {
        self.mesh.index_count()
    }
}
