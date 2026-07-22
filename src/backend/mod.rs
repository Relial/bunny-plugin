use anyhow::{Context, Result, anyhow};
use epaint::Color32;
use windows::Win32::Graphics::Direct3D9::{
    D3DPT_TRIANGLELIST, D3DTS_PROJECTION, D3DTS_VIEW, IDirect3DDevice9,
};
use windows_numerics::Matrix4x4;

use crate::{
    backend::{
        mesh::{Buffers, FVF_CUSTOMVERTEX},
        texture_manager::TextureManager,
    },
    core::{Bunny3d, texture::TextureId},
};

mod mesh;
mod texture_manager;

pub const VERTEX_SIZE: u32 = 24;
pub const INDEX_SIZE: u32 = 4;

#[derive(Debug)]
pub struct Bunny3dBackend {
    pub data: Bunny3d,
    buffers: Buffers,
    texture_manager: TextureManager,
    should_reset: bool,
    skip_frame: u8,
}

impl Bunny3dBackend {
    pub fn new(device: &IDirect3DDevice9) -> Result<Self> {
        Ok(Self {
            data: Default::default(),
            buffers: Buffers::create_buffers(device, 16384 * VERTEX_SIZE, 16384 * INDEX_SIZE)
                .context("Failed to create buffers")?,
            texture_manager: TextureManager::new(device)?,
            should_reset: false,
            skip_frame: 5,
        })
    }

    pub fn start_frame(&mut self) {
        self.data.start_frame();
    }

    pub fn allocate_textures(
        &mut self,
        device: &IDirect3DDevice9,
        allocated: &mut Vec<TextureId>,
    ) -> Result<()> {
        let allocation_count = self.data.allocations_len();
        if allocation_count == 0 {
            return Ok(());
        }
        for allocation in self.data.extract_allocations() {
            allocated.push(allocation.id);
            self.texture_manager.allocate(device, allocation)?;
        }
        Ok(())
    }

    pub fn free_texture(&mut self, texture: TextureId) -> bool {
        self.texture_manager.free(texture)
    }

    pub fn draw(
        &mut self,
        device: &IDirect3DDevice9,
        view_matrix: Matrix4x4,
        projection_matrix: Matrix4x4,
    ) -> Result<()> {
        if self.skip_frame > 0 {
            if self.should_reset {
                self.buffers.recreate_buffers(device)?;
                self.texture_manager.reallocate_all(device)?;
            }
            self.skip_frame -= 1;
            return Ok(());
        }

        if self.data.is_empty() {
            return Ok(());
        }

        self.buffers
            .update_vertex_buffer(device, self.data.vertex_buffer())?;
        self.buffers
            .update_index_buffer(device, self.data.index_buffer())?;

        unsafe {
            device
                .SetFVF(FVF_CUSTOMVERTEX)
                .context("Failed to set FVF")?;

            device
                .SetTransform(D3DTS_VIEW, &view_matrix)
                .context("Failed to set view matrix")?;
            device
                .SetTransform(D3DTS_PROJECTION, &projection_matrix)
                .context("Failed to set projection patrix")?;

            let vertex_buffer = self
                .buffers
                .vtx
                .as_ref()
                .ok_or(anyhow!("Failed to get vertex buffer"))?;
            let index_buffer = self
                .buffers
                .idx
                .as_ref()
                .ok_or(anyhow!("Failed to get index buffer"))?;
            device
                .SetStreamSource(0, vertex_buffer, 0, VERTEX_SIZE)
                .context("Failed to set stream source")?;
            device
                .SetIndices(index_buffer)
                .context("Failed to set indices")?;
        }

        let mut current_vtx = 0;
        let mut current_idx = 0;
        for mesh in self.data.meshes() {
            mesh.setup(device)?;

            let texture = self.texture_manager.get(mesh.texture)?;
            unsafe {
                device
                    .SetTexture(0, texture)
                    .context("Failed to set texture")?
            };

            unsafe {
                device
                    .DrawIndexedPrimitive(
                        D3DPT_TRIANGLELIST,
                        current_vtx as i32,
                        0,
                        mesh.vertices as u32,
                        current_idx as u32,
                        (mesh.indices / 3) as u32,
                    )
                    .context("Failed to draw indexed primitive")?;
            }
            current_vtx += mesh.vertices;
            current_idx += mesh.indices;
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        self.buffers.delete_buffers();
        self.texture_manager.deallocate_all();
        self.should_reset = true;
        self.skip_frame = 5;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpuColor([u8; 4]);

impl GpuColor {
    pub fn from_rgba(bytes: &[u8]) -> Self {
        Self([bytes[2], bytes[1], bytes[0], bytes[3]])
    }

    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Color32> for GpuColor {
    fn from(value: Color32) -> Self {
        let cols = value.to_array();
        Self([cols[2], cols[1], cols[0], cols[3]])
    }
}
