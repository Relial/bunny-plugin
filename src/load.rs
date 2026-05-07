use egui::emath::Float;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SizeHint {
    Scale(f32),
    Width(u32),
    Height(u32),
    Size {
        width: u32,
        height: u32,
        maintain_aspect_ratio: bool,
    },
}

impl Default for SizeHint {
    fn default() -> Self {
        Self::Scale(1.0)
    }
}

impl SizeHint {
    pub fn scale_by(self, factor: f32) -> Self {
        match self {
            SizeHint::Scale(scale) => Self::Scale(factor * scale),
            SizeHint::Width(width) => Self::Width((factor * width as f32).round() as _),
            SizeHint::Height(height) => Self::Height((factor * height as f32).round() as _),
            SizeHint::Size {
                width,
                height,
                maintain_aspect_ratio,
            } => Self::Size {
                width: (factor * width as f32).round() as _,
                height: (factor * height as f32).round() as _,
                maintain_aspect_ratio,
            },
        }
    }
}

impl From<SizeHint> for egui::SizeHint {
    #[inline(always)]
    fn from(value: SizeHint) -> Self {
        match value {
            SizeHint::Scale(scale) => Self::Scale(scale.ord()),
            SizeHint::Width(width) => Self::Width(width),
            SizeHint::Height(height) => Self::Height(height),
            SizeHint::Size {
                width,
                height,
                maintain_aspect_ratio,
            } => Self::Size {
                width,
                height,
                maintain_aspect_ratio,
            },
        }
    }
}
