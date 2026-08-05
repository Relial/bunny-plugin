use abi_stable::std_types::RVec;
use anyhow::{Context as _, Result};
use glam::Mat4;
use windows::Win32::Graphics::Direct3D9::{
    D3DRS_FILLMODE, D3DTRANSFORMSTATETYPE, IDirect3DDevice9,
};
use windows_numerics::Matrix4x4;

use crate::{
    backend::VERTEX_SIZE,
    core::{Bunny3dComponent, FillMode, texture::TextureId},
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

    pub fn add(&mut self, component: &Bunny3dComponent) {
        let indices_count = component.index_count();
        let indices = component.mesh.index_buffer_bytes();
        self.index_bytes.extend_from_slice(indices);

        let vertex_size = VERTEX_SIZE as usize;
        let vertex_count = component.vertex_count();
        let vertex_size_required = vertex_size * vertex_count;
        if self.vertex_bytes.len() - self.vertex_bytes_position < vertex_size_required {
            self.vertex_bytes
                .extend(std::iter::repeat_n(0, vertex_size_required));
        }
        component.mesh.update_vertex_buffer(
            &mut self.vertex_bytes
                [self.vertex_bytes_position..self.vertex_bytes_position + vertex_size_required],
            vertex_size,
        );

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
            fill: component.fill,
            world_matrix: d3dmat,
            texture: component.texture.unwrap_or_default(),
        });
        self.vertex_bytes_position += vertex_size_required;
    }
}

impl DrawList {
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
}

#[derive(Debug)]
#[repr(C)]
pub struct MeshDescriptor {
    world_matrix: Matrix4x4,
    pub(crate) vertices: usize,
    pub(crate) indices: usize,
    fill: FillMode,
    pub(crate) texture: TextureId,
}

impl MeshDescriptor {
    pub(crate) fn setup(&self, device: &IDirect3DDevice9) -> Result<()> {
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
