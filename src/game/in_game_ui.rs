use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::ui::*;

use super::{game_state::GameState, level::Level};

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {
        clear_ui_system_set(app, GameState::InGame)
            .add_enter_system(GameState::InGame, display_ui)
            .add_system(button_pressed.run_in_state(GameState::InGame));
    }
}

fn display_ui(mut commands: Commands, level: Res<Level>) {
    commands
        .ui_root()
        .position(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Auto)
        .padding(3.)
        .with_children(|parent| {
            if let Some(label) = &level.title {
                parent.text(label).size(30.);
            }
            parent.text("Build a canal connecting the water to the goals");
        });

    #[cfg(feature = "dev")]
    commands
        .ui_root()
        .position(Val::Auto, Val::Px(0.), Val::Px(0.), Val::Auto)
        .with_children(|parent| {
            parent.button("editor", "Editor").style(ButtonStyle::Small);
        });
}

fn button_pressed(mut events: EventReader<ButtonClickEvent>, mut commands: Commands) {
    for event in events.iter() {
        if event.0 == "editor" {
            commands.insert_resource(NextState(GameState::Editor));
        }
    }
}
