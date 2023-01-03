use std::io::Write;

use bevy::{asset::FileAssetIo, prelude::*};
use iyes_loopless::prelude::*;

use crate::ui::*;

use super::{
    board::{Tile, TileEvent, TileType},
    game_state::GameState,
    level::Level,
};

pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        clear_ui_system_set(app, GameState::Editor)
            .add_loopless_state(EditorOperation::None)
            .add_enter_system(GameState::Editor, display_ui)
            .add_system(update_labels.run_in_state(GameState::Editor))
            .add_system(tile_clicked.run_in_state(GameState::Editor))
            .add_system(tile_hovered_set.run_in_state(GameState::Editor))
            .add_system(button_pressed.run_in_state(GameState::Editor));
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum EditorOperation {
    None,
    RaiseHeight,
    LowerHeight,
    ToggleType(TileType),
    SetGoal,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum EditorUiElement {
    CurrentModeText,
}

fn display_ui(mut commands: Commands) {
    commands.insert_resource(NextState(EditorOperation::None));
    commands
        .ui_root()
        .position(Val::Px(0.), Val::Px(0.), Val::Auto, Val::Px(0.))
        .padding(0.)
        .with_children(|parent| {
            parent.div().opaque().with_children(|parent| {
                parent
                    .text("No Operation Selected")
                    .id(EditorUiElement::CurrentModeText)
                    .spawn();
                parent.div().horizontal().with_children(|parent| {
                    parent.button("raise", "Raise Height").spawn();
                    parent.button("lower", "Lower Height").spawn();
                    parent.button("toggle", "Toggle Type").spawn();
                    parent.button("goal", "Set Goals").spawn();
                });
                parent
                    .div()
                    .position(Val::Auto, Val::Px(2.), Val::Px(2.), Val::Auto)
                    .horizontal()
                    .padding(1.)
                    .with_children(|parent| {
                        parent
                            .button("save", "Save")
                            .style(ButtonStyle::Small)
                            .spawn();
                        parent
                            .button("exit_editor", "X")
                            .style(ButtonStyle::Small)
                            .spawn();
                    });
            });
        });
}

fn update_labels(
    mut labels: Query<(&mut GameText, &UiId<EditorUiElement>)>,
    operation: Res<CurrentState<EditorOperation>>,
) {
    if operation.is_changed() {
        for (mut label, id) in labels.iter_mut() {
            match id.val() {
                EditorUiElement::CurrentModeText => {
                    label.text = match operation.0 {
                        EditorOperation::None => "No Operation Selected".to_string(),
                        EditorOperation::RaiseHeight => "Raise Height".to_string(),
                        EditorOperation::LowerHeight => "Lower Height".to_string(),
                        EditorOperation::ToggleType(t) => format!("Set to {t:?}"),
                        EditorOperation::SetGoal => "Set Goals".to_string(),
                    }
                }
            }
        }
    }
}

fn button_pressed(
    mut events: EventReader<ButtonClickEvent>,
    mut commands: Commands,
    operation: Res<CurrentState<EditorOperation>>,
    tiles: Query<&Tile>,
    level: Res<Level>,
) {
    for event in events.iter() {
        if event.0 == "exit_editor" {
            commands.insert_resource(NextState(EditorOperation::None));
            commands.insert_resource(NextState(GameState::TurnStart));
        } else if event.0 == "raise" {
            commands.insert_resource(NextState(EditorOperation::RaiseHeight));
        } else if event.0 == "lower" {
            commands.insert_resource(NextState(EditorOperation::LowerHeight));
        } else if event.0 == "goal" {
            commands.insert_resource(NextState(EditorOperation::SetGoal));
        } else if event.0 == "toggle" {
            let next = match operation.0 {
                EditorOperation::ToggleType(t) => match t {
                    super::board::TileType::Land => super::board::TileType::City,
                    super::board::TileType::City => super::board::TileType::Canal,
                    super::board::TileType::Canal => super::board::TileType::Land,
                },
                _ => TileType::Land,
            };
            commands.insert_resource(NextState(EditorOperation::ToggleType(next)));
        } else if event.0 == "save" {
            save(&tiles, &level);
        }
    }
}

fn tile_clicked(
    mut events: EventReader<TileEvent>,
    mut tiles: Query<&mut Tile>,
    operation: Res<CurrentState<EditorOperation>>,
) {
    for event in events.iter() {
        if let TileEvent::Clicked(old_tile, entity) = event {
            if let Ok(mut new_tile) = tiles.get_mut(*entity) {
                match operation.0 {
                    EditorOperation::None => {}
                    EditorOperation::RaiseHeight => {
                        new_tile.z = old_tile.z + 1;
                    }
                    EditorOperation::LowerHeight => {
                        new_tile.z = old_tile.z.checked_sub(1).unwrap_or_default();
                    }
                    EditorOperation::ToggleType(t) => {
                        new_tile.tile_type = t;
                    }
                    EditorOperation::SetGoal => {
                        new_tile.is_goal = !new_tile.is_goal;
                    }
                }
            }
        }
    }
}

fn tile_hovered_set(
    mut events: EventReader<TileEvent>,
    mut tiles: Query<&mut Tile>,
    operation: Res<CurrentState<EditorOperation>>,
    buttons: Res<Input<MouseButton>>,
) {
    if let EditorOperation::ToggleType(t) = operation.0 {
        if buttons.pressed(MouseButton::Left) {
            for event in events.iter() {
                if let TileEvent::HoverStarted(old_tile, entity) = event {
                    if old_tile.tile_type == t {
                        continue;
                    }
                    if let Ok(mut new_tile) = tiles.get_mut(*entity) {
                        new_tile.tile_type = t;
                    }
                }
            }
        }
    }
}

fn save(tiles: &Query<&Tile>, level: &Res<Level>) {
    let level = Level {
        tiles: tiles.iter().cloned().collect(),
        title: level.title.clone(),
    };
    let mut path = FileAssetIo::get_base_path();
    path.push("assets");
    path.push("levels");
    if let Ok(time) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        let time = time.as_secs();
        path.push(format!("edited_level_{time:?}.lvl.json"));

        if let Ok(json) = serde_json::to_string(&level) {
            if let Ok(mut file) = std::fs::File::create(path) {
                let _ = write!(&mut file, "{json}");
            }
        }
    }
}
