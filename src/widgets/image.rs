use abi_stable::std_types::{
    RBox, RCowStr,
    ROption::{self, RNone, RSome},
    RString, Tuple2,
};
pub use egui::Sense;
use egui::{Color32, NumExt, Rect, Vec2, pos2};

use crate::{
    elements::Widget,
    image_source::ImageLoader,
    load::{Bytes, SizeHint},
    paint::{corner_radius::CornerRadius, textures::TextureOptions},
};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Image<'a> {
    image_loader: ImageLoader<'a>,
    texture_options: TextureOptions,
    image_options: ImageOptions,
    sense: Sense,
    size: ImageSize,
    show_loading_spinner: bool,
    alt_text: ROption<RString>,
}

impl<'a> Image<'a> {
    pub fn new(source: impl Into<ImageLoader<'a>>) -> Self {
        Self {
            image_loader: source.into(),
            texture_options: Default::default(),
            image_options: Default::default(),
            sense: Sense::hover(),
            size: Default::default(),
            show_loading_spinner: false,
            alt_text: RNone,
        }
    }

    pub fn from_uri(uri: impl Into<RCowStr<'a>>) -> Self {
        Self::new(ImageLoader::from_uri(uri))
    }

    pub fn from_bytes(uri: impl Into<RCowStr<'static>>, bytes: impl Into<Bytes>) -> Self {
        Self::new(ImageLoader::from_bytes(uri, bytes))
    }

    #[inline]
    pub fn texture_options(mut self, texture_options: TextureOptions) -> Self {
        self.texture_options = texture_options;
        self
    }

    #[inline]
    pub fn max_width(mut self, width: f32) -> Self {
        self.size.max_size.x = width;
        self
    }

    #[inline]
    pub fn max_height(mut self, height: f32) -> Self {
        self.size.max_size.y = height;
        self
    }

    #[inline]
    pub fn max_size(mut self, size: Vec2) -> Self {
        self.size.max_size = size;
        self
    }

    #[inline]
    pub fn maintain_aspect_ratio(mut self, value: bool) -> Self {
        self.size.maintain_aspect_ratio = value;
        self
    }

    #[inline]
    pub fn fit_to_original_size(mut self, scale: f32) -> Self {
        self.size.fit = ImageFit::Original { scale };
        self
    }

    #[inline]
    pub fn fit_to_exact_size(mut self, size: Vec2) -> Self {
        self.size.fit = ImageFit::Exact(size);
        self
    }

    #[inline]
    pub fn fit_to_fraction(mut self, fraction: Vec2) -> Self {
        self.size.fit = ImageFit::Fraction(fraction);
        self
    }

    #[inline]
    pub fn shrink_to_fit(self) -> Self {
        self.fit_to_fraction(Vec2::new(1.0, 1.0))
    }

    #[inline]
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }

    #[inline]
    pub fn uv(mut self, uv: impl Into<Rect>) -> Self {
        self.image_options.uv = uv.into();
        self
    }

    #[inline]
    pub fn bg_fill(mut self, bg_fill: impl Into<Color32>) -> Self {
        self.image_options.bg_fill = bg_fill.into();
        self
    }

    #[inline]
    pub fn tint(mut self, tint: impl Into<Color32>) -> Self {
        self.image_options.tint = tint.into();
        self
    }

    #[inline]
    pub fn rotate(mut self, angle: f32, origin: Vec2) -> Self {
        self.image_options.rotation = RSome(Tuple2(angle, origin));
        self.image_options.corner_radius = CornerRadius::ZERO;
        self
    }

    #[inline]
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.image_options.corner_radius = corner_radius.into();
        if self.image_options.corner_radius != CornerRadius::ZERO {
            self.image_options.rotation = RNone;
        }
        self
    }

    #[inline]
    pub fn show_loading_spinner(mut self, show: bool) -> Self {
        self.show_loading_spinner = show;
        self
    }

    #[inline]
    pub fn alt_text(mut self, label: impl Into<RString>) -> Self {
        self.alt_text = RSome(label.into());
        self
    }
}

impl<'a, T: Into<ImageLoader<'a>>> From<T> for Image<'a> {
    fn from(value: T) -> Self {
        Image::new(value)
    }
}

impl<'a> From<Image<'a>> for Widget<'a> {
    fn from(value: Image<'a>) -> Self {
        Self::Image(RBox::new(value))
    }
}

impl egui::Widget for Image<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Image {
            image_loader,
            texture_options,
            image_options,
            sense,
            size,
            show_loading_spinner,
            alt_text,
        } = self;
        let mut image = egui::Image::new(image_loader.source)
            .texture_options(texture_options.into())
            .uv(image_options.uv)
            .bg_fill(image_options.bg_fill)
            .tint(image_options.tint)
            .corner_radius(image_options.corner_radius)
            .sense(sense)
            .max_size(size.max_size)
            .maintain_aspect_ratio(size.maintain_aspect_ratio)
            .show_loading_spinner(show_loading_spinner);
        if let RSome(Tuple2(angle, origin)) = image_options.rotation {
            image = image.rotate(angle, origin);
        }
        if let RSome(alt_text) = alt_text {
            image = image.alt_text(alt_text);
        }
        image = match size.fit {
            ImageFit::Original { scale } => image.fit_to_original_size(scale),
            ImageFit::Fraction(fraction) => image.fit_to_fraction(fraction),
            ImageFit::Exact(size) => image.fit_to_exact_size(size),
        };

        image.ui(ui)
    }
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
    pub rotation: ROption<Tuple2<f32, Vec2>>,
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
