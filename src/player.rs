use bevy::log::info;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player));
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut assets: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_sheet = asset_server.load("Actor/Characters/BlueNinja/SpriteSheet.png");

    let texture_atlas = assets.add(TextureAtlas::from_grid(
        sprite_sheet,
        Vec2::new(16.0, 16.0),
        4,
        7,
        None,
        None,
    ));

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas,
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            ..default()
        },
        Name::new("Player"),
    ));
}
