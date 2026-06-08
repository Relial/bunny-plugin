use abi_stable::{
    rvec,
    std_types::{
        ROption::{self, RNone, RSome},
        RVec,
    },
};
use egui::{Event, Pos2, RawInput, Vec2};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputOptions {
    pub max_click_dist: f32,
    pub max_click_duration: f64,
    pub max_double_click_delay: f64,
}

impl Default for InputOptions {
    fn default() -> Self {
        Self {
            max_click_dist: 6.0,
            max_click_duration: 0.8,
            max_double_click_delay: 0.3,
        }
    }
}

#[repr(C)]
pub struct InputState {
    pub pointer: PointerState,
    pub modifiers: Modifiers,
}

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
#[derive(Clone, Debug, PartialEq)]
pub struct PointerState {
    time: f64,
    latest_pos: ROption<Pos2>,
    interact_pos: ROption<Pos2>,
    delta: Vec2,
    down: [bool; 5],
    press_origin: ROption<Pos2>,
    press_start_time: ROption<f64>,
    hast_moved_too_much_for_a_click: bool,
    last_click_pos: ROption<Pos2>,
    last_click_time: f64,
    last_last_click_time: f64,
    last_move_time: f64,
    pointer_events: RVec<PointerEvent>,
    options: InputOptions,
}

impl Default for PointerState {
    fn default() -> Self {
        Self {
            time: -f64::INFINITY,
            latest_pos: RNone,
            interact_pos: RNone,
            delta: Vec2::ZERO,
            down: Default::default(),
            press_origin: RNone,
            press_start_time: RNone,
            hast_moved_too_much_for_a_click: false,
            last_click_pos: RNone,
            last_click_time: f64::NEG_INFINITY,
            last_last_click_time: f64::NEG_INFINITY,
            last_move_time: f64::NEG_INFINITY,
            pointer_events: rvec![],
            options: Default::default(),
        }
    }
}

impl PointerState {
    pub fn collect(mut self, time: f64, new: &RawInput, options: InputOptions) -> Self {
        self.time = time;
        self.options = options;
        self.pointer_events.clear();
        let old_pos = self.latest_pos;
        self.interact_pos = self.latest_pos;

        for event in &new.events {
            match event {
                Event::PointerMoved(pos) => {
                    let pos = *pos;
                    self.latest_pos = RSome(pos);
                    self.interact_pos = RSome(pos);

                    if let RSome(press_origin) = self.press_origin {
                        self.hast_moved_too_much_for_a_click |=
                            press_origin.distance(pos) > self.options.max_click_dist;
                    }

                    self.last_move_time = time;
                    self.pointer_events.push(PointerEvent::Moved(pos));
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

                    self.latest_pos = RSome(pos);
                    self.interact_pos = RSome(pos);

                    if pressed {
                        self.press_origin = RSome(pos);
                        self.press_start_time = RSome(time);
                        self.hast_moved_too_much_for_a_click = false;
                        self.pointer_events.push(PointerEvent::Pressed {
                            position: pos,
                            button: button.into(),
                        });
                    } else {
                        let clicked = self.could_any_button_be_click();

                        let click = if clicked {
                            let click_dist_sq = self
                                .last_click_pos
                                .map_or(0.0, |last_pos| last_pos.distance_sq(pos));

                            let double_click = (time - self.last_click_time)
                                < self.options.max_double_click_delay
                                && click_dist_sq
                                    < self.options.max_click_dist * self.options.max_click_dist;
                            let triple_click = (time - self.last_last_click_time)
                                < (self.options.max_double_click_delay * 2.0)
                                && click_dist_sq
                                    < self.options.max_click_dist * self.options.max_click_dist;
                            let count = if triple_click {
                                3
                            } else if double_click {
                                2
                            } else {
                                1
                            };

                            self.last_last_click_time = self.last_click_time;
                            self.last_click_time = time;
                            self.last_click_pos = RSome(pos);

                            RSome(Click {
                                pos,
                                count,
                                modifiers: modifiers.into(),
                            })
                        } else {
                            RNone
                        };

                        self.pointer_events.push(PointerEvent::Released {
                            click,
                            button: button.into(),
                        });

                        self.press_origin = RNone;
                        self.press_start_time = RNone;
                    }

                    self.down[button as usize] = pressed;
                }
                Event::PointerGone => {
                    self.latest_pos = RNone;
                }
                _ => {}
            }
        }

        self.delta = if let (RSome(old_pos), RSome(new_pos)) = (old_pos, self.latest_pos) {
            new_pos - old_pos
        } else {
            Vec2::ZERO
        };

        self
    }

    #[inline(always)]
    pub fn delta(&self) -> Vec2 {
        self.delta
    }

    #[inline(always)]
    pub fn press_origin(&self) -> Option<Pos2> {
        self.press_origin.into()
    }

    #[inline(always)]
    pub fn total_drag_delta(&self) -> Option<Vec2> {
        if let RSome(latest_pos) = self.latest_pos
            && let RSome(press_origin) = self.press_origin
        {
            Some(latest_pos - press_origin)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn press_start_time(&self) -> Option<f64> {
        self.press_start_time.into()
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

    #[inline(always)]
    pub fn time_since_last_movement(&self) -> f32 {
        (self.time - self.last_move_time) as f32
    }

    #[inline(always)]
    pub fn time_since_last_click(&self) -> f32 {
        (self.time - self.last_click_time) as f32
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

    pub fn button_double_clicked(&self, button: PointerButton) -> bool {
        self.pointer_events.iter().any(|event| matches!(event, &PointerEvent::Released { click: RSome(click), button: b } if b == button && click.is_double()))
    }

    pub fn button_triple_clicked(&self, button: PointerButton) -> bool {
        self.pointer_events.iter().any(|event| matches!(event, &PointerEvent::Released { click: RSome(click), button: b } if b == button && click.is_triple()))
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

impl From<egui::InputOptions> for InputOptions {
    fn from(value: egui::InputOptions) -> Self {
        Self {
            max_click_dist: value.max_click_dist,
            max_click_duration: value.max_click_duration,
            max_double_click_delay: value.max_double_click_delay,
        }
    }
}
