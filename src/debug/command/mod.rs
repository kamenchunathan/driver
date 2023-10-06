use std::str::FromStr;

use bevy::log::{info, warn};
use bevy::prelude::*;
use bevy_egui::{
    egui,
    egui::{Align, Color32, Frame, Margin, TextEdit},
    EguiContexts,
};

pub struct CommandPalettePlugin;

impl Plugin for CommandPalettePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandEvent>()
            .insert_resource(CommandPalette::default())
            .add_systems(
                Update,
                (command_palette, handle_commands.after(command_palette)),
            );
    }
}

#[derive(Debug, Default, Resource, Reflect)]
struct CommandPalette {
    open: bool,
    value: String,
}

#[derive(Debug, Event, Reflect)]
struct CommandEvent(Command);

fn command_palette(
    input: Res<Input<KeyCode>>,
    mut palette_state: ResMut<CommandPalette>,
    mut contexts: EguiContexts,
    mut event_writer: EventWriter<CommandEvent>,
) {
    // handle input
    let ctrl = input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    if ctrl && input.just_pressed(KeyCode::P) {
        palette_state.open = !palette_state.open;
    }

    // Render palette
    if palette_state.open {
        egui::Area::new("command_palette")
            .anchor(egui::Align2::CENTER_TOP, egui::Vec2::new(0.0, 2.0))
            .show(&mut contexts.ctx_mut(), |ui| {
                ui.set_max_height(50.0);
                ui.set_max_width(500.0);

                let response = ui
                    .add(TextEdit::singleline(&mut palette_state.value).hint_text("Enter command"));

                response.request_focus();

                if ui.input(|inp| inp.key_pressed(egui::Key::Enter)) {
                    if let Ok(command) = Command::from_str(&palette_state.value) {
                        event_writer.send(CommandEvent(command));
                        palette_state.value.clear();
                        palette_state.open = false;
                    } else {
                        warn!("Invalid command");
                    }
                }
            });
    }
}

fn handle_commands(mut events: EventReader<CommandEvent>) {
    for CommandEvent(cmd) in events.iter() {
        info!(cmd = cmd.to_string(), "Handling");
    }
}

#[derive(Debug, Reflect)]
enum Command {
    NoOp,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "no op" => Ok(Command::NoOp),
            _ => Err(()),
        }
    }
}

impl ToString for Command {
    fn to_string(&self) -> String {
        String::from("NoOp")
    }
}
