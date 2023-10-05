use bevy::{prelude::*, window::close_on_esc};

use driver::DriverGamePlugins;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Driver"),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            DriverGamePlugins,
        ))
        .add_systems(Update, close_on_esc)
        .run()
}
