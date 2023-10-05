use bevy::{prelude::*, window::close_on_esc};
use tracing_subscriber::{prelude::*, EnvFilter};

use driver::DriverGamePlugins;

fn main() {
    init_tracing();

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

fn init_tracing() {
    let now = time::OffsetDateTime::now_utc();

    let (file_appender, _) = tracing_appender::non_blocking(tracing_appender::rolling::minutely(
        "log",
        format!("{now}"),
    ));

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_writer(file_appender)
                .with_filter(EnvFilter::new(
                    "debug,wgpu_core=warn,wgpu_hal=warn,naga=info,naga_oil=info",
                )),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(EnvFilter::new("info,wgpu_core=warn,wgpu_hal=warn")),
        )
        .init();
}
