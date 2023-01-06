use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use iyes_loopless::prelude::IntoConditionalSystem;
use iyes_loopless::state::NextState;

use crate::app_state::*;
use crate::ui::*;

use super::game_state::GameState;
use super::level::*;

pub struct InitialDescriptionUiPlugin;

impl Plugin for InitialDescriptionUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        clear_ui_system_set(app, GameState::Description)
            .add_enter_system(GameState::Description, display_ui)
            .add_system(button_pressed.run_in_state(GameState::Description));
    }
}

fn display_ui(mut commands: Commands, level: Res<Level>) {
    if !level.is_changed() {
        return;
    }
    if let Some(description) = &level.initial_description {
        commands
            .ui_root()
            .position(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Auto)
            .for_state(GameState::Description)
            .with_children(|parent| {
                parent.div().opaque().with_children(|parent| {
                    parent
                        .text(level.title.as_ref().unwrap_or(&"Start Level".into()))
                        .size(100.)
                        .style(FontStyle::Italic);
                    parent.text(description);
                });
            });
        commands
            .ui_root()
            .position(Val::Px(0.), Val::Px(0.), Val::Auto, Val::Px(0.))
            .for_state(GameState::Description)
            .with_children(|parent| {
                parent.div().opaque().with_children(|parent| {
                    parent.button("play", "Start");
                    parent
                        .button("menu", "Main Menu")
                        .style(ButtonStyle::Secondary);
                });
            });
    }
}

fn button_pressed(mut events: EventReader<ButtonClickEvent>, mut commands: Commands) {
    for event in events.iter() {
        if event.0 == "menu" {
            commands.insert_resource(NextState(AppState::MainMenu));
        } else if event.0 == "play" {
            commands.insert_resource(NextState(GameState::InGame));
        }
    }
}