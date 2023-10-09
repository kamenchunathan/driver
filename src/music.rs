use bevy::{
    audio::{PlaybackSettings, Volume, VolumeLevel, PlaybackMode},
    prelude::*,
    utils::HashMap,
};
use bevy_asset_loader::prelude::*;

#[derive(Debug)]
pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AudioAssetsLoadingState>()
            .add_loading_state(
                LoadingState::new(AudioAssetsLoadingState::Loading)
                    .continue_to_state(AudioAssetsLoadingState::Complete)
                    .on_failure_continue_to_state(AudioAssetsLoadingState::Failed),
            )
            .add_collection_to_loading_state::<_, AudioAssets>(AudioAssetsLoadingState::Loading)
            .insert_resource(AudioSettings { music_volume: 0.1 })
            .add_systems(OnEnter(AudioAssetsLoadingState::Complete), setup);
    }
}

#[derive(Debug, Resource, Reflect)]
struct AudioSettings {
    music_volume: f32,
}

#[derive(Debug, Resource, AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "Music", collection(typed, mapped))]
    music: HashMap<String, Handle<AudioSource>>,

    #[asset(path = "Sounds/Menu", collection(typed, mapped))]
    menu_sounds: HashMap<String, Handle<AudioSource>>,

    #[asset(path = "Sounds/Game", collection(typed, mapped))]
    game_sounds: HashMap<String, Handle<AudioSource>>,
}

/// This enum describes the stages in which assets are loaded
///
/// It is currently used for loading assets required at different points
/// on the game depending on the game state
///
/// In future, it may be used for different chunks of maps to
/// only load those required for a specific section of the map
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, States)]
pub enum AudioAssetsLoadingState {
    /// There is currently only one stage of loading the assets but this
    /// could be split further in future
    #[default]
    Loading,
    Complete,
    Failed,
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    audio_assets: Res<AudioAssets>,
    audio_settings: Res<AudioSettings>,
) {
    debug!("Audio assets: {audio_assets:#?}");
    let source = audio_assets
        .music
        .get("Music/2 - The Cave.ogg")
        .unwrap()
        .clone();

    commands.spawn((
        AudioBundle {
            source,
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::Relative(VolumeLevel::new(audio_settings.music_volume)),
                ..default()
            },
        },
        Name::new("Music player"),
    ));
}

fn volume_changed(
    mut audio_settings: Res<AudioSettings>,
    mut music_settings_query: Query<&mut PlaybackSettings>,
) {
    if audio_settings.is_changed() {
        let mut music_settings = music_settings_query.get_single_mut().unwrap();
        music_settings.volume = Volume::Relative(VolumeLevel::new(audio_settings.music_volume));
    }
}
