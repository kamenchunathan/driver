use bevy::{audio::VolumeLevel, prelude::*};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let track = asset_server.load("Musics/1 - Adventure Begin.ogg");
    commands.spawn(AudioBundle {
        source: track,
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: bevy::audio::Volume::Relative(VolumeLevel::new(10.0)),
            ..default()
        },
    });
}
