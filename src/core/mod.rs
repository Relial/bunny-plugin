use glam::{Quat, Vec3};

use crate::{
    backend::GpuColor,
    core::{
        draw_list::DrawList,
        mesh::Mesh,
        texture::{TextureAllocation, TextureId, TextureSource, Textures},
    },
};

pub(crate) mod draw_list;
pub mod mesh;
pub mod texture;

#[derive(Debug, Default)]
#[repr(C)]
pub struct Bunny3d {
    textures: Textures,
    pub(crate) normal_draws: DrawList,
    pub(crate) no_depth_buffer_draws: DrawList,
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
}

impl Bunny3d {
    pub(crate) fn start_frame(&mut self) {
        self.normal_draws.start_frame();
        self.no_depth_buffer_draws.start_frame();
    }

    pub(crate) fn extract_allocations(&mut self) -> impl Iterator<Item = TextureAllocation> {
        self.textures.extract_allocations()
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
