use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::ui::*;

use super::{
    board::{Tile, TileEvent},
    game_state::GameState,
};

pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        clear_ui_system_set(app, GameState::Editor)
            .add_loopless_state(EditorOperation::None)
            .add_enter_system(GameState::Editor, display_ui)
            .add_system(update_labels.run_in_state(GameState::Editor))
            .add_system(tile_clicked.run_in_state(GameState::Editor))
            .add_system(button_pressed.run_in_state(GameState::Editor));
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum EditorOperation {
    None,
    RaiseHeight,
    LowerHeight,
    ToggleType,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum EditorUiElement {
    CurrentModeText,
}

fn display_ui(mut commands: Commands) {
    commands.insert_resource(NextState(EditorOperation::None));
    commands
        .spawn(
            UiRoot::new()
                .position(Val::Px(0.), Val::Px(0.), Val::Auto, Val::Px(0.))
                .padding(0.),
        )
        .with_children(|parent| {
            parent.spawn(Div::new().opaque()).with_children(|parent| {
                parent.spawn(
                    GameText::new("No Operation Selected").id(EditorUiElement::CurrentModeText),
                );
                parent
                    .spawn(Div::new().horizontal())
                    .with_children(|parent| {
                        parent.spawn(GameButton::new("raise", "Raise Height"));
                        parent.spawn(GameButton::new("lower", "Lower Height"));
                        parent.spawn(GameButton::new("toggle", "Toggle Type"));
                    });
                parent
                    .spawn(
                        Div::new()
                            .position(Val::Auto, Val::Px(2.), Val::Px(2.), Val::Auto)
                            .padding(1.),
                    )
                    .with_children(|parent| {
                        parent.spawn(GameButton::new("exit_editor", "X").style(ButtonStyle::Exit));
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
                        EditorOperation::ToggleType => "Toggle Terrain Types".to_string(),
                    }
                }
            }
        }
    }
}

fn button_pressed(mut events: EventReader<ButtonClickEvent>, mut commands: Commands) {
    for event in events.iter() {
        if event.0 == "exit_editor" {
            commands.insert_resource(NextState(EditorOperation::None));
            commands.insert_resource(NextState(GameState::TurnStart));
        } else if event.0 == "raise" {
            commands.insert_resource(NextState(EditorOperation::RaiseHeight));
        } else if event.0 == "lower" {
            commands.insert_resource(NextState(EditorOperation::LowerHeight));
        } else if event.0 == "toggle" {
            commands.insert_resource(NextState(EditorOperation::ToggleType));
        }
    }
}

fn tile_clicked(
    mut events: EventReader<TileEvent>,
    mut tiles: Query<&mut Tile>,
    operation: Res<CurrentState<EditorOperation>>,
) {
    for event in events.iter() {
        let TileEvent::Clicked(old_tile, entity) = event;
        {
            if let Ok(mut new_tile) = tiles.get_mut(*entity) {
                match operation.0 {
                    EditorOperation::None => {}
                    EditorOperation::RaiseHeight => {
                        new_tile.z = old_tile.z + 1;
                    }
                    EditorOperation::LowerHeight => {
                        new_tile.z = old_tile.z.checked_sub(1).unwrap_or_default();
                    }
                    EditorOperation::ToggleType => {
                        new_tile.tile_type = match old_tile.tile_type {
                            super::board::TileType::Land => super::board::TileType::City,
                            super::board::TileType::City => super::board::TileType::Canal,
                            super::board::TileType::Canal => super::board::TileType::Land,
                        }
                    }
                }
            }
        }
    }
}
