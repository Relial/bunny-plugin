pub mod camera;
pub mod fonts;
pub mod texture;
pub use glam;

use crate::{
    camera::Camera,
    fonts::{CustomFont, CustomFonts, NamedCustomFont},
    texture::{SharedSizedTexture, SharedTexture, SharedTextures},
};

#[derive(Debug)]
#[repr(C)]
pub struct BunnyContext {
    camera: Camera,
    fonts: CustomFonts,
    textures: SharedTextures,
}

impl BunnyContext {
    /// Information about the game camera
    #[inline]
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Get a custom font loaded by the manager by its filename without extension
    ///
    /// Fonts are loaded at startup, so you can expect them to be available immediately
    #[inline]
    pub fn get_font(&self, name: impl AsRef<str>) -> Option<CustomFont> {
        self.fonts.get_font(name)
    }

    /// Custom fonts loaded by the manager
    ///
    /// Fonts are loaded at startup, so you can expect them to be available immediately
    #[inline]
    pub fn fonts(&self) -> &[NamedCustomFont] {
        self.fonts.fonts()
    }

    /// Get a texture loaded by the manager by its filename
    ///
    /// The textures are loaded asynchronously, so you should not assume this returns what you want at startup
    #[inline]
    pub fn get_texture(&self, name: impl AsRef<str>) -> Option<SharedSizedTexture> {
        self.textures.get_texture(name)
    }

    /// Textures loaded by the manager
    ///
    /// The textures are loaded asynchronously, so you should not assume this returns what you want at startup
    #[inline]
    pub fn textures(&self) -> &[SharedTexture] {
        self.textures.textures()
    }
}

#[cfg(feature = "manager")]
impl BunnyContext {
    pub fn new(camera: Camera, fonts: CustomFonts, textures: SharedTextures) -> Self {
        Self {
            camera,
            fonts,
            textures,
        }
    }

    #[inline]
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    #[inline]
    pub fn set_textures(&mut self, textures: SharedTextures) {
        self.textures = textures;
    }
}
