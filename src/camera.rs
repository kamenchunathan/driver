/// Camera controller
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

#[derive(Debug)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::AQUAMARINE),
        },
        ..default()
    });
}
