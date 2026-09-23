// reimplement this so plugins don't need egui as a dependency
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Sense(u8);

bitflags::bitflags! {
    impl Sense: u8 {
        const HOVER = 0;
        const CLICK = 1<<0;
        const DRAG = 1<<1;
        const FOCUSABLE = 1<<2;
    }
}

impl std::fmt::Debug for Sense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sense {{")?;
        todo!()
    }
}

impl Sense {
    #[inline]
    pub fn hover() -> Self {
        Self::empty()
    }

    #[inline]
    pub fn focusable_noninteractive() -> Self {
        Self::FOCUSABLE
    }

    #[inline]
    pub fn click() -> Self {
        Self::CLICK | Self::FOCUSABLE
    }

    #[inline]
    pub fn drag() -> Self {
        Self::DRAG | Self::FOCUSABLE
    }

    #[inline]
    pub fn click_and_drag() -> Self {
        Self::CLICK | Self::FOCUSABLE | Self::DRAG
    }

    #[inline]
    pub fn interactive(&self) -> bool {
        self.intersects(Self::CLICK | Self::DRAG)
    }

    #[inline]
    pub fn senses_click(&self) -> bool {
        self.contains(Self::CLICK)
    }

    #[inline]
    pub fn senses_drag(&self) -> bool {
        self.contains(Self::DRAG)
    }

    #[inline]
    pub fn is_focusable(&self) -> bool {
        self.contains(Self::FOCUSABLE)
    }
}

#[cfg(feature = "manager")]
impl From<Sense> for egui::Sense {
    #[inline]
    fn from(value: Sense) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[cfg(feature = "manager")]
impl From<egui::Sense> for Sense {
    #[inline]
    fn from(value: egui::Sense) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
