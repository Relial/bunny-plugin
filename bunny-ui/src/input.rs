use crate::key::Key;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyEvent {
    pub key: Key,
    pub modifiers: Modifiers,
    pub pressed: bool,
    pub repeat: bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerButton {
    Primary = 0,
    Secondary = 1,
    Middle = 2,
    Extra1 = 3,
    Extra2 = 4,
}

impl From<egui::PointerButton> for PointerButton {
    fn from(value: egui::PointerButton) -> Self {
        match value {
            egui::PointerButton::Primary => Self::Primary,
            egui::PointerButton::Secondary => Self::Secondary,
            egui::PointerButton::Middle => Self::Middle,
            egui::PointerButton::Extra1 => Self::Extra1,
            egui::PointerButton::Extra2 => Self::Extra2,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Modifiers {
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
}

impl Modifiers {
    pub const NONE: Self = Self {
        alt: false,
        ctrl: false,
        shift: false,
    };

    pub const ALT: Self = Self {
        alt: true,
        ctrl: false,
        shift: false,
    };

    pub const CTRL: Self = Self {
        alt: false,
        ctrl: true,
        shift: false,
    };

    pub const SHIFT: Self = Self {
        alt: false,
        ctrl: false,
        shift: true,
    };

    #[inline]
    pub const fn plus(self, rhs: Self) -> Self {
        Self {
            alt: self.alt | rhs.alt,
            ctrl: self.ctrl | rhs.ctrl,
            shift: self.shift | rhs.shift,
        }
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        self == &Self::default()
    }

    #[inline]
    pub fn any(&self) -> bool {
        !self.is_none()
    }

    #[inline]
    pub fn all(&self) -> bool {
        self.alt && self.ctrl && self.shift
    }

    pub fn matches_logically(&self, pattern: Self) -> bool {
        if pattern.alt && !self.alt {
            return false;
        }
        if pattern.shift && !self.shift {
            return false;
        }
        if pattern.ctrl && !self.ctrl {
            return false;
        }

        true
    }
}

impl std::ops::BitOr for Modifiers {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.plus(rhs)
    }
}

impl std::ops::BitOrAssign for Modifiers {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl From<egui::Modifiers> for Modifiers {
    fn from(value: egui::Modifiers) -> Self {
        Self {
            alt: value.alt,
            ctrl: value.ctrl,
            shift: value.shift,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyboardShortcut {
    pub logical_key: Key,
    pub modifiers: Modifiers,
}

impl KeyboardShortcut {
    pub const fn new(modifiers: Modifiers, logical_key: Key) -> Self {
        Self {
            modifiers,
            logical_key,
        }
    }

    pub fn format(&self) -> String {
        let mut s = String::new();

        let mut append_if = |modifier_is_active, modifier_name| {
            if modifier_is_active {
                if !s.is_empty() {
                    s += "+";
                }
                s += modifier_name;
            }
        };

        append_if(self.modifiers.ctrl, "Ctrl");
        append_if(self.modifiers.alt, "Alt");
        append_if(self.modifiers.shift, "Shift");

        if !s.is_empty() {
            s += "+";
        }
        s += self.logical_key.name();

        s
    }
}

impl From<egui::KeyboardShortcut> for KeyboardShortcut {
    fn from(value: egui::KeyboardShortcut) -> Self {
        Self {
            modifiers: value.modifiers.into(),
            logical_key: value.logical_key.into(),
        }
    }
}
