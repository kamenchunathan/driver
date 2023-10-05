// For development purposes only
#![allow(unused)]

mod camera;
mod music;
mod player;
mod states;

#[cfg(feature = "debug")]
pub mod debug;

use bevy::{app::PluginGroupBuilder, prelude::*};

use camera::CameraPlugin;
use music::MusicPlugin;
use player::PlayerPlugin;

#[cfg(feature = "debug")]
use debug::DebugPlugins;

pub struct DriverGamePlugins;

impl PluginGroup for DriverGamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CameraPlugin)
            .add(PlayerPlugin)
            .add(MusicPlugin)

    }
}
