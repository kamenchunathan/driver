mod ui;

use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use ui::DebugUI;

/// A plugingroup for the debug features e.g. a debug ui
pub struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(DebugUI)
            .add(WorldInspectorPlugin::new())
    }
}
