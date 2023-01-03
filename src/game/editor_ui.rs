use std::io::Write;

use bevy::{asset::FileAssetIo, prelude::*};
use iyes_loopless::prelude::*;

use crate::ui::*;

use super::{
    board::{Tile, TileEvent, TileType},
    game_state::GameState,
    level::{Level, TileInfo},
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

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum EditorUiElement {
    CurrentModeText,
    Width,
    Height,
}

fn display_ui(mut commands: Commands, level: Res<Level>) {
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
            });
        });
    commands
        .ui_root()
        .position(Val::Auto, Val::Px(2.), Val::Px(2.), Val::Auto)
        .with_children(|parent| {
            parent
                .div()
                .horizontal()
                .padding(1.)
                .with_children(|parent| {
                    parent.text("Width:").size(15.).spawn();
                    parent
                        .text(level.width.to_string())
                        .id(EditorUiElement::Width)
                        .size(15.)
                        .spawn();
                    parent
                        .button("width_add", "+")
                        .style(ButtonStyle::Small)
                        .spawn();
                    parent
                        .button("width_sub", "-")
                        .style(ButtonStyle::Small)
                        .spawn();
                    parent.text("Height:").size(15.).spawn();
                    parent
                        .text(level.height.to_string())
                        .id(EditorUiElement::Height)
                        .size(15.)
                        .spawn();
                    parent
                        .button("height_add", "+")
                        .style(ButtonStyle::Small)
                        .spawn();
                    parent
                        .button("height_sub", "-")
                        .style(ButtonStyle::Small)
                        .spawn();
                    parent
                        .button("new", "New")
                        .style(ButtonStyle::Small)
                        .spawn();
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
}

fn update_labels(
    mut labels: Query<(&mut GameText, &UiId<EditorUiElement>)>,
    operation: Res<CurrentState<EditorOperation>>,
    level: Res<Level>,
) {
    if operation.is_changed() || level.is_changed() {
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
                EditorUiElement::Width => {
                    label.text = level.width.to_string();
                }
                EditorUiElement::Height => {
                    label.text = level.height.to_string();
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
    mut level: ResMut<Level>,
) {
    for event in events.iter() {
        if event.0 == "exit_editor" {
            commands.insert_resource(NextState(EditorOperation::None));
            commands.insert_resource(NextState(GameState::InGame));
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
                    super::board::TileType::City => super::board::TileType::CanalDry,
                    super::board::TileType::CanalDry => super::board::TileType::CanalWet,
                    super::board::TileType::CanalWet => super::board::TileType::Land,
                },
                _ => TileType::Land,
            };
            commands.insert_resource(NextState(EditorOperation::ToggleType(next)));
        } else if event.0 == "save" {
            save(&tiles, &level);
        } else if event.0 == "new" {
            for column in level.tiles.iter_mut() {
                for mut tile in column.iter_mut() {
                    tile.height = 0;
                    tile.tile_type = TileType::Land;
                    tile.is_goal = false;
                }
            }
        } else if event.0 == "width_add" {
            reset_tile_dimensions(level.width + 1, level.height, &mut level, &tiles);
        } else if event.0 == "width_sub" {
            reset_tile_dimensions(level.width - 1, level.height, &mut level, &tiles);
        } else if event.0 == "height_add" {
            reset_tile_dimensions(level.width, level.height + 1, &mut level, &tiles);
        } else if event.0 == "height_sub" {
            reset_tile_dimensions(level.width, level.height - 1, &mut level, &tiles);
        }
    }
}

fn reset_tile_dimensions(width: usize, height: usize, mut level: &mut Level, tiles: &Query<&Tile>) {
    let mut tiles = tiles_to_tile_info(tiles.iter(), level.width, level.height);
    while width < tiles.len() {
        let _ = tiles.pop();
    }
    while width > tiles.len() {
        tiles.push((0..height).map(|_| TileInfo::default()).collect());
    }

    for row in tiles.iter_mut() {
        while height < row.len() {
            let _ = row.pop();
        }
        while height > row.len() {
            row.push(TileInfo::default());
        }
    }

    level.tiles = tiles;
    level.width = width;
    level.height = height;
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

fn tiles_to_tile_info<'a, T: Iterator<Item = &'a Tile>>(
    tiles: T,
    width: usize,
    height: usize,
) -> Vec<Vec<TileInfo>> {
    let mut tile_vec: Vec<Vec<TileInfo>> = (0..width)
        .map(|_| (0..height).map(|_| TileInfo::default()).collect())
        .collect();

    for tile in tiles {
        if let Some(row) = tile_vec.get_mut(tile.x) {
            if let Some(mut info) = row.get_mut(tile.y) {
                info.height = tile.z;
                info.is_goal = tile.is_goal;
                info.tile_type = tile.tile_type;
            }
        }
    }

    tile_vec
}

fn save(tiles: &Query<&Tile>, level: &Level) {
    let tiles = tiles_to_tile_info(tiles.iter(), level.width, level.height);

    let level = Level {
        tiles,
        title: level.title.clone(),
        width: level.width,
        height: level.height,
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
