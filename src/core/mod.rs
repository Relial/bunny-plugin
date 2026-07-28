use abi_stable::{rvec, std_types::RVec};
use anyhow::{Context as _, Result, anyhow};
use bevy_mesh::Mesh;
use epaint::Color32;
use glam::{Mat4, Quat, Vec3};
use windows::Win32::Graphics::Direct3D9::{
    D3DRS_FILLMODE, D3DRS_ZENABLE, D3DRS_ZWRITEENABLE, D3DTRANSFORMSTATETYPE, IDirect3DDevice9,
};
use windows_numerics::Matrix4x4;

use crate::{
    backend::{GpuColor, VERTEX_SIZE},
    core::texture::{TextureAllocation, TextureId, TextureSource, Textures},
};

pub mod texture;

#[derive(Debug)]
#[repr(C)]
pub struct Bunny3d {
    textures: Textures,
    vertex_bytes: RVec<u8>,
    index_bytes: RVec<u8>,
    descriptors: RVec<MeshDescriptor>,
    vertex_bytes_position: usize,
}

impl Default for Bunny3d {
    fn default() -> Self {
        Self {
            vertex_bytes: rvec![0; 2400],
            vertex_bytes_position: 0,
            index_bytes: RVec::with_capacity(2400),
            descriptors: Default::default(),
            textures: Textures::default(),
        }
    }
}

impl Bunny3d {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.descriptors.is_empty()
    }

    pub fn add(&mut self, component: &Bunny3dComponent) -> Result<()> {
        let indices_count = component
            .indices_count()
            .ok_or(anyhow!("Component with no indices"))?;
        let indices = component
            .indices()
            .ok_or(anyhow!("Component mesh data extracted before extract call"))?;
        self.index_bytes.extend_from_slice(indices);

        let vertex_size = VERTEX_SIZE as usize;
        let vertex_count = component.vertex_count();
        let vertex_size_required = vertex_size * vertex_count;
        if self.vertex_bytes.len() - self.vertex_bytes_position < vertex_size_required {
            self.vertex_bytes
                .extend(std::iter::repeat_n(0, vertex_size_required));
        }
        component
            .vertices(
                &mut self.vertex_bytes
                    [self.vertex_bytes_position..self.vertex_bytes_position + vertex_size_required],
                vertex_size,
                vertex_count,
            )
            .ok_or(anyhow!("Failed to extract vertices from mesh"))?;

        let mat = Mat4::from_scale_rotation_translation(
            component.scale,
            component.rotation,
            component.translation,
        );
        let cols = mat.to_cols_array();
        let d3dmat = Matrix4x4 {
            M11: cols[0],
            M12: cols[1],
            M13: cols[2],
            M14: cols[3],
            M21: cols[4],
            M22: cols[5],
            M23: cols[6],
            M24: cols[7],
            M31: cols[8],
            M32: cols[9],
            M33: cols[10],
            M34: cols[11],
            M41: cols[12],
            M42: cols[13],
            M43: cols[14],
            M44: cols[15],
        };
        self.descriptors.push(MeshDescriptor {
            vertices: vertex_count,
            indices: indices_count,
            z_buffer: component.z_buffer,
            fill: component.fill,
            world_matrix: d3dmat,
            texture: component.texture.unwrap_or_default(),
        });
        self.vertex_bytes_position += vertex_size_required;
        Ok(())
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
        self.vertex_bytes_position = 0;
        self.index_bytes.clear();
        self.descriptors.clear();
    }

    pub(crate) fn vertex_buffer(&self) -> &[u8] {
        &self.vertex_bytes[0..self.vertex_bytes_position]
    }

    pub(crate) fn index_buffer(&self) -> &[u8] {
        &self.index_bytes
    }

    pub(crate) fn meshes(&self) -> &[MeshDescriptor] {
        &self.descriptors
    }

    pub(crate) fn extract_allocations(&mut self) -> impl Iterator<Item = TextureAllocation> {
        self.textures.extract_allocations()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct MeshDescriptor {
    world_matrix: Matrix4x4,
    pub(crate) vertices: usize,
    pub(crate) indices: usize,
    fill: FillMode,
    pub(crate) texture: TextureId,
    z_buffer: bool,
}

impl MeshDescriptor {
    pub(crate) fn setup(&self, device: &IDirect3DDevice9) -> Result<()> {
        unsafe {
            device
                .SetRenderState(D3DRS_ZENABLE, self.z_buffer as u32)
                .context("Failed to set ZENABLE")?;
            device
                .SetRenderState(D3DRS_ZWRITEENABLE, self.z_buffer as u32)
                .context("Failed to setZWRITEENABLE")?;
            device
                .SetRenderState(D3DRS_FILLMODE, self.fill as u32)
                .context("Failed to set FILLMODE")?;
            device
                .SetTransform(D3DTRANSFORMSTATETYPE(256), &self.world_matrix)
                .context("Failed to set world matrix")?;
        }
        Ok(())
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
    z_buffer: bool,
    fill: FillMode,
    scale: Vec3,
    rotation: Quat,
    translation: Vec3,
    color: Option<GpuColor>,
    texture: Option<TextureId>,
}

impl Bunny3dComponent {
    pub fn new(mesh: impl Into<Mesh>) -> Self {
        Self {
            mesh: mesh.into(),
            z_buffer: true,
            fill: FillMode::Wireframe,
            scale: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            rotation: Quat::IDENTITY,
            translation: Vec3::ZERO,
            color: None,
            texture: None,
        }
    }

    #[inline]
    pub fn z_buffer(mut self, z_buffer: bool) -> Self {
        self.z_buffer = z_buffer;
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
        self.color = Some(color.into());
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
        self.mesh.count_vertices()
    }

    fn vertices(
        &self,
        vertex_buffer: &mut [u8],
        vertex_size: usize,
        vertex_count: usize,
    ) -> Option<()> {
        let positions = self.mesh.attribute(Mesh::ATTRIBUTE_POSITION)?.get_bytes();
        let position_size = 12;
        let color = self.color.unwrap_or(GpuColor::from(Color32::WHITE));
        for (vertex_index, position_bytes) in positions
            .chunks_exact(position_size)
            .take(vertex_count)
            .enumerate()
        {
            let offset = vertex_index * vertex_size;
            vertex_buffer[offset..offset + position_size].copy_from_slice(position_bytes);
            let offset = offset + position_size;
            vertex_buffer[offset..offset + std::mem::size_of::<GpuColor>()]
                .copy_from_slice(color.bytes());
        }

        if let Some(uvs) = self
            .mesh
            .attribute(Mesh::ATTRIBUTE_UV_0)
            .map(|a| a.get_bytes())
        {
            let uv_size = 8;
            for (vertex_index, uv_bytes) in uvs.chunks_exact(uv_size).take(vertex_count).enumerate()
            {
                let offset =
                    vertex_index * vertex_size + position_size + std::mem::size_of::<GpuColor>();
                vertex_buffer[offset..offset + uv_size].copy_from_slice(uv_bytes);
            }
        }
        Some(())
    }

    fn indices_count(&self) -> Option<usize> {
        self.mesh.indices().map(|i| i.len())
    }

    fn indices(&self) -> Option<&[u8]> {
        self.mesh.get_index_buffer_bytes()
    }
}
