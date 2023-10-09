use std::time::Duration;

use bevy::log::info;
use bevy::prelude::*;
use bevy::time::Timer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player))
            .add_systems(Update, animate_player);
    }
}

#[derive(Debug, Component, Reflect)]
struct AnimationPlayer {
    frame_time: f32,
    timer: Timer,
}

impl AnimationPlayer {
    fn new(frame_time: f32) -> Self {
        Self {
            frame_time,
            timer: Timer::new(Duration::from_secs_f32(frame_time), TimerMode::Repeating),
        }
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
        Name::new("Player"),
        SpriteSheetBundle {
            texture_atlas,
            sprite: TextureAtlasSprite {
                index: 1,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            ..default()
        },
        AnimationPlayer::new(0.4),
    ));
}

fn animate_player(mut player: Query<(&mut TextureAtlasSprite, &mut AnimationPlayer)>, time: Res<Time>) {
    if let Ok((mut sprite, mut player)) = player.get_single_mut() {
        player
            .timer
            .tick(Duration::from_secs_f32(time.delta_seconds()));

        if player.timer.just_finished() {
            sprite.index = (sprite.index + 4) % 13;
        }
    }
}
