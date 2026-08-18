#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

impl Sense {
    #[inline]
    pub const fn hover() -> Self {
        Self::empty()
    }

    #[inline]
    pub const fn focusable_noninteractive() -> Self {
        Self::CLICK.union(Self::FOCUSABLE)
    }

    #[inline]
    pub const fn click() -> Self {
        Self::CLICK.union(Self::FOCUSABLE)
    }

    #[inline]
    pub const fn drag() -> Self {
        Self::DRAG.union(Self::FOCUSABLE)
    }

    #[inline]
    pub const fn click_and_drag() -> Self {
        Self::CLICK.union(Self::FOCUSABLE).union(Self::DRAG)
    }

    #[inline]
    pub const fn interactive(&self) -> bool {
        self.intersects(Self::CLICK.union(Self::DRAG))
    }

    #[inline]
    pub const fn senses_click(&self) -> bool {
        self.contains(Self::CLICK)
    }

    #[inline]
    pub const fn senses_drag(&self) -> bool {
        self.contains(Self::DRAG)
    }

    #[inline]
    pub const fn is_focusable(&self) -> bool {
        self.contains(Self::FOCUSABLE)
    }
}

#[cfg(feature = "manager")]
impl From<Sense> for egui::Sense {
    fn from(value: Sense) -> Self {
        let bits = value.bits();
        Self::from_bits_retain(bits)
    }
}
