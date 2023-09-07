use bevy::prelude::*;

use crate::app_state::*;
use crate::assets::CanalManiaAssets;
use crate::ui::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        clear_ui_system_set(app, AppState::MainMenu)
            .add_systems(OnEnter(AppState::MainMenu), display_main_menu)
            .add_systems(Update, button_pressed.run_if(in_state(AppState::MainMenu)));
    }
}

fn display_main_menu(mut commands: Commands, _assets: Res<CanalManiaAssets>) {
    commands
        .ui_root()
        .for_state(AppState::MainMenu)
        .with_children(|parent| {
            parent
                .text("Canal Mania")
                .size(100.)
                .style(FontStyle::Italic);
            parent.button("start_game", "Start Game");
            parent
                .button("credits", "Credits")
                .style(ButtonStyle::Secondary);
        });
}

fn button_pressed(mut events: EventReader<ButtonClickEvent>, mut commands: Commands) {
    for event in events.iter() {
        if event.0 == "start_game" {
            commands.insert_resource(NextState(Some(AppState::ChooseLevel)));
        } else if event.0 == "credits" {
            commands.insert_resource(NextState(Some(AppState::Credits)));
        }
    }
}
