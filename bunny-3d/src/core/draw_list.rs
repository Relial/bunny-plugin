use abi_stable::std_types::RVec;
use anyhow::{Context as _, Result};
use shared::texture::TextureId;
use windows::Win32::Graphics::Direct3D9::{
    D3DPRIMITIVETYPE, D3DPT_LINELIST, D3DPT_POINTLIST, D3DPT_TRIANGLELIST, D3DRS_FILLMODE,
    D3DTRANSFORMSTATETYPE, IDirect3DDevice9,
};
use windows_numerics::Matrix4x4;

use crate::{
    backend::VERTEX_SIZE,
    core::{DrawOptions, FillMode, mesh::Mesh},
};

#[derive(Debug, Default)]
#[repr(C)]
pub struct DrawList {
    vertex_bytes: RVec<u8>,
    index_bytes: RVec<u8>,
    descriptors: RVec<MeshDescriptor>,
    vertex_bytes_position: usize,
}

impl DrawList {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.descriptors.is_empty()
    }

    pub fn add(&mut self, mesh: &Mesh, draw_options: &DrawOptions) {
        let indices_count = mesh.index_count();
        let indices = mesh.index_buffer_bytes();
        self.index_bytes.extend_from_slice(indices);

        let vertex_size = VERTEX_SIZE as usize;
        let vertex_count = mesh.vertex_count();
        let vertex_size_required = vertex_size * vertex_count;
        if self.vertex_bytes.len() - self.vertex_bytes_position < vertex_size_required {
            self.vertex_bytes
                .extend(std::iter::repeat_n(0, vertex_size_required));
        }
        mesh.update_vertex_buffer(
            &mut self.vertex_bytes
                [self.vertex_bytes_position..self.vertex_bytes_position + vertex_size_required],
            vertex_size,
            draw_options.override_vertex_color,
        );

        let mat = draw_options.transform.to_matrix_d3d();
        self.descriptors.push(MeshDescriptor {
            vertices: vertex_count,
            indices: indices_count,
            fill: draw_options.fill,
            world_matrix: mat,
            texture: draw_options.texture.unwrap_or_default(),
            primitive_topology: mesh.primitive_topology,
        });
        self.vertex_bytes_position += vertex_size_required;
    }
}

impl DrawList {
    pub fn start_frame(&mut self) {
        self.vertex_bytes_position = 0;
        self.index_bytes.clear();
        self.descriptors.clear();
    }

    pub fn vertex_buffer(&self) -> &[u8] {
        &self.vertex_bytes[0..self.vertex_bytes_position]
    }

    pub fn index_buffer(&self) -> &[u8] {
        &self.index_bytes
    }

    pub fn meshes(&self) -> &[MeshDescriptor] {
        &self.descriptors
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct MeshDescriptor {
    world_matrix: Matrix4x4,
    pub vertices: usize,
    pub indices: usize,
    fill: FillMode,
    pub texture: TextureId,
    pub primitive_topology: PrimitiveTopology,
}

impl MeshDescriptor {
    pub fn setup(&self, device: &IDirect3DDevice9) -> Result<()> {
        unsafe {
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

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    TriangleList,
}

impl PrimitiveTopology {
    #[inline]
    pub fn to_d3d(self) -> D3DPRIMITIVETYPE {
        self.into()
    }
}

impl From<PrimitiveTopology> for D3DPRIMITIVETYPE {
    #[inline]
    fn from(value: PrimitiveTopology) -> Self {
        match value {
            PrimitiveTopology::PointList => D3DPT_POINTLIST,
            PrimitiveTopology::LineList => D3DPT_LINELIST,
            PrimitiveTopology::TriangleList => D3DPT_TRIANGLELIST,
        }
    }
}

#[cfg(feature = "bevy")]
impl TryFrom<bevy_mesh::PrimitiveTopology> for PrimitiveTopology {
    type Error = anyhow::Error;

    fn try_from(
        value: bevy_mesh::PrimitiveTopology,
    ) -> std::prelude::v1::Result<Self, Self::Error> {
        use anyhow::anyhow;

        match value {
            bevy_mesh::PrimitiveTopology::PointList => Ok(Self::PointList),
            bevy_mesh::PrimitiveTopology::LineList => Ok(Self::LineList),
            bevy_mesh::PrimitiveTopology::LineStrip => Err(anyhow!("LineStrip not supported")),
            bevy_mesh::PrimitiveTopology::TriangleList => Ok(Self::TriangleList),
            bevy_mesh::PrimitiveTopology::TriangleStrip => {
                Err(anyhow!("TriangleStrip not supported"))
            }
        }
    }
}
