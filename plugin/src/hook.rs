use abi_stable::std_types::ROption::{self, RSome};
#[cfg(feature = "3d")]
use bunny_3d::Bunny3d;
#[cfg(feature = "ui")]
use bunny_ui::BunnyUi;
#[cfg(any(feature = "ui", feature = "3d"))]
use shared::BunnyContext;

pub type SimpleCallback = unsafe extern "C" fn();

#[cfg(feature = "ui")]
pub type UiCallback = unsafe extern "C" fn(&mut BunnyUi, &BunnyContext);
#[cfg(not(feature = "ui"))]
pub type UiCallback = unsafe extern "C" fn();

#[cfg(feature = "3d")]
pub type Bunny3dCallback = unsafe extern "C" fn(&mut Bunny3d, &BunnyContext);
#[cfg(not(feature = "3d"))]
pub type Bunny3dCallback = unsafe extern "C" fn();

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Hooks {
    lobby: ROption<SimpleCallback>,
    quest: ROption<SimpleCallback>,
    quest_ending: ROption<SimpleCallback>,
    quest_complete: ROption<SimpleCallback>,

    save: ROption<SimpleCallback>,

    ui_menu: ROption<UiCallback>,
    ui_free: ROption<UiCallback>,

    bunny3d: ROption<Bunny3dCallback>,
}

impl Hooks {
    #[inline]
    pub fn hook_callback(&self, kind: HookKind) -> Option<SimpleCallback> {
        match kind {
            HookKind::Lobby => self.lobby(),
            HookKind::Quest => self.quest(),
            HookKind::QuestEnding => self.quest_ending(),
            HookKind::QuestComplete => self.quest_complete(),
        }
    }

    #[inline]
    pub fn lobby(&self) -> Option<SimpleCallback> {
        self.lobby.into()
    }

    #[inline]
    pub fn quest(&self) -> Option<SimpleCallback> {
        self.quest.into()
    }

    #[inline]
    pub fn quest_ending(&self) -> Option<SimpleCallback> {
        self.quest_ending.into()
    }

    #[inline]
    pub fn quest_complete(&self) -> Option<SimpleCallback> {
        self.quest_complete.into()
    }

    #[inline]
    pub fn ui_menu(&self) -> Option<UiCallback> {
        self.ui_menu.into()
    }

    #[inline]
    pub fn ui_free(&self) -> Option<UiCallback> {
        self.ui_free.into()
    }

    #[inline]
    pub fn save(&self) -> Option<SimpleCallback> {
        self.save.into()
    }

    #[inline]
    pub fn bunny3d(&self) -> Option<Bunny3dCallback> {
        self.bunny3d.into()
    }

    #[inline]
    pub fn set_lobby(&mut self, callback: SimpleCallback) {
        self.lobby = RSome(callback);
    }

    #[inline]
    pub fn set_quest(&mut self, callback: SimpleCallback) {
        self.quest = RSome(callback);
    }

    #[inline]
    pub fn set_quest_ending(&mut self, callback: SimpleCallback) {
        self.quest_ending = RSome(callback);
    }

    #[inline]
    pub fn set_quest_complete(&mut self, callback: SimpleCallback) {
        self.quest_complete = RSome(callback);
    }

    #[inline]
    pub fn set_ui_menu(&mut self, callback: UiCallback) {
        self.ui_menu = RSome(callback);
    }

    #[inline]
    pub fn set_ui_free(&mut self, callback: UiCallback) {
        self.ui_free = RSome(callback);
    }

    #[inline]
    pub fn set_save(&mut self, callback: SimpleCallback) {
        self.save = RSome(callback);
    }

    #[inline]
    pub fn set_bunny3d(&mut self, callback: Bunny3dCallback) {
        self.bunny3d = RSome(callback);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HookKind {
    Lobby,
    Quest,
    QuestEnding,
    QuestComplete,
}
