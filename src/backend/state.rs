use anyhow::Result;
use windows::Win32::Graphics::Direct3D9::{
    IDirect3DDevice9, IDirect3DStateBlock9, D3DBLENDOP_ADD, D3DBLEND_INVSRCALPHA, D3DBLEND_ONE,
    D3DCULL_NONE, D3DRS_ALPHABLENDENABLE, D3DRS_ALPHATESTENABLE, D3DRS_BLENDOP, D3DRS_BLENDOPALPHA,
    D3DRS_CLIPPING, D3DRS_COLORWRITEENABLE, D3DRS_CULLMODE, D3DRS_DESTBLEND, D3DRS_DESTBLENDALPHA,
    D3DRS_FOGENABLE, D3DRS_LASTPIXEL, D3DRS_LIGHTING, D3DRS_RANGEFOGENABLE,
    D3DRS_SCISSORTESTENABLE, D3DRS_SEPARATEALPHABLENDENABLE, D3DRS_SHADEMODE, D3DRS_SPECULARENABLE,
    D3DRS_SRCBLEND, D3DRS_SRCBLENDALPHA, D3DRS_SRGBWRITEENABLE, D3DRS_STENCILENABLE,
    D3DRS_TEXTUREFACTOR, D3DRS_ZENABLE, D3DRS_ZWRITEENABLE, D3DSAMP_ADDRESSU, D3DSAMP_ADDRESSV,
    D3DSAMP_ADDRESSW, D3DSAMP_BORDERCOLOR, D3DSAMP_MAGFILTER, D3DSAMP_MINFILTER, D3DSAMP_MIPFILTER,
    D3DSBT_ALL, D3DSHADE_GOURAUD, D3DTADDRESS_CLAMP, D3DTA_CURRENT, D3DTA_DIFFUSE, D3DTA_TEXTURE,
    D3DTEXF_LINEAR, D3DTOP_DISABLE, D3DTOP_MODULATE, D3DTRANSFORMSTATETYPE, D3DTSS_ALPHAARG0,
    D3DTSS_ALPHAARG1, D3DTSS_ALPHAARG2, D3DTSS_ALPHAOP, D3DTSS_COLORARG0, D3DTSS_COLORARG1,
    D3DTSS_COLORARG2, D3DTSS_COLOROP,
};
use windows_numerics::Matrix4x4;

use crate::backend::mesh::FVF_CUSTOMVERTEX;

pub struct DxState {
    original_state: IDirect3DStateBlock9,
    original_world: Matrix4x4,
    device: IDirect3DDevice9,
}

impl DxState {
    pub fn setup(device: &IDirect3DDevice9) -> Self {
        unsafe {
            let original_state = device.CreateStateBlock(D3DSBT_ALL).unwrap();
            original_state.Capture().unwrap();
            let mut original_world: Matrix4x4 = Default::default();
            device
                .GetTransform(D3DTRANSFORMSTATETYPE(256), &mut original_world)
                .unwrap();
            setup_state(device).unwrap();
            Self {
                original_state,
                original_world,
                device: device.clone(),
            }
        }
    }
}

impl Drop for DxState {
    fn drop(&mut self) {
        unsafe {
            self.device
                .SetTransform(D3DTRANSFORMSTATETYPE(256), &self.original_world)
                .unwrap();
            self.original_state.Apply().unwrap();
        }
    }
}

