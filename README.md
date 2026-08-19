## Overview

Library for making plugins for [Bunny Manager](https://github.com/Relial/bunny-manager).

## Usage

Add bunny-plugin as a dependency in cargo.toml:

```
[dependencies]
bunny-plugin = { git = "https://github.com/Relial/bunny-plugin.git", tag = "api-v3" }
```

Export the init function:

```
use bunny_plugin::{PluginContext, PluginInfo}

#[unsafe(no_mangle)]
pub extern "C" fn init(context: &PluginContext) -> PluginInfo {
    PluginInfo::new("Plugin Name", "Plugin Version")
}
```

Register callbacks through PluginInfo:

```
use bunny_plugin::{PluginContext, PluginInfo, bunny_ui::ui::BunnyUi}

#[unsafe(no_mangle)]
pub extern "C" fn init(context: &PluginContext) -> PluginInfo {
    PluginInfo::new("Plugin Name", "Plugin Version")
        .ui_menu(ui_menu)
        .lobby_hook(on_lobby_update)
}

pub extern "C" fn ui_menu(ui: &mut BunnyUi) {
    ui.label("Waow");
}

pub extern "C" fn on_lobby_update() {
    ...
}
```

## Feature flags

- ui (default) - use BunnyUi for manager settings menu and 2d overlay drawing
- 3d - use Bunny3d for drawing 3d shapes in the game world
- serde - Serialize and Deserialize implementations for various types
- bevy - converting Bevy's Mesh type into Bunny3d's Mesh type
- tobj - converting tobj's Mesh type into Bunny3d's Mesh type
- image - converting image crate DynamicImages into Bunny3d textures
