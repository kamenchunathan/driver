// For development purposes only
#![allow(unused)]

mod camera;
mod music;
mod player;
mod states;

use bevy::{app::PluginGroupBuilder, prelude::*};

use camera::CameraPlugin;
use music::MusicPlugin;
use player::PlayerPlugin;

pub struct DriverGamePlugins;

impl PluginGroup for DriverGamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CameraPlugin)
            .add(PlayerPlugin)
            .add(MusicPlugin)
    }
}
