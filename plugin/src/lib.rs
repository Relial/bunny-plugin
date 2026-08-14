#![feature(sync_unsafe_cell)]

use std::{path::Path, str::FromStr};

pub use shared::texture::TextureId;

#[cfg(feature = "ui")]
pub use bunny_ui;

#[cfg(feature = "3d")]
pub use bunny_3d;

#[cfg(feature = "plugin")]
#[unsafe(no_mangle)]
pub static BUNNY_API_VERSION: u32 = 3;

use abi_stable::std_types::{
    ROption::{self, RNone, RSome},
    RString,
};
use anyhow::{Result, anyhow};
use tracing_subscriber::filter::LevelFilter;

use crate::hook::{Bunny3dCallback, Hooks, SimpleCallback, UiCallback};

pub mod hook;
pub mod hook_builder;
pub mod hook_cell;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum GameMode {
    LowGrade,
    HighGrade,
}

impl std::fmt::Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GameMode::LowGrade => "Low Grade Edition",
            GameMode::HighGrade => "High Grade Edition",
        };
        write!(f, "{s}")
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MhfoInfo {
    pub game_mode: GameMode,
    pub address: usize,
}

impl MhfoInfo {
    pub fn new(game_mode: GameMode, address: usize) -> Self {
        Self { game_mode, address }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct PluginContext {
    mhfo_info: MhfoInfo,
    config_dir: RString,
    log_level: LogLevel,
}

impl PluginContext {
    pub fn new(mhfo_info: MhfoInfo, config_dir: impl Into<RString>, log_level: LogLevel) -> Self {
        Self {
            mhfo_info,
            config_dir: config_dir.into(),
            log_level,
        }
    }

    #[inline]
    pub fn mhfo_info(&self) -> MhfoInfo {
        self.mhfo_info
    }

    #[inline]
    pub fn config_dir(&self) -> &Path {
        Path::new(self.config_dir.as_str())
    }

    #[inline]
    pub fn log_level(&self) -> LogLevel {
        self.log_level
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct PluginInfo {
    name: RString,
    version: RString,
    pub hooks: Hooks,
    init_fail_reason: ROption<RString>,
}

impl PluginInfo {
    pub fn new(name: impl Into<RString>, version: impl Into<RString>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            hooks: Default::default(),
            init_fail_reason: RNone,
        }
    }

    #[inline]
    pub fn lobby_hook(mut self, callback: SimpleCallback) -> Self {
        self.hooks.set_lobby(callback);
        self
    }

    #[inline]
    pub fn quest_hook(mut self, callback: SimpleCallback) -> Self {
        self.hooks.set_quest(callback);
        self
    }

    #[inline]
    pub fn quest_ending_hook(mut self, callback: SimpleCallback) -> Self {
        self.hooks.set_quest_ending(callback);
        self
    }

    #[inline]
    pub fn quest_complete_hook(mut self, callback: SimpleCallback) -> Self {
        self.hooks.set_quest_complete(callback);
        self
    }

    #[inline]
    pub fn ui_menu(mut self, callback: UiCallback) -> Self {
        self.hooks.set_ui_menu(callback);
        self
    }

    #[inline]
    pub fn ui_free(mut self, callback: UiCallback) -> Self {
        self.hooks.set_ui_free(callback);
        self
    }

    #[inline]
    pub fn save(mut self, callback: SimpleCallback) -> Self {
        self.hooks.set_save(callback);
        self
    }

    #[inline]
    pub fn bunny3d(mut self, callback: Bunny3dCallback) -> Self {
        self.hooks.set_bunny3d(callback);
        self
    }

    #[inline]
    pub fn init_fail(mut self, fail_reason: impl Into<RString>) -> Self {
        self.init_fail_reason = RSome(fail_reason.into());
        self
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn version(&self) -> &str {
        &self.version
    }

    #[inline]
    pub fn init(&self) -> Result<()> {
        if let RSome(fail_reason) = &self.init_fail_reason {
            Err(anyhow!("{}", fail_reason))
        } else {
            Ok(())
        }
    }
}

#[derive(PartialEq, Debug, Default, Clone, Copy)]
#[repr(C)]
pub enum LogLevel {
    Off = 0,
    Error = 1,
    Warn = 2,
    #[default]
    Info = 3,
    Debug = 4,
    Trace = 5,
}

impl FromStr for LogLevel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "OFF" => Ok(Self::Off),
            "ERROR" => Ok(Self::Error),
            "WARN" => Ok(Self::Warn),
            "INFO" => Ok(Self::Info),
            "DEBUG" => Ok(Self::Debug),
            "TRACE" => Ok(Self::Trace),
            _ => Err(anyhow!("Failed to parse LogLevel from str")),
        }
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Off => LevelFilter::OFF,
            LogLevel::Error => LevelFilter::ERROR,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Trace => LevelFilter::TRACE,
        }
    }
}
