use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use iyes_loopless::prelude::IntoConditionalSystem;
use iyes_loopless::state::NextState;

use crate::app_state::*;
use crate::ui::*;

pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        clear_ui_system_set(app, AppState::Credits)
            .add_enter_system(AppState::Credits, display_credits)
            .add_system(button_pressed.run_in_state(AppState::Credits));
    }
}

fn display_credits(mut commands: Commands) {
    commands.spawn(UiRoot).with_children(|parent| {
        parent.spawn(GameText::new("Credits").size(100.).style(FontStyle::Italic));
        parent.spawn(GameText::new("Created by Lee-Orr"));

        parent.spawn(GameText::new("for Historically Accurate Game Jam 6"));

        parent.spawn(GameText::new(
            "Using the Libre-Baskerville font by Rodrigo Fuenzalida and Pablo Impallari",
        ));

        parent.spawn(GameButton::new("menu", "Back"));
    });
}

fn button_pressed(mut events: EventReader<ButtonClickEvent>, mut commands: Commands) {
    for event in events.iter() {
        if event.0 == "menu" {
            commands.insert_resource(NextState(AppState::MainMenu));
        }
    }
}
