#![feature(sync_unsafe_cell)]

use std::{path::Path, str::FromStr};
use abi_stable::std_types::{ROption::{self, RNone, RSome}, RString};

#[cfg(any(feature = "ui", feature = "3d"))]
pub use shared::*;

#[cfg(feature = "ui")]
pub use bunny_ui;

#[cfg(feature = "3d")]
pub use bunny_3d;

#[unsafe(no_mangle)]
pub static BUNNY_API_VERSION: u32 = 3;

pub use abi_stable::std_types as abi_stable_std;

#[cfg(feature = "manager")]
use anyhow::Result;
use anyhow::anyhow;
use tracing_subscriber::filter::LevelFilter;

use crate::hook::{Bunny3dCallback, Hooks, SimpleCallback, UiCallback};

pub mod game_version;
pub mod hook;
pub mod hook_builder;
pub mod hook_cell;

#[derive(Clone, Copy, Debug, PartialEq)]
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

/// Information about the game dll (mhfo.dll/mhfo-hd.dll)
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MhfoInfo {
    /// Whether the game's running in low grade (mhfo.dll) or high grade (mhfo-hd.dll) mode
    pub game_mode: GameMode,
    /// The game dll's base address
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

    /// Information about the game dll (mhfo.dll/mhfo-hd.dll)
    #[inline]
    pub fn mhfo_info(&self) -> MhfoInfo {
        self.mhfo_info
    }

    /// Path to bunny_config. Save plugin config and anything else you want to persist about the plugin state here.
    #[inline]
    pub fn config_dir(&self) -> &Path {
        Path::new(self.config_dir.as_str())
    }

    /// Desired log level. This is configured by the user in cardamom-loader.toml
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
    pub no_unload: bool,
}

impl PluginInfo {
    pub fn new(name: impl Into<RString>, version: impl Into<RString>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            hooks: Default::default(),
            init_fail_reason: RNone,
            no_unload: false,
        }
    }

    /// Called early in the frame when not on a quest
    #[inline]
    pub fn lobby_hook(mut self, callback: SimpleCallback) -> Self {
        self.hooks.set_lobby(callback);
        self
    }

    /// Called early in the frame during quests
    #[inline]
    pub fn quest_hook(mut self, callback: SimpleCallback) -> Self {
        self.hooks.set_quest(callback);
        self
    }

    /// Called early in the frame during the quest end wait when a quest is complete, retired or failed
    #[inline]
    pub fn quest_ending_hook(mut self, callback: SimpleCallback) -> Self {
        self.hooks.set_quest_ending(callback);
        self
    }

    /// Called early in the frame when a quest is complete, during the rewards screen and the fadeout
    #[inline]
    pub fn quest_complete_hook(mut self, callback: SimpleCallback) -> Self {
        self.hooks.set_quest_complete(callback);
        self
    }

    /// Called once per frame when this plugin's manager dropdown menu is open, just before the frame is presented
    #[inline]
    pub fn ui_menu(mut self, callback: UiCallback) -> Self {
        self.hooks.set_ui_menu(callback);
        self
    }

    /// Called once per frame just before the frame is presented
    #[inline]
    pub fn ui_free(mut self, callback: UiCallback) -> Self {
        self.hooks.set_ui_free(callback);
        self
    }

    /// Called once per user defined autosave interval
    #[inline]
    pub fn save(mut self, callback: SimpleCallback) -> Self {
        self.hooks.set_save(callback);
        self
    }

    /// Called once per frame at a timing that's suitable for 3d rendering
    #[inline]
    pub fn bunny3d(mut self, callback: Bunny3dCallback) -> Self {
        self.hooks.set_bunny3d(callback);
        self
    }

    /// Specify a reason initialization failed. If you set this the manager won't make any more calls to the plugin, and the failure reason is shown to the user.
    ///
    /// The plugin will remain loaded unless toggled by the user
    #[inline]
    pub fn init_fail(mut self, fail_reason: impl Into<RString>) -> Self {
        self.init_fail_reason = RSome(fail_reason.into());
        self
    }

    /// Specify that the plugin shouldn't be unloaded. The plugin's toggle checkmark will be faded and uninteractable.
    ///
    /// Does nothing if init_fail is set
    #[inline]
    pub fn no_unload(mut self) -> Self {
        self.no_unload = true;
        self
    }

    #[cfg(feature = "manager")]
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[cfg(feature = "manager")]
    #[inline]
    pub fn version(&self) -> &str {
        &self.version
    }

    #[cfg(feature = "manager")]
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
