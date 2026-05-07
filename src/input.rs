use abi_stable::{
    rvec,
    std_types::{
        ROption::{self, RNone, RSome},
        RVec,
    },
};
use egui::{Event, Pos2, RawInput};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Modifiers {
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Click {
    pub pos: Pos2,
    pub count: u32,
    pub modifiers: Modifiers,
}

impl Click {
    pub fn is_double(&self) -> bool {
        self.count == 2
    }

    pub fn is_triple(&self) -> bool {
        self.count == 3
    }
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerEvent {
    Moved(Pos2),
    Pressed {
        position: Pos2,
        button: PointerButton,
    },
    Released {
        click: ROption<Click>,
        button: PointerButton,
    },
}

impl PointerEvent {
    pub fn is_press(&self) -> bool {
        matches!(self, Self::Pressed { .. })
    }

    pub fn is_release(&self) -> bool {
        matches!(self, Self::Released { .. })
    }

    pub fn is_click(&self) -> bool {
        matches!(
            self,
            Self::Released {
                click: RSome(_),
                ..
            }
        )
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PointerState {
    latest_pos: ROption<Pos2>,
    interact_pos: ROption<Pos2>,
    down: [bool; 5],
    pointer_events: RVec<PointerEvent>,
}

impl Default for PointerState {
    fn default() -> Self {
        Self {
            latest_pos: RNone,
            interact_pos: RNone,
            down: Default::default(),
            pointer_events: rvec![],
        }
    }
}

impl PointerState {
    pub fn collect(previous: &PointerState, raw_input: &RawInput) -> Self {
        let mut new = Self {
            latest_pos: previous.latest_pos,
            interact_pos: previous.interact_pos,
            down: previous.down,
            pointer_events: rvec![],
        };
        new.interact_pos = new.latest_pos;
        for event in &raw_input.events {
            match event {
                Event::PointerMoved(pos) => {
                    let pos = *pos;
                    new.latest_pos = RSome(pos);
                    new.interact_pos = RSome(pos);
                    new.pointer_events.push(PointerEvent::Moved(pos));
                }
                Event::PointerButton {
                    pos,
                    button,
                    pressed,
                    modifiers,
                } => {
                    let pos = *pos;
                    let button = *button;
                    let pressed = *pressed;
                    let modifiers = *modifiers;
                    new.latest_pos = RSome(pos);
                    new.interact_pos = RSome(pos);
                    if pressed {
                        new.pointer_events.push(PointerEvent::Pressed {
                            position: pos,
                            button: button.into(),
                        });
                    } else {
                        let clicked = new.could_any_button_be_click();

                        let click = if clicked {
                            RSome(Click {
                                pos,
                                count: 1,
                                modifiers: modifiers.into(),
                            })
                        } else {
                            RNone
                        };
                        new.pointer_events.push(PointerEvent::Released {
                            click,
                            button: button.into(),
                        });
                    }
                    new.down[button as usize] = pressed;
                }
                Event::PointerGone => {
                    new.latest_pos = RNone;
                }
                _ => {}
            }
        }
        new
    }

    #[inline(always)]
    pub fn latest_pos(&self) -> ROption<Pos2> {
        self.latest_pos
    }

    #[inline(always)]
    pub fn interact_pos(&self) -> ROption<Pos2> {
        self.interact_pos
    }

    #[inline(always)]
    pub fn has_pointer(&self) -> bool {
        self.latest_pos.is_some()
    }

    pub fn any_pressed(&self) -> bool {
        self.pointer_events.iter().any(|event| event.is_press())
    }

    pub fn any_released(&self) -> bool {
        self.pointer_events.iter().any(|event| event.is_release())
    }

    pub fn button_pressed(&self, button: PointerButton) -> bool {
        self.pointer_events
            .iter()
            .any(|event| matches!(event, &PointerEvent::Pressed { button: b, .. } if button == b))
    }

    pub fn button_released(&self, button: PointerButton) -> bool {
        self.pointer_events
            .iter()
            .any(|event| matches!(event, &PointerEvent::Released { button: b, .. } if button == b))
    }

    pub fn primary_pressed(&self) -> bool {
        self.button_pressed(PointerButton::Primary)
    }

    pub fn secondary_pressed(&self) -> bool {
        self.button_pressed(PointerButton::Secondary)
    }

    pub fn primary_released(&self) -> bool {
        self.button_released(PointerButton::Primary)
    }

    pub fn secondary_released(&self) -> bool {
        self.button_released(PointerButton::Secondary)
    }

    pub fn any_down(&self) -> bool {
        self.down.iter().any(|&down| down)
    }

    pub fn any_click(&self) -> bool {
        self.pointer_events.iter().any(|event| event.is_click())
    }

    pub fn button_clicked(&self, button: PointerButton) -> bool {
        self.pointer_events.iter().any(|event| matches!(event, &PointerEvent::Released { click: RSome(_), button: b } if button == b))
    }

    pub fn primary_clicked(&self) -> bool {
        self.button_clicked(PointerButton::Primary)
    }

    pub fn secondary_clicked(&self) -> bool {
        self.button_clicked(PointerButton::Secondary)
    }

    #[inline(always)]
    pub fn button_down(&self, button: PointerButton) -> bool {
        self.down[button as usize]
    }

    #[inline(always)]
    pub fn primary_down(&self) -> bool {
        self.button_down(PointerButton::Primary)
    }

    #[inline(always)]
    pub fn secondary_down(&self) -> bool {
        self.button_down(PointerButton::Secondary)
    }

    #[inline(always)]
    pub fn middle_down(&self) -> bool {
        self.button_down(PointerButton::Middle)
    }

    pub fn could_any_button_be_click(&self) -> bool {
        self.any_down() || self.any_released()
    }
}
