use abi_stable::std_types::ROption::{self, RSome};

#[cfg(feature = "manager")]
use crate::closure::PluginClosure;
use crate::{BunnyUi, paint::TextWrapMode};

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
enum SidesKind {
    #[default]
    Extend,
    ShrinkLeft,
    ShrinkRight,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Sides {
    wrap_mode: ROption<TextWrapMode>,
    height: ROption<f32>,
    spacing: ROption<f32>,
    kind: SidesKind,
}

impl Sides {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn height(mut self, height: f32) -> Self {
        self.height = RSome(height);
        self
    }

    #[inline]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = RSome(spacing);
        self
    }

    #[inline]
    pub fn shrink_left(mut self) -> Self {
        self.kind = SidesKind::ShrinkLeft;
        self
    }

    #[inline]
    pub fn shrink_right(mut self) -> Self {
        self.kind = SidesKind::ShrinkRight;
        self
    }

    #[inline]
    pub fn extend(mut self) -> Self {
        self.kind = SidesKind::Extend;
        self
    }

    #[inline]
    pub fn wrap_mode(mut self, wrap_mode: TextWrapMode) -> Self {
        self.wrap_mode = RSome(wrap_mode);
        self
    }

    #[inline]
    pub fn truncate(mut self) -> Self {
        self.wrap_mode = RSome(TextWrapMode::Truncate);
        self
    }

    #[inline]
    pub fn wrap(mut self) -> Self {
        self.wrap_mode = RSome(TextWrapMode::Wrap);
        self
    }

    #[inline]
    pub fn show<RetL, RetR>(
        self,
        ui: &mut BunnyUi,
        add_contents_left: impl FnMut(&mut BunnyUi) -> RetL,
        add_contents_right: impl FnMut(&mut BunnyUi) -> RetR,
    ) -> (RetL, RetR) {
        ui.sides_show(self, add_contents_left, add_contents_right)
    }
}

#[cfg(feature = "manager")]
impl Sides {
    pub(crate) fn show_impl(
        self,
        ui: &mut egui::Ui,
        contents_left: PluginClosure,
        contents_right: PluginClosure,
    ) {
        let Sides {
            wrap_mode,
            height,
            spacing,
            kind,
        } = self;
        let mut sides = egui::Sides::default();
        sides = match kind {
            SidesKind::Extend => sides,
            SidesKind::ShrinkLeft => sides.shrink_left(),
            SidesKind::ShrinkRight => sides.shrink_right(),
        };
        if let RSome(wrap_mode) = wrap_mode {
            sides = sides.wrap_mode(wrap_mode.into());
        }
        if let RSome(height) = height {
            sides = sides.height(height);
        }
        if let RSome(spacing) = spacing {
            sides = sides.spacing(spacing);
        }

        sides.show(
            ui,
            |ui| {
                let mut b = BunnyUi::new(ui);
                contents_left.call(&mut b);
            },
            |ui| {
                let mut b = BunnyUi::new(ui);
                contents_right.call(&mut b);
            },
        );
    }
}
