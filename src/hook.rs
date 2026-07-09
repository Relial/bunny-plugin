use abi_stable::std_types::ROption::{self, RSome};

pub type HookCallback = unsafe extern "C" fn();

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Hooks {
    lobby: ROption<HookCallback>,
    quest: ROption<HookCallback>,
    quest_ending: ROption<HookCallback>,
    quest_complete: ROption<HookCallback>,
}

impl Hooks {
    pub fn callback(&self, kind: HookKind) -> Option<HookCallback> {
        match kind {
            HookKind::Lobby => self.lobby(),
            HookKind::Quest => self.quest(),
            HookKind::QuestEnding => self.quest_ending(),
            HookKind::QuestComplete => self.quest_complete(),
        }
    }

    pub fn lobby(&self) -> Option<HookCallback> {
        self.lobby.into()
    }

    pub fn quest(&self) -> Option<HookCallback> {
        self.quest.into()
    }

    pub fn quest_ending(&self) -> Option<HookCallback> {
        self.quest_ending.into()
    }

    pub fn quest_complete(&self) -> Option<HookCallback> {
        self.quest_complete.into()
    }

    pub fn set_lobby(&mut self, callback: HookCallback) {
        self.lobby = RSome(callback);
    }

    pub fn set_quest(&mut self, callback: HookCallback) {
        self.quest = RSome(callback);
    }

    pub fn set_quest_ending(&mut self, callback: HookCallback) {
        self.quest_ending = RSome(callback);
    }

    pub fn set_quest_complete(&mut self, callback: HookCallback) {
        self.quest_complete = RSome(callback);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HookKind {
    Lobby,
    Quest,
    QuestEnding,
    QuestComplete,
}
