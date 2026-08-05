use std::ffi::c_void;

use anyhow::{Context, Result, anyhow, bail};
use windows::Win32::{
    Foundation::HANDLE,
    Graphics::Direct3D9::{
        D3DFMT_INDEX32, D3DFVF_DIFFUSE, D3DFVF_TEX1, D3DFVF_XYZ, D3DLOCK_DISCARD, D3DPOOL_DEFAULT,
        D3DUSAGE_DYNAMIC, D3DUSAGE_WRITEONLY, IDirect3DDevice9, IDirect3DIndexBuffer9,
        IDirect3DVertexBuffer9,
    },
};

use crate::backend::{INDEX_SIZE, VERTEX_SIZE};

pub const FVF_CUSTOMVERTEX: u32 = D3DFVF_XYZ | D3DFVF_DIFFUSE | D3DFVF_TEX1;

#[derive(Debug)]
pub struct Buffers {
    pub vtx: Option<IDirect3DVertexBuffer9>,
    pub idx: Option<IDirect3DIndexBuffer9>,
    vtx_size: u32,
    idx_size: u32,
}

impl Buffers {
    pub fn create_buffers(
        device: &IDirect3DDevice9,
        vtx_size: u32,
        idx_size: u32,
    ) -> Result<Buffers> {
        Ok(Buffers {
            vtx_size,
            idx_size,
            vtx: Some(create_vertex_buffer(device, vtx_size)?),
            idx: Some(create_index_buffer(device, idx_size)?),
        })
    }

    pub fn delete_buffers(&mut self) {
        self.vtx = None;
        self.idx = None;
    }

    pub fn recreate_buffers(&mut self, device: &IDirect3DDevice9) -> Result<()> {
        if self.vtx_size == 0 || self.idx_size == 0 {
            bail!("No buffers were created before recreate_buffers call");
        }
        if self.vtx.is_some() || self.idx.is_some() {
            bail!("Buffers already created");
        }
        self.vtx = Some(create_vertex_buffer(device, self.vtx_size)?);
        self.idx = Some(create_index_buffer(device, self.idx_size)?);

        Ok(())
    }

    pub fn update_vertex_buffer(
        &mut self,
        device: &IDirect3DDevice9,
        vertices: &[u8],
    ) -> Result<()> {
        unsafe {
            let buf_len = vertices.len() as u32;

            if self.vtx_size < buf_len {
                let new_size = buf_len + 1024 * VERTEX_SIZE;
                self.vtx = Some(create_vertex_buffer(device, new_size)?);
                self.vtx_size = new_size;
            }

            let vtx = self
                .vtx
                .as_mut()
                .ok_or(anyhow!("unable to get vertex buffer"))?;

            let mut buffer: *mut u8 = std::mem::zeroed();

            vtx.Lock(
                0,
                buf_len,
                (&raw mut buffer) as *mut *mut c_void,
                D3DLOCK_DISCARD as u32,
            )
            .context("unable to lock vertex buffer")?;

            let buffer = std::slice::from_raw_parts_mut(buffer, buf_len as usize);

            buffer.copy_from_slice(vertices);

            vtx.Unlock().context("unable to unlock vtx buffer")?;
            Ok(())
        }
    }

    pub fn update_index_buffer(&mut self, device: &IDirect3DDevice9, indices: &[u8]) -> Result<()> {
        unsafe {
            let buf_len = indices.len() as u32;

            if self.idx_size < buf_len {
                let new_size = buf_len + 1024 * INDEX_SIZE;
                self.idx = Some(create_index_buffer(device, new_size)?);
                self.idx_size = new_size;
            }

            let idx = self.idx.as_mut().context("unable to get index buffer")?;

            let mut buffer: *mut u8 = std::mem::zeroed();

            idx.Lock(
                0,
                buf_len,
                &mut buffer as *mut *mut u8 as *mut *mut c_void,
                D3DLOCK_DISCARD as u32,
            )
            .context("unable to lock index buffer")?;

            let buffer = std::slice::from_raw_parts_mut(buffer, buf_len as usize);

            buffer.copy_from_slice(indices);

            idx.Unlock().context("unable to unlock idx buffer")?;
            Ok(())
        }
    }
}

fn create_vertex_buffer(device: &IDirect3DDevice9, size: u32) -> Result<IDirect3DVertexBuffer9> {
    unsafe {
        let mut vertex_buffer: Option<IDirect3DVertexBuffer9> = None;
        device
            .CreateVertexBuffer(
                size,
                (D3DUSAGE_DYNAMIC | D3DUSAGE_WRITEONLY) as u32,
                FVF_CUSTOMVERTEX,
                D3DPOOL_DEFAULT,
                &mut vertex_buffer,
                std::ptr::null_mut::<HANDLE>(),
            )
            .context("Failed to create vertex buffer")?;

        vertex_buffer.ok_or(anyhow!("Failed to create vertex buffer"))
    }
}

fn create_index_buffer(device: &IDirect3DDevice9, size: u32) -> Result<IDirect3DIndexBuffer9> {
    unsafe {
        let mut index_buffer: Option<IDirect3DIndexBuffer9> = None;
        device
            .CreateIndexBuffer(
                size,
                (D3DUSAGE_DYNAMIC | D3DUSAGE_WRITEONLY) as u32,
                D3DFMT_INDEX32,
                D3DPOOL_DEFAULT,
                &mut index_buffer,
                std::ptr::null_mut::<HANDLE>(),
            )
            .context("Failed to create index buffer")?;

        index_buffer.ok_or(anyhow!("Failed to create index buffer"))
    }
}
