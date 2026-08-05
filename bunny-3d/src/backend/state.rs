use anyhow::{Context, Result, anyhow};
use windows::Win32::Graphics::Direct3D9::{
    D3DBLEND_INVSRCALPHA, D3DBLEND_ONE, D3DBLENDOP_ADD, D3DCULL_NONE, D3DRS_ALPHABLENDENABLE,
    D3DRS_ALPHATESTENABLE, D3DRS_BLENDOP, D3DRS_BLENDOPALPHA, D3DRS_CLIPPING,
    D3DRS_COLORWRITEENABLE, D3DRS_CULLMODE, D3DRS_DESTBLEND, D3DRS_DESTBLENDALPHA, D3DRS_FOGENABLE,
    D3DRS_LASTPIXEL, D3DRS_LIGHTING, D3DRS_RANGEFOGENABLE, D3DRS_SCISSORTESTENABLE,
    D3DRS_SEPARATEALPHABLENDENABLE, D3DRS_SHADEMODE, D3DRS_SPECULARENABLE, D3DRS_SRCBLEND,
    D3DRS_SRCBLENDALPHA, D3DRS_SRGBWRITEENABLE, D3DRS_STENCILENABLE, D3DRS_TEXTUREFACTOR,
    D3DRS_ZENABLE, D3DRS_ZWRITEENABLE, D3DSAMP_ADDRESSU, D3DSAMP_ADDRESSV, D3DSAMP_ADDRESSW,
    D3DSAMP_BORDERCOLOR, D3DSAMP_MAGFILTER, D3DSAMP_MINFILTER, D3DSAMP_MIPFILTER, D3DSBT_ALL,
    D3DSHADE_GOURAUD, D3DTA_CURRENT, D3DTA_DIFFUSE, D3DTA_TEXTURE, D3DTADDRESS_CLAMP,
    D3DTEXF_LINEAR, D3DTOP_DISABLE, D3DTOP_MODULATE, D3DTSS_ALPHAARG0, D3DTSS_ALPHAARG1,
    D3DTSS_ALPHAARG2, D3DTSS_ALPHAOP, D3DTSS_COLORARG0, D3DTSS_COLORARG1, D3DTSS_COLORARG2,
    D3DTSS_COLOROP, IDirect3DDevice9, IDirect3DStateBlock9,
};

use crate::backend::mesh::FVF_CUSTOMVERTEX;

#[derive(Default, Debug)]
pub struct GpuState {
    state: Option<IDirect3DStateBlock9>,
    game_state: Option<IDirect3DStateBlock9>,
}

impl GpuState {
    pub fn backup(&mut self, device: &IDirect3DDevice9) -> Result<()> {
        unsafe {
            if let Some(game_state) = &self.game_state {
                game_state
                    .Capture()
                    .context("Failed to capture state block")?;
            } else {
                let state = device
                    .CreateStateBlock(D3DSBT_ALL)
                    .context("Failed to create game state block")?;
                state.Capture().context("Failed to capture state block")?;
                self.game_state = Some(state);
            }
        }
        Ok(())
    }

    pub fn setup(&mut self, device: &IDirect3DDevice9, z_buffer: bool) -> Result<()> {
        unsafe {
            if let Some(new_state) = &self.state {
                new_state.Apply().context("Failed to apply saved state")?;
            } else {
                self.state = Some(setup_state_block(device)?);
            }

            device
                .SetRenderState(D3DRS_ZENABLE, z_buffer as u32)
                .context("Failed to set ZENABLE")?;
            device
                .SetRenderState(D3DRS_ZWRITEENABLE, z_buffer as u32)
                .context("Failed to set ZWRITEENABLE")?;
        }

        Ok(())
    }

    pub fn restore(&mut self) -> Result<()> {
        let saved_state = self
            .game_state
            .as_ref()
            .ok_or(anyhow!("No game state block saved"))?;
        unsafe {
            saved_state
                .Apply()
                .context("Failed to apply saved game state")?;
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        self.state = None;
        self.game_state = None;
    }
}

fn setup_state_block(device: &IDirect3DDevice9) -> Result<IDirect3DStateBlock9> {
    unsafe {
        device
            .BeginStateBlock()
            .context("Failed to begin state block")?;

        // set up fvf
        device.SetFVF(FVF_CUSTOMVERTEX)?;
        device.SetPixelShader(None)?;
        device.SetVertexShader(None)?;

        // set up render state
        device.SetRenderState(D3DRS_SCISSORTESTENABLE, false as u32)?;
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

        device
            .EndStateBlock()
            .context("Failed to finish state block")
    }
}
