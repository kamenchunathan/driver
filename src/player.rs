use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let player_texture_atlas = asset_server.load("Actor/Animals/Cat/SpriteSheet.png");
    commands.spawn(SpriteSheetBundle {
        texture_atlas: player_texture_atlas,
        transform: Transform {
            translation: Vec3::new(-10.0, 10.0, 0.0),
            ..default()
        },
        ..default()
    });
}
