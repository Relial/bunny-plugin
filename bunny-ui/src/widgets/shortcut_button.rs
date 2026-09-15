use abi_stable::std_types::ROption::{self, RNone, RSome};
use egui::Id;

use crate::{KeyboardShortcut, paint::corner_radius::CornerRadius};

#[repr(C)]
pub struct ShortcutButton<'a> {
    bind: &'a mut KeyboardShortcut,
    id: Id,
    corner_radius: ROption<CornerRadius>,
    keybind_not_set: bool,
}

impl<'a> ShortcutButton<'a> {
    pub fn new(shortcut: &'a mut KeyboardShortcut, id: Id) -> Self {
        Self {
            bind: shortcut,
            keybind_not_set: false,
            corner_radius: RNone,
            id,
        }
    }

    #[inline]
    pub fn keybind_not_set(mut self, keybind_not_set: bool) -> Self {
        self.keybind_not_set = keybind_not_set;
        self
    }

    #[inline]
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = RSome(corner_radius.into());
        self
    }
}

#[cfg(feature = "manager")]
fn get_expecting(ui: &egui::Ui, id: Id) -> bool {
    ui.ctx()
        .memory_mut(|mem| *mem.data.get_temp_mut_or_default(ui.make_persistent_id(id)))
}

#[cfg(feature = "manager")]
fn set_expecting(ui: &egui::Ui, id: Id, expecting: bool) {
    ui.ctx().memory_mut(|mem| {
        *mem.data.get_temp_mut_or_default(ui.make_persistent_id(id)) = expecting;
    });
}

#[cfg(feature = "manager")]
impl egui::Widget for ShortcutButton<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let text = if self.keybind_not_set {
            "Keybind not set".to_string()
        } else {
            self.bind.format()
        };
        let mut expecting = get_expecting(ui, self.id);
        let mut button = egui::Button::new(text);
        if let RSome(corner_radius) = self.corner_radius {
            button = button.corner_radius(corner_radius);
        }
        if expecting {
            button = button.selected(true);
        }

        let mut response = ui.add(button);

        let prev_expecting = expecting;
        if response.clicked() {
            expecting = !expecting;
        }

        if expecting {
            if response.clicked_elsewhere() {
                expecting = false;
            } else {
                if let Some((key, mods)) = ui.input(|i| {
                    i.events.iter().find_map(|e| match e {
                        egui::Event::Key {
                            key,
                            pressed: true,
                            modifiers,
                            repeat: false,
                            ..
                        } => Some((*key, *modifiers)),
                        _ => None,
                    })
                }) {
                    ui.input_mut(|i| i.consume_key(mods, key));
                    let shortcut = KeyboardShortcut::new(mods.into(), key.into());
                    *self.bind = shortcut;
                    response.mark_changed();
                    expecting = false;
                }
            }
        }

        if prev_expecting != expecting {
            set_expecting(ui, self.id, expecting);
        }
        response
    }
}
