use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{app_state::AppState, ui::*};

use super::{
    game_state::{GameActionMode, GameResources, GameState},
    level::Level,
};

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {
        clear_ui_system_set(app, GameState::InGame)
            .add_enter_system(GameState::InGame, display_ui)
            .add_system(update_labels.run_in_state(GameState::InGame))
            .add_system(button_pressed.run_in_state(GameState::InGame));
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum GameUiId {
    ActionText,
    CostText,
}

fn display_ui(mut commands: Commands, level: Res<Level>) {
    commands
        .ui_root()
        .position(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Auto)
        .padding(3.)
        .for_state(GameState::InGame)
        .with_children(|parent| {
            if let Some(label) = &level.title {
                parent.text(label).size(30.);
            }
            parent.text("").size(20.).id(GameUiId::CostText);
            parent
                .text("Choose an Action")
                .size(15.)
                .id(GameUiId::ActionText);
        });

    commands
        .ui_root()
        .position(Val::Px(0.), Val::Px(0.), Val::Auto, Val::Px(0.))
        .padding(3.)
        .for_state(GameState::InGame)
        .with_children(|parent| {
            parent.div().horizontal().with_children(|parent| {
                parent
                    .button("dig", "Dig Canal")
                    .style(ButtonStyle::Primary);
                parent
                    .button("lock", "Construct Lock")
                    .style(ButtonStyle::Primary);
            });
        });

    commands
        .ui_root()
        .for_state(GameState::InGame)
        .position(Val::Px(0.), Val::Auto, Val::Px(0.), Val::Auto)
        .with_children(|parent| {
            parent
                .button("choose-level", "Choose Another Level")
                .style(ButtonStyle::Small);
        });

    #[cfg(feature = "dev")]
    commands
        .ui_root()
        .for_state(GameState::InGame)
        .position(Val::Auto, Val::Px(0.), Val::Px(0.), Val::Auto)
        .with_children(|parent| {
            parent.button("editor", "Editor").style(ButtonStyle::Small);
        });
}

fn update_labels(
    mut labels: Query<(&mut GameText, &UiId<GameUiId>)>,
    operation: Res<CurrentState<GameActionMode>>,
    resources: Res<GameResources>,
) {
    if operation.is_changed() {
        for (mut label, id) in labels.iter_mut() {
            if let GameUiId::ActionText = id.val() {
                label.text(match operation.0 {
                    GameActionMode::None => "Choose An Action",
                    GameActionMode::DigCanal => "Dig Canal Tiles",
                    GameActionMode::ConstructLock => "Construct Lock Tiles",
                });
            }
        }
    }
    if resources.is_changed() {
        for (mut label, id) in labels.iter_mut() {
            if let GameUiId::CostText = id.val() {
                label.text(format!("Canal Cost: {} Pounds", resources.cost_so_far));
            }
        }
    }
}

fn button_pressed(mut events: EventReader<ButtonClickEvent>, mut commands: Commands) {
    for event in events.iter() {
        if event.0 == "editor" {
            commands.insert_resource(NextState(GameState::Editor));
        } else if event.0 == "dig" {
            commands.insert_resource(NextState(GameActionMode::DigCanal));
        } else if event.0 == "lock" {
            commands.insert_resource(NextState(GameActionMode::ConstructLock));
        } else if event.0 == "choose-level" {
            commands.insert_resource(NextState(AppState::ChooseLevel));
        }
    }
}
