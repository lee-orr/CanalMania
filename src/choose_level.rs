use bevy::prelude::*;

use crate::app_state::*;

use crate::assets::CanalManiaAssets;
use crate::game::level::Level;
use crate::game::level::LevelList;
use crate::ui::*;

pub struct ChooseLevelPlugin;

impl Plugin for ChooseLevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        clear_ui_system_set(app, AppState::ChooseLevel)
            .add_systems(OnEnter(AppState::ChooseLevel), display_ui)
            .add_system(load_board)
            .add_systems(
                Update,
                button_pressed.run_if(in_state(AppState::ChooseLevel)),
            );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum ElId {
    Text,
    LevelButtonContainer,
}

fn display_ui(
    mut commands: Commands,
    level_list_asset: Res<Assets<LevelList>>,
    assets: Res<CanalManiaAssets>,
) {
    if let Some(levels) = level_list_asset.get(&assets.level_list) {
        commands
            .ui_root()
            .for_state(AppState::ChooseLevel)
            .with_children(|parent| {
                parent
                    .text("Choose Level")
                    .size(100.)
                    .style(FontStyle::Italic)
                    .id(ElId::Text);

                parent
                    .div()
                    .id(ElId::LevelButtonContainer)
                    .with_children(|parent| {
                        for level in levels.levels.iter() {
                            let file = &level.file;
                            let name = &level.name;
                            parent.button(format!("level:{file}"), name);
                        }
                    });
                parent.div().padding(5.).with_children(|parent| {
                    parent.button("back", "Back").style(ButtonStyle::Small);
                });
            });
    }
}

fn button_pressed(
    mut events: EventReader<ButtonClickEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    levels: Res<Assets<Level>>,
    mut texts: Query<(&UiId<ElId>, &mut GameText)>,
    containers: Query<(Entity, &UiId<ElId>), With<Div>>,
) {
    for event in events.iter() {
        if event.0.starts_with("level:") {
            let file = event.0.replace("level:", "levels/");
            let handle = asset_server.load::<Level, String>(file);

            if let Some(asset) = levels.get(&handle) {
                commands.insert_resource(asset.clone());
                commands.insert_resource(NextState(Some(AppState::InGame)));
            } else {
                for (id, mut text) in texts.iter_mut() {
                    if id.val() == &ElId::Text {
                        text.text = "Loading...".into();
                    }
                }
                for (entity, id) in containers.iter() {
                    if id.val() == &ElId::LevelButtonContainer {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        } else if event.0 == "back" {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
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
                    commands.insert_resource(NextState(Some(AppState::InGame)));
                }
            }
            AssetEvent::Modified { handle } => {
                if let Some(asset) = levels.get(handle) {
                    commands.insert_resource(asset.clone());
                    commands.insert_resource(NextState(Some(AppState::InGame)));
                }
            }
            _ => {}
        }
    }
}
