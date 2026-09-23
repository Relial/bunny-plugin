use abi_stable::std_types::{ROption, RString};
use emath::{Pos2, Vec2};

use crate::types::key::Key;

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct KeyEvent {
    pub key: Key,
    pub modifiers: Modifiers,
    pub pressed: bool,
    pub repeat: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub enum PointerButton {
    Primary = 0,
    Secondary = 1,
    Middle = 2,
    Extra1 = 3,
    Extra2 = 4,
}

#[cfg(feature = "manager")]
impl From<PointerButton> for egui::PointerButton {
    fn from(value: PointerButton) -> Self {
        match value {
            PointerButton::Primary => Self::Primary,
            PointerButton::Secondary => Self::Secondary,
            PointerButton::Middle => Self::Middle,
            PointerButton::Extra1 => Self::Extra1,
            PointerButton::Extra2 => Self::Extra2,
        }
    }
}

#[cfg(feature = "manager")]
impl From<egui::PointerButton> for PointerButton {
    #[inline]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
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

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.plus(rhs)
    }
}

impl std::ops::BitOrAssign for Modifiers {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

#[cfg(feature = "manager")]
impl From<egui::Modifiers> for Modifiers {
    #[inline]
    fn from(value: egui::Modifiers) -> Self {
        Self {
            alt: value.alt,
            ctrl: value.ctrl,
            shift: value.shift,
        }
    }
}

#[cfg(feature = "manager")]
impl From<Modifiers> for egui::Modifiers {
    #[inline]
    fn from(value: Modifiers) -> Self {
        let Modifiers { alt, ctrl, shift } = value;
        Self {
            alt,
            ctrl,
            shift,
            mac_cmd: false,
            command: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
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

#[cfg(feature = "manager")]
impl From<egui::KeyboardShortcut> for KeyboardShortcut {
    #[inline]
    fn from(value: egui::KeyboardShortcut) -> Self {
        Self {
            modifiers: value.modifiers.into(),
            logical_key: value.logical_key.into(),
        }
    }
}

#[cfg(feature = "manager")]
impl From<KeyboardShortcut> for egui::KeyboardShortcut {
    #[inline]
    fn from(value: KeyboardShortcut) -> Self {
        let KeyboardShortcut {
            logical_key,
            modifiers,
        } = value;
        Self {
            modifiers: modifiers.into(),
            logical_key: logical_key.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub enum Event {
    Copy,
    Cut,
    Paste(RString),
    Text(RString),
    Key {
        key: Key,
        physical_key: ROption<Key>,
        pressed: bool,
        repeat: bool,
        modifiers: Modifiers,
    },
    PointerMoved(Pos2),
    MouseMoved(Vec2),
    PointerButton {
        pos: Pos2,
        button: PointerButton,
        pressed: bool,
        modifiers: Modifiers,
    },
    PointerGone,
    Zoom(f32),
    Rotate(f32),
    Ime,
    Touch,
    MouseWheel {
        unit: MouseWheelUnit,
        delta: Vec2,
        modifiers: Modifiers,
    },
    WindowFocused(bool),
    AccessKitActionRequest,
    Screenshot,
}

#[cfg(feature = "manager")]
impl From<&egui::Event> for Event {
    #[inline]
    fn from(value: &egui::Event) -> Self {
        match value {
            egui::Event::Copy => Self::Copy,
            egui::Event::Cut => Self::Cut,
            egui::Event::Paste(paste) => Self::Paste(paste.as_str().into()),
            egui::Event::Text(text) => Self::Text(text.as_str().into()),
            egui::Event::Key {
                key,
                physical_key,
                pressed,
                repeat,
                modifiers,
            } => Self::Key {
                key: (*key).into(),
                physical_key: physical_key.map(|k| k.into()).into(),
                pressed: *pressed,
                repeat: *repeat,
                modifiers: (*modifiers).into(),
            },
            egui::Event::PointerMoved(pos2) => Self::PointerMoved(*pos2),
            egui::Event::MouseMoved(vec2) => Self::MouseMoved(*vec2),
            egui::Event::PointerButton {
                pos,
                button,
                pressed,
                modifiers,
            } => Self::PointerButton {
                pos: *pos,
                button: (*button).into(),
                pressed: *pressed,
                modifiers: (*modifiers).into(),
            },
            egui::Event::PointerGone => Self::PointerGone,
            egui::Event::Zoom(zoom) => Self::Zoom(*zoom),
            egui::Event::Rotate(rotate) => Self::Rotate(*rotate),
            egui::Event::Ime(_) => Self::Ime,
            egui::Event::Touch {
                device_id: _,
                id: _,
                phase: _,
                pos: _,
                force: _,
            } => Self::Touch,
            egui::Event::MouseWheel {
                unit,
                delta,
                phase: _,
                modifiers,
            } => Self::MouseWheel {
                unit: (*unit).into(),
                delta: *delta,
                modifiers: (*modifiers).into(),
            },
            egui::Event::WindowFocused(focused) => Self::WindowFocused(*focused),
            egui::Event::AccessKitActionRequest(_) => Self::AccessKitActionRequest,
            egui::Event::Screenshot {
                viewport_id: _,
                user_data: _,
                image: _,
            } => Self::Screenshot,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum MouseWheelUnit {
    Point,
    Line,
    Page,
}

#[cfg(feature = "manager")]
impl From<egui::MouseWheelUnit> for MouseWheelUnit {
    #[inline]
    fn from(value: egui::MouseWheelUnit) -> Self {
        match value {
            egui::MouseWheelUnit::Point => Self::Point,
            egui::MouseWheelUnit::Line => Self::Line,
            egui::MouseWheelUnit::Page => Self::Page,
        }
    }
}
