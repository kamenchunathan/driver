mod command;

use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use command::CommandPalettePlugin;

/// A plugingroup for the debug features e.g. a debug ui
pub struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EguiPlugin)
            .add(WorldInspectorPlugin::new())
            .add(CommandPalettePlugin)
    }
}
