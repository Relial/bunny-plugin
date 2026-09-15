use abi_stable::std_types::ROption::{self, RNone, RSome};

#[repr(C)]
pub struct Separator {
    spacing: ROption<f32>,
    grow: f32,
    is_horizontal_line: ROption<bool>,
}

impl Separator {
    #[inline]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = RSome(spacing);
        self
    }

    #[inline]
    pub fn horizontal(mut self) -> Self {
        self.is_horizontal_line = RSome(true);
        self
    }

    #[inline]
    pub fn vertical(mut self) -> Self {
        self.is_horizontal_line = RSome(false);
        self
    }

    #[inline]
    pub fn grow(mut self, grow: f32) -> Self {
        self.grow += grow;
        self
    }

    #[inline]
    pub fn shrink(mut self, shrink: f32) -> Self {
        self.grow -= shrink;
        self
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self {
            spacing: RNone,
            grow: 0.0,
            is_horizontal_line: RNone,
        }
    }
}

#[cfg(feature = "manager")]
impl egui::Widget for Separator {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut separator = egui::Separator::default();
        if let RSome(spacing) = self.spacing {
            separator = separator.spacing(spacing);
        }
        if self.grow > 0.0 {
            separator = separator.grow(self.grow)
        } else if self.grow < 0.0 {
            separator = separator.shrink(self.grow)
        }
        if let RSome(horizontal) = self.is_horizontal_line {
            if horizontal {
                separator = separator.horizontal();
            } else {
                separator = separator.vertical();
            }
        }
        ui.add(separator)
    }
}
