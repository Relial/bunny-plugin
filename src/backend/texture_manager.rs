use abi_stable::rvec;
use anyhow::{Context, Result, anyhow};
use epaint::Color32;
use rapidhash::RapidHashMap;
use windows::Win32::Graphics::Direct3D9::{
    D3DFMT_A8R8G8B8, D3DLOCKED_RECT, D3DPOOL_DEFAULT, D3DUSAGE_DYNAMIC, IDirect3DDevice9,
    IDirect3DTexture9,
};

use crate::{
    backend::GpuColor,
    core::texture::{TextureAllocation, TextureId},
};

#[derive(Debug)]
pub struct TextureManager {
    textures: RapidHashMap<TextureId, Texture>,
    shared: RapidHashMap<TextureId, IDirect3DTexture9>,
}

impl TextureManager {
    pub fn new(device: &IDirect3DDevice9) -> Result<Self> {
        let mut t = Self {
            textures: Default::default(),
            shared: Default::default(),
        };
        t.allocate(
            device,
            TextureAllocation {
                pixels: rvec![Color32::WHITE.into()],
                size: [1, 1],
                id: TextureId::Managed(0),
            },
        )?;
        Ok(t)
    }

    pub fn allocate(
        &mut self,
        device: &IDirect3DDevice9,
        allocation: TextureAllocation,
    ) -> Result<()> {
        let resource = allocate_texture(device, &allocation.pixels, allocation.size)?;
        let managed = Texture {
            resource: Some(resource),
            pixels: allocation.pixels.into(),
            size: allocation.size,
        };
        self.textures.insert(allocation.id, managed);
        Ok(())
    }

    pub fn free(&mut self, texture: TextureId) -> bool {
        self.textures.remove(&texture).is_some()
    }

    pub fn get(&self, id: TextureId) -> Result<&IDirect3DTexture9> {
        match id {
            TextureId::Managed(_) => self
                .textures
                .get(&id)
                .ok_or_else(|| anyhow!("Texture {} doesn't exist", id))?
                .resource
                .as_ref()
                .ok_or_else(|| anyhow!("Tried to get texture {} when it was deallocated", id)),
            TextureId::Shared(_) => self
                .shared
                .get(&id)
                .ok_or_else(|| anyhow!("Texture {} doesn't exist", id)),
        }
    }

    pub fn deallocate_all(&mut self) {
        for t in self.textures.values_mut() {
            t.resource = None;
        }
        self.shared.clear();
    }

    pub fn reallocate_all(&mut self, device: &IDirect3DDevice9) -> Result<()> {
        for t in self.textures.values_mut() {
            let res = allocate_texture(device, &t.pixels, t.size)?;
            t.resource = Some(res);
        }
        Ok(())
    }

    pub fn add_shared(&mut self, textures: impl IntoIterator<Item = (TextureId, IDirect3DTexture9)>) {
        self.shared.extend(textures);
    }
}

#[derive(Debug)]
pub struct Texture {
    resource: Option<IDirect3DTexture9>,
    pixels: Vec<GpuColor>,
    size: [u32; 2],
}

fn allocate_texture(
    device: &IDirect3DDevice9,
    pixels: &[GpuColor],
    size: [u32; 2],
) -> Result<IDirect3DTexture9> {
    unsafe {
        let mut tex: Option<IDirect3DTexture9> = None;
        device
            .CreateTexture(
                size[0],
                size[1],
                1,
                D3DUSAGE_DYNAMIC as u32,
                D3DFMT_A8R8G8B8,
                D3DPOOL_DEFAULT,
                &mut tex,
                std::ptr::null_mut(),
            )
            .context("Failed to create texture")?;
        let mut r: D3DLOCKED_RECT = Default::default();
        let tex = tex.ok_or(anyhow!("Failed to create texture"))?;
        tex.LockRect(0, &mut r, std::ptr::null_mut(), 0)
            .context("Failed to lock rect")?;
        std::slice::from_raw_parts_mut(r.pBits as *mut GpuColor, pixels.len())
            .copy_from_slice(pixels);
        tex.UnlockRect(0).context("Failed to unlock rect")?;
        Ok(tex)
    }
}
