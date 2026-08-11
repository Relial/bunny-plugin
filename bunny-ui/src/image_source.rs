use std::borrow::Cow;

use abi_stable::std_types::RCowStr;
use anyhow::Result;
use egui::{Context, load::TexturePoll};
use shared::texture::{NamedTexture, SizedTexture};

use crate::load::Bytes;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum ImageSource<'a> {
    Uri(RCowStr<'a>),
    Texture(SizedTexture),
    Bytes { uri: RCowStr<'static>, bytes: Bytes },
}

impl<'a> ImageSource<'a> {
    pub fn from_uri(uri: impl Into<RCowStr<'a>>) -> Self {
        Self::Uri(uri.into())
    }

    pub fn from_bytes(uri: impl Into<RCowStr<'static>>, bytes: impl Into<Bytes>) -> Self {
        Self::Bytes {
            uri: uri.into(),
            bytes: bytes.into(),
        }
    }

    pub(crate) fn convert_to_texture(self, ctx: &Context) -> Result<TexturePoll> {
        let source: egui::ImageSource = self.into();
        let res = source.load(
            ctx,
            egui::TextureOptions::default(),
            egui::SizeHint::default(),
        )?;
        Ok(res)
    }
}

impl<'a> From<ImageSource<'a>> for egui::ImageSource<'a> {
    fn from(value: ImageSource<'a>) -> Self {
        match value {
            ImageSource::Uri(uri) => {
                let uri_cow: Cow<'a, str> = uri.into();
                Self::Uri(uri_cow)
            }
            ImageSource::Texture(texture) => Self::Texture(texture.into()),
            ImageSource::Bytes { uri, bytes } => {
                let uri_cow: Cow<'static, str> = uri.into();
                let bytes = match bytes.into_inner() {
                    abi_stable::std_types::RCow::Borrowed(slice) => {
                        egui::load::Bytes::Static(slice.into())
                    }
                    abi_stable::std_types::RCow::Owned(vec) => {
                        egui::load::Bytes::Shared(vec.to_vec().into())
                    }
                };
                Self::Bytes {
                    uri: uri_cow,
                    bytes,
                }
            }
        }
    }
}

impl<'a> From<&'a str> for ImageSource<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        Self::Uri(value.into())
    }
}

impl<'a> From<&'a String> for ImageSource<'a> {
    #[inline]
    fn from(value: &'a String) -> Self {
        Self::Uri(value.as_str().into())
    }
}

impl From<String> for ImageSource<'static> {
    fn from(value: String) -> Self {
        Self::Uri(value.into())
    }
}

impl<'a> From<&'a Cow<'a, str>> for ImageSource<'a> {
    #[inline]
    fn from(value: &'a Cow<'a, str>) -> Self {
        Self::Uri(value.clone().into())
    }
}

impl<'a> From<Cow<'a, str>> for ImageSource<'a> {
    #[inline]
    fn from(value: Cow<'a, str>) -> Self {
        Self::Uri(value.into())
    }
}

impl<'a> From<RCowStr<'a>> for ImageSource<'a> {
    fn from(value: RCowStr<'a>) -> Self {
        Self::Uri(value)
    }
}

impl<'a> From<&RCowStr<'a>> for ImageSource<'a> {
    fn from(value: &RCowStr<'a>) -> Self {
        Self::Uri(value.clone())
    }
}

impl<T: Into<Bytes>> From<(&'static str, T)> for ImageSource<'static> {
    #[inline]
    fn from((uri, bytes): (&'static str, T)) -> Self {
        Self::Bytes {
            uri: uri.into(),
            bytes: bytes.into(),
        }
    }
}

impl<T: Into<Bytes>> From<(Cow<'static, str>, T)> for ImageSource<'static> {
    #[inline]
    fn from((uri, bytes): (Cow<'static, str>, T)) -> Self {
        Self::Bytes {
            uri: uri.into(),
            bytes: bytes.into(),
        }
    }
}

impl<T: Into<Bytes>> From<(RCowStr<'static>, T)> for ImageSource<'static> {
    #[inline]
    fn from((uri, bytes): (RCowStr<'static>, T)) -> Self {
        Self::Bytes {
            uri,
            bytes: bytes.into(),
        }
    }
}

impl<T: Into<Bytes>> From<(String, T)> for ImageSource<'static> {
    #[inline]
    fn from((uri, bytes): (String, T)) -> Self {
        Self::Bytes {
            uri: uri.into(),
            bytes: bytes.into(),
        }
    }
}

impl From<&NamedTexture> for ImageSource<'_> {
    fn from(value: &NamedTexture) -> Self {
        Self::Texture(*value.texture())
    }
}

impl From<&SizedTexture> for ImageSource<'_> {
    fn from(value: &SizedTexture) -> Self {
        Self::Texture(*value)
    }
}