fn setup_state(device: &IDirect3DDevice9) -> Result<()> {
    unsafe {
        // set up fvf
        device.SetFVF(FVF_CUSTOMVERTEX)?;
        device.SetPixelShader(None)?;
        device.SetVertexShader(None)?;

        // set up render state
        device.SetRenderState(D3DRS_SCISSORTESTENABLE, false as u32)?;
        device.SetRenderState(D3DRS_ZENABLE, true as u32)?;
        device.SetRenderState(D3DRS_ZWRITEENABLE, true as u32)?;
        device.SetRenderState(D3DRS_SHADEMODE, D3DSHADE_GOURAUD.0 as _)?;
        device.SetRenderState(D3DRS_ALPHATESTENABLE, false as _)?;
        device.SetRenderState(D3DRS_CULLMODE, D3DCULL_NONE.0 as _)?;
        device.SetRenderState(D3DRS_ALPHABLENDENABLE, true as _)?;
        device.SetRenderState(D3DRS_BLENDOP, D3DBLENDOP_ADD.0 as _)?;
        device.SetRenderState(D3DRS_SRCBLEND, D3DBLEND_ONE.0 as _)?;
        device.SetRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA.0 as _)?;
        device.SetRenderState(D3DRS_SEPARATEALPHABLENDENABLE, true as _)?;
        device.SetRenderState(D3DRS_BLENDOPALPHA, D3DBLENDOP_ADD.0 as _)?;
        device.SetRenderState(D3DRS_SRCBLENDALPHA, D3DBLEND_ONE.0 as _)?;
        device.SetRenderState(D3DRS_DESTBLENDALPHA, D3DBLEND_INVSRCALPHA.0 as _)?;
        device.SetRenderState(D3DRS_FOGENABLE, false as _)?;
        device.SetRenderState(D3DRS_RANGEFOGENABLE, false as _)?;
        device.SetRenderState(D3DRS_SPECULARENABLE, false as _)?;
        device.SetRenderState(D3DRS_STENCILENABLE, false as _)?;
        device.SetRenderState(D3DRS_CLIPPING, true as _)?;
        device.SetRenderState(D3DRS_LIGHTING, false as _)?;
        device.SetRenderState(D3DRS_TEXTUREFACTOR, 0xFFFFFFFF)?;
        device.SetRenderState(D3DRS_COLORWRITEENABLE, 0xFFFFFFFF)?;
        device.SetRenderState(D3DRS_SRGBWRITEENABLE, false as _)?;
        device.SetRenderState(D3DRS_LASTPIXEL, true as _)?;

        // set up texture stages
        device.SetTextureStageState(0, D3DTSS_COLOROP, D3DTOP_MODULATE.0 as _)?;
        device.SetTextureStageState(0, D3DTSS_COLORARG0, D3DTA_CURRENT)?;
        device.SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TEXTURE)?;
        device.SetTextureStageState(0, D3DTSS_COLORARG2, D3DTA_DIFFUSE)?;
        device.SetTextureStageState(0, D3DTSS_ALPHAOP, D3DTOP_MODULATE.0 as _)?;
        device.SetTextureStageState(0, D3DTSS_ALPHAARG0, D3DTA_CURRENT)?;
        device.SetTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_TEXTURE)?;
        device.SetTextureStageState(0, D3DTSS_ALPHAARG2, D3DTA_DIFFUSE)?;

        device.SetTextureStageState(1, D3DTSS_COLOROP, D3DTOP_DISABLE.0 as _)?;
        device.SetTextureStageState(1, D3DTSS_ALPHAOP, D3DTOP_DISABLE.0 as _)?;

        device.SetTextureStageState(2, D3DTSS_COLOROP, D3DTOP_DISABLE.0 as _)?;
        device.SetTextureStageState(2, D3DTSS_ALPHAOP, D3DTOP_DISABLE.0 as _)?;

        // set up sampler
        device.SetSamplerState(0, D3DSAMP_MINFILTER, D3DTEXF_LINEAR.0 as _)?;
        device.SetSamplerState(0, D3DSAMP_MIPFILTER, D3DTEXF_LINEAR.0 as _)?;
        device.SetSamplerState(0, D3DSAMP_MAGFILTER, D3DTEXF_LINEAR.0 as _)?;
        device.SetSamplerState(0, D3DSAMP_BORDERCOLOR, 0xFFFFFFFF)?;
        device.SetSamplerState(0, D3DSAMP_ADDRESSU, D3DTADDRESS_CLAMP.0 as _)?;
        device.SetSamplerState(0, D3DSAMP_ADDRESSV, D3DTADDRESS_CLAMP.0 as _)?;
        device.SetSamplerState(0, D3DSAMP_ADDRESSW, D3DTADDRESS_CLAMP.0 as _)?;

        Ok(())
    }
}
