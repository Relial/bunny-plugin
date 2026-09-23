#[cfg(feature = "manager")]
use std::sync::Arc;

use abi_stable::std_types::{RHashMap, RString, RVec};
use rapidhash::fast::RandomState;

#[derive(Debug)]
#[repr(C)]
pub struct CustomFonts(CustomFontsImpl);

impl CustomFonts {
    #[cfg(feature = "manager")]
    pub fn new(fonts: impl IntoIterator<Item = (RString, CustomFont)>) -> Self {
        Self(CustomFontsImpl::new(fonts))
    }

    /// Get a font loaded by the manager by its filename without the extension
    #[inline]
    pub fn get_font(&self, name: impl AsRef<str>) -> Option<CustomFont> {
        self.0.get_font(name)
    }

    /// Fonts loaded by the manager
    #[inline]
    pub fn fonts(&self) -> &[NamedCustomFont] {
        self.0.fonts()
    }
}

#[derive(Debug)]
#[repr(C)]
struct CustomFontsImpl {
    list: RVec<NamedCustomFont>,
    map: RHashMap<RString, CustomFont, RandomState>,
}

impl CustomFontsImpl {
    #[cfg(feature = "manager")]
    pub fn new(fonts: impl IntoIterator<Item = (RString, CustomFont)>) -> Self {
        let (map, list) = fonts
            .into_iter()
            .map(|(name, font)| ((name.clone(), font), NamedCustomFont::new(name, font)))
            .unzip();
        Self { list, map }
    }

    #[inline]
    pub fn get_font(&self, name: impl AsRef<str>) -> Option<CustomFont> {
        self.map.get(name.as_ref()).copied()
    }

    #[inline]
    pub fn fonts(&self) -> &[NamedCustomFont] {
        self.list.as_slice()
    }
}

/// A font loaded by the manager
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct CustomFont {
    arc_ptr: *const str,
}

#[cfg(feature = "manager")]
impl CustomFont {
    pub fn new(name: Arc<str>) -> Self {
        let arc_ptr = Arc::into_raw(name);
        Self { arc_ptr }
    }

    #[cfg(feature = "manager")]
    pub fn into_name(self) -> Arc<str> {
        unsafe {
            Arc::increment_strong_count(self.arc_ptr);
            Arc::from_raw(self.arc_ptr)
        }
    }
}

/// A font loaded by the manager
#[derive(Clone, Debug)]
#[repr(C)]
pub struct NamedCustomFont {
    name: RString,
    font: CustomFont,
}

impl NamedCustomFont {
    #[cfg(feature = "manager")]
    fn new(name: impl Into<RString>, font: CustomFont) -> Self {
        Self {
            name: name.into(),
            font,
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    #[inline]
    pub fn font(&self) -> CustomFont {
        self.font
    }
}
