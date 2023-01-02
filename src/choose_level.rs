use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use iyes_loopless::prelude::IntoConditionalSystem;
use iyes_loopless::state::NextState;

use crate::app_state::*;

use crate::game::level::Level;
use crate::game::level::LevelList;
use crate::ui::*;

pub struct ChooseLevelPlugin;

impl Plugin for ChooseLevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        clear_ui_system_set(app, AppState::ChooseLevel)
            .add_enter_system(AppState::ChooseLevel, display_ui)
            .add_system(load_board)
            .add_system(button_pressed.run_in_state(AppState::ChooseLevel));
    }
}

fn display_ui(mut commands: Commands, levels: Res<LevelList>) {
    commands.spawn(UiRoot::new()).with_children(|parent| {
        parent.spawn(
            GameText::new("Choose Level")
                .size(100.)
                .style(FontStyle::Italic),
        );

        for level in levels.levels.iter() {
            let file = &level.file;
            let name = &level.name;
            parent.spawn(GameButton::new(format!("level:{file}"), name));
        }
    });
}

fn button_pressed(
    mut events: EventReader<ButtonClickEvent>,
    _commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in events.iter() {
        if event.0.starts_with("level:") {
            let file = event.0.replace("level:", "levels/");
            let _ = asset_server.load::<Level, String>(file);
        }
    }
}

fn load_board(
    mut ev_asset: EventReader<AssetEvent<Level>>,
    mut commands: Commands,
    levels: Res<Assets<Level>>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                if let Some(asset) = levels.get(handle) {
                    commands.insert_resource(asset.clone());
                    commands.insert_resource(NextState(AppState::InGame));
                }
            }
            AssetEvent::Modified { handle } => {
                if let Some(asset) = levels.get(handle) {
                    commands.insert_resource(asset.clone());
                    commands.insert_resource(NextState(AppState::InGame));
                }
            }
            _ => {}
        }
    }
}
