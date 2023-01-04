use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use iyes_loopless::prelude::IntoConditionalSystem;
use iyes_loopless::state::NextState;

use crate::app_state::*;
use crate::ui::*;

use super::game_state::GameResources;
use super::game_state::GameState;
use super::level::*;

pub struct GameCompleteUiPlugin;

impl Plugin for GameCompleteUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        clear_ui_system_set(app, GameState::Complete)
            .add_enter_system(GameState::Complete, display_ui)
            .add_system(button_pressed.run_in_state(GameState::Complete));
    }
}

fn display_ui(mut commands: Commands, resource: Res<GameResources>, level: Res<Level>) {
    commands
        .ui_root()
        .for_state(GameState::Complete)
        .with_children(|parent| {
            parent
                .text(format!(
                    "{} Complete!",
                    level.title.as_ref().unwrap_or(&"Level".into())
                ))
                .size(100.)
                .style(FontStyle::Italic);
            parent.text(format!("The canal cost you {}", resource.cost_so_far));
            parent.button("level", "Play Another Level");
            parent
                .button("menu", "Main Menu")
                .style(ButtonStyle::Secondary);
        });
}

fn button_pressed(mut events: EventReader<ButtonClickEvent>, mut commands: Commands) {
    for event in events.iter() {
        if event.0 == "menu" {
            commands.insert_resource(NextState(AppState::MainMenu));
        } else if event.0 == "level" {
            commands.insert_resource(NextState(AppState::ChooseLevel));
        }
    }
}
