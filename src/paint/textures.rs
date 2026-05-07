use abi_stable::std_types::ROption::{self, RNone};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextureOptions {
    pub magnification: TextureFilter,
    pub minification: TextureFilter,
    pub wrap_mode: TextureWrapMode,
    pub mipmap_mode: ROption<TextureFilter>,
}

impl TextureOptions {
    /// Linear magnification and minification.
    pub const LINEAR: Self = Self {
        magnification: TextureFilter::Linear,
        minification: TextureFilter::Linear,
        wrap_mode: TextureWrapMode::ClampToEdge,
        mipmap_mode: RNone,
    };

    /// Nearest magnification and minification.
    pub const NEAREST: Self = Self {
        magnification: TextureFilter::Nearest,
        minification: TextureFilter::Nearest,
        wrap_mode: TextureWrapMode::ClampToEdge,
        mipmap_mode: RNone,
    };

    /// Linear magnification and minification, but with the texture repeated.
    pub const LINEAR_REPEAT: Self = Self {
        magnification: TextureFilter::Linear,
        minification: TextureFilter::Linear,
        wrap_mode: TextureWrapMode::Repeat,
        mipmap_mode: RNone,
    };

    /// Linear magnification and minification, but with the texture mirrored and repeated.
    pub const LINEAR_MIRRORED_REPEAT: Self = Self {
        magnification: TextureFilter::Linear,
        minification: TextureFilter::Linear,
        wrap_mode: TextureWrapMode::MirroredRepeat,
        mipmap_mode: RNone,
    };

    /// Nearest magnification and minification, but with the texture repeated.
    pub const NEAREST_REPEAT: Self = Self {
        magnification: TextureFilter::Nearest,
        minification: TextureFilter::Nearest,
        wrap_mode: TextureWrapMode::Repeat,
        mipmap_mode: RNone,
    };

    /// Nearest magnification and minification, but with the texture mirrored and repeated.
    pub const NEAREST_MIRRORED_REPEAT: Self = Self {
        magnification: TextureFilter::Nearest,
        minification: TextureFilter::Nearest,
        wrap_mode: TextureWrapMode::MirroredRepeat,
        mipmap_mode: RNone,
    };

    pub const fn with_mipmap_mode(self, mipmap_mode: ROption<TextureFilter>) -> Self {
        Self {
            mipmap_mode,
            ..self
        }
    }
}

impl From<TextureOptions> for egui::TextureOptions {
    #[inline(always)]
    fn from(value: TextureOptions) -> Self {
        let TextureOptions {
            magnification,
            minification,
            wrap_mode,
            mipmap_mode,
        } = value;
        Self {
            magnification: magnification.into(),
            minification: minification.into(),
            wrap_mode: wrap_mode.into(),
            mipmap_mode: mipmap_mode.map(|m| m.into()).into(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureFilter {
    Nearest,
    Linear,
}

impl From<TextureFilter> for egui::TextureFilter {
    #[inline(always)]
    fn from(value: TextureFilter) -> Self {
        match value {
            TextureFilter::Nearest => Self::Nearest,
            TextureFilter::Linear => Self::Linear,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextureWrapMode {
    #[default]
    ClampToEdge,
    Repeat,
    MirroredRepeat,
}

impl From<TextureWrapMode> for egui::TextureWrapMode {
    #[inline(always)]
    fn from(value: TextureWrapMode) -> Self {
        match value {
            TextureWrapMode::ClampToEdge => Self::ClampToEdge,
            TextureWrapMode::Repeat => Self::Repeat,
            TextureWrapMode::MirroredRepeat => Self::MirroredRepeat,
        }
    }
}
