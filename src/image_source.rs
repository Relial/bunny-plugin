use std::borrow::Cow;

use abi_stable::std_types::{RCowSlice, RCowStr};
use anyhow::Result;
use egui::{Context, load::TexturePoll};

use crate::{load::SizeHint, paint::textures::TextureOptions};

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct ImageLoader<'a> {
    pub source: ImageSource<'a>,
    pub texture_options: TextureOptions,
    pub size_hint: SizeHint,
}

impl<'a> ImageLoader<'a> {
    pub fn to_texture(self, ctx: &Context) -> Result<TexturePoll> {
        let source: egui::ImageSource = self.source.into();
        let res = source.load(ctx, self.texture_options.into(), self.size_hint.into())?;
        Ok(res)
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum ImageSource<'a> {
    Uri(RCowStr<'a>),
    Bytes {
        uri: RCowStr<'static>,
        bytes: RCowSlice<'static, u8>,
    },
}

impl ImageSource<'static> {
    pub fn from_bytes(
        uri: impl Into<RCowStr<'static>>,
        bytes: impl Into<RCowSlice<'static, u8>>,
    ) -> Self {
        Self::Bytes {
            uri: uri.into(),
            bytes: bytes.into(),
        }
    }
}

impl<'a> From<ImageSource<'a>> for egui::ImageSource<'a> {
    fn from(value: ImageSource<'a>) -> Self {
        match value {
            ImageSource::Uri(uri) => {
                let uri_cow: Cow<'a, str> = uri.into();
                Self::Uri(uri_cow)
            }
            ImageSource::Bytes { uri, bytes } => {
                let uri_cow: Cow<'static, str> = uri.into();
                let bytes = match bytes {
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

impl<T: Into<RCowSlice<'static, u8>>> From<(&'static str, T)> for ImageSource<'static> {
    #[inline]
    fn from((uri, bytes): (&'static str, T)) -> Self {
        Self::Bytes {
            uri: uri.into(),
            bytes: bytes.into(),
        }
    }
}

impl<T: Into<RCowSlice<'static, u8>>> From<(Cow<'static, str>, T)> for ImageSource<'static> {
    #[inline]
    fn from((uri, bytes): (Cow<'static, str>, T)) -> Self {
        Self::Bytes {
            uri: uri.into(),
            bytes: bytes.into(),
        }
    }
}

impl<T: Into<RCowSlice<'static, u8>>> From<(String, T)> for ImageSource<'static> {
    #[inline]
    fn from((uri, bytes): (String, T)) -> Self {
        Self::Bytes {
            uri: uri.into(),
            bytes: bytes.into(),
        }
    }
}
