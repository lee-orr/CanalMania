use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use iyes_loopless::prelude::IntoConditionalSystem;
use iyes_loopless::state::NextState;

use crate::app_state::*;
use crate::ui::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        clear_ui_system_set(app, AppState::MainMenu)
            .add_enter_system(AppState::MainMenu, display_main_menu)
            .add_system(button_pressed.run_in_state(AppState::MainMenu));
    }
}

fn display_main_menu(mut commands: Commands) {
    println!("Display Main Menu");
    commands.spawn(UiRoot::new()).with_children(|parent| {
        parent.spawn(
            GameText::new("Canal Mania")
                .size(100.)
                .style(FontStyle::Italic),
        );
        parent.spawn(GameButton::new("start_game", "Start Game"));
        parent.spawn(GameButton::new("credits", "Credits").style(ButtonStyle::Secondary));
    });
}

fn button_pressed(mut events: EventReader<ButtonClickEvent>, mut commands: Commands) {
    for event in events.iter() {
        if event.0 == "start_game" {
            commands.insert_resource(NextState(AppState::ChooseLevel));
        } else if event.0 == "credits" {
            commands.insert_resource(NextState(AppState::Credits));
        }
    }
}
