use abi_stable::std_types::{
    ROption::{self, RNone},
    RString, Tuple2,
};
use egui::{Color32, NumExt, Rect, Sense, Vec2, emath::Rot2, pos2};

use crate::{
    image_source::ImageSource,
    load::SizeHint,
    paint::{corner_radius::CornerRadius, textures::TextureOptions},
};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Image<'a> {
    source: ImageSource<'a>,
    texture_options: TextureOptions,
    image_options: ImageOptions,
    sense: Sense,
    size: ImageSize,
    show_loading_spinner: ROption<bool>,
    alt_text: ROption<RString>,
}

impl<'a> Image<'a> {
    
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ImageSize {
    pub maintain_aspect_ratio: bool,
    pub max_size: Vec2,
    pub fit: ImageFit,
}

impl Default for ImageSize {
    #[inline]
    fn default() -> Self {
        Self {
            maintain_aspect_ratio: true,
            max_size: Vec2::INFINITY,
            fit: ImageFit::Fraction(Vec2::new(1.0, 1.0)),
        }
    }
}

impl ImageSize {
    pub fn hint(&self, available_size: Vec2, pixels_per_point: f32) -> SizeHint {
        let Self {
            maintain_aspect_ratio,
            max_size,
            fit,
        } = *self;
        let point_size = match fit {
            ImageFit::Original { scale } => return SizeHint::Scale(pixels_per_point * scale),
            ImageFit::Fraction(fract) => available_size * fract,
            ImageFit::Exact(size) => size,
        };
        let point_size = point_size.at_most(max_size);

        let pixel_size = pixels_per_point * point_size;

        match (pixel_size.x.is_finite(), pixel_size.y.is_finite()) {
            (true, true) => SizeHint::Size {
                width: pixel_size.x.round() as u32,
                height: pixel_size.y.round() as u32,
                maintain_aspect_ratio,
            },
            (true, false) => SizeHint::Width(pixel_size.x.round() as u32),
            (false, true) => SizeHint::Height(pixel_size.y.round() as u32),
            (false, false) => SizeHint::Scale(pixels_per_point),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum ImageFit {
    Original { scale: f32 },
    Fraction(Vec2),
    Exact(Vec2),
}

impl ImageFit {
    pub fn resolve(self, available_size: Vec2, image_size: Vec2) -> Vec2 {
        match self {
            ImageFit::Original { scale } => image_size * scale,
            ImageFit::Fraction(vec2) => available_size * vec2,
            ImageFit::Exact(vec2) => vec2,
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImageOptions {
    pub uv: Rect,
    pub bg_fill: Color32,
    pub tint: Color32,
    pub rotation: ROption<Tuple2<Rot2, Vec2>>,
    pub corner_radius: CornerRadius,
}

impl Default for ImageOptions {
    fn default() -> Self {
        Self {
            uv: Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            bg_fill: Default::default(),
            tint: Color32::WHITE,
            rotation: RNone,
            corner_radius: CornerRadius::ZERO,
        }
    }
}
