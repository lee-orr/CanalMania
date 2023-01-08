use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{app_state::AppState, assets::CanalManiaAssets, ui::*};

use super::{
    game_state::{GameActionMode, GameResources, GameState},
    level::{Level, LevelTools},
};

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {
        clear_ui_system_set(app, GameState::InGame)
            .init_resource::<SidebarText>()
            .add_enter_system(GameState::InGame, display_ui)
            .add_system(update_labels.run_in_state(GameState::InGame))
            .add_system(update_sidebar.run_in_state(GameState::InGame))
            .add_system(update_buttons.run_in_state(GameState::InGame))
            .add_system(button_pressed.run_in_state(GameState::InGame));
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum GameUiId {
    CostText,
    SidebarText,
    Sidebar,
    Dig,
    Lock,
    Aquaduct,
    Demolish,
}

#[derive(Resource, Debug, Clone, Default, Reflect)]
pub struct SidebarText(pub Option<String>);

fn display_ui(
    mut commands: Commands,
    level: Res<Level>,
    asset: Res<CanalManiaAssets>,
    tools: Res<LevelTools>,
    sidebar: Res<SidebarText>,
    resources: Res<GameResources>,
    operation: Res<CurrentState<GameActionMode>>,
) {
    commands
        .ui_root()
        .position(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Auto)
        .padding(3.)
        .for_state(GameState::InGame)
        .with_children(|parent| {
            if let Some(label) = &level.title {
                parent.text(label).size(35.);
            }
            parent.div().horizontal().opaque().with_children(|parent| {
                parent
                    .icon(asset.coin_icon.clone())
                    .size(GameIconSize::Normal);
                parent
                    .text(resources.cost_so_far.to_string())
                    .size(20.)
                    .id(GameUiId::CostText);
            });
        });

    commands
        .ui_root()
        .position(Val::Px(0.), Val::Px(0.), Val::Auto, Val::Px(0.))
        .padding(3.)
        .for_state(GameState::InGame)
        .with_children(|parent| {
            parent.div().horizontal().opaque().with_children(|parent| {
                parent
                    .button("dig", "Dig Canal\nFlow water along a plane")
                    .id(GameUiId::Dig)
                    .style(ButtonStyle::Action)
                    .hidden(!tools.canal)
                    .selected(operation.0 == GameActionMode::DigCanal)
                    .icon(asset.dig_canal_icon.clone());
                parent
                    .button("lock", "Construct Lock\nConnect canals to water above them")
                    .id(GameUiId::Lock)
                    .style(ButtonStyle::Action)
                    .hidden(!tools.lock)
                    .selected(operation.0 == GameActionMode::ConstructLock)
                    .icon(asset.lock_icon.clone());
                parent
                    .button("aquaduct", "Construct Aquaduct\nAllow canals to cross a gap\nMust be built to level")
                    .id(GameUiId::Aquaduct)
                    .style(ButtonStyle::Action)
                    .hidden(!tools.aquaduct)
                    .selected(operation.0 == GameActionMode::BuildAquaduct)
                    .icon(asset.aqueduct_icon.clone());
                parent
                    .button("demolish", "Demolish\nMade a mistake? Demolish it.")
                    .id(GameUiId::Demolish)
                    .style(ButtonStyle::Action)
                    .hidden(!tools.demolish)
                    .selected(operation.0 == GameActionMode::Demolish)
                    .icon(asset.demolish_icon.clone());
            });
        });

    commands
        .ui_root()
        .for_state(GameState::InGame)
        .position(Val::Px(0.), Val::Auto, Val::Px(0.), Val::Auto)
        .with_children(|parent| {
            parent.div().padding(50.);
            parent
                .div()
                .position(Val::Px(0.), Val::Auto, Val::Px(0.), Val::Auto)
                .with_children(|parent| {
                    parent
                        .button("choose-level", "Choose Another Level")
                        .style(ButtonStyle::Small)
                        .icon(asset.menu_icon.clone())
                        .hover_direction(crate::ui::div::Direction::Horizontal);
                });
            parent
                .div()
                .opaque()
                .size(Size::new(Val::Px(200.), Val::Auto))
                .hidden(sidebar.0.is_none())
                .id(GameUiId::Sidebar)
                .with_children(|parent| {
                    parent
                        .text(sidebar.0.as_ref().unwrap_or(&String::new()))
                        .size(15.)
                        .id(GameUiId::SidebarText);
                });
            parent.div().position(Val::Px(0.), Val::Auto, Val::Px(30.), Val::Auto).with_children(|parent| {
            parent.button("help", "Drag Midde Mouse Button, Arrow Keys or WSAD to move the camera\n\nDrag Right Mouse Button, Control + Arrow Keys or WSAD to orbit the camera\n\nScroll Wheel, + or - Keys to zoom").icon(asset.help_icon.clone()).hover_direction(crate::ui::div::Direction::Horizontal).style(ButtonStyle::Small);
        });
        });

    #[cfg(feature = "dev")]
    commands
        .ui_root()
        .for_state(GameState::InGame)
        .position(Val::Auto, Val::Px(0.), Val::Auto, Val::Px(0.))
        .with_children(|parent| {
            parent.button("editor", "Editor").style(ButtonStyle::Small);
        });
}

fn update_labels(
    mut labels: Query<(&mut GameText, &UiId<GameUiId>)>,
    mut buttons: Query<(&mut GameButton, &UiId<GameUiId>)>,
    operation: Res<CurrentState<GameActionMode>>,
    resources: Res<GameResources>,
) {
    if operation.is_changed() {
        for (mut button, id) in buttons.iter_mut() {
            let selected = match id.val() {
                GameUiId::Dig => operation.0 == GameActionMode::DigCanal,
                GameUiId::Lock => operation.0 == GameActionMode::ConstructLock,
                GameUiId::Aquaduct => operation.0 == GameActionMode::BuildAquaduct,
                GameUiId::Demolish => operation.0 == GameActionMode::Demolish,
                _ => {
                    continue;
                }
            };
            button.selected(selected);
        }
    }
    if resources.is_changed() {
        for (mut label, id) in labels.iter_mut() {
            if let GameUiId::CostText = id.val() {
                label.text(resources.cost_so_far.to_string());
            }
        }
    }
}

fn update_sidebar(
    mut labels: Query<(&mut GameText, &UiId<GameUiId>)>,
    mut divs: Query<(&mut Div, &UiId<GameUiId>)>,
    sidebar: Res<SidebarText>,
) {
    if sidebar.is_changed() {
        info!("Sidebar changed");
        for (mut label, id) in labels.iter_mut() {
            if let GameUiId::SidebarText = id.val() {
                if let Some(text) = &sidebar.0 {
                    info!("Updated sidebar text");
                    label.text(text);
                }
            }
        }

        for (mut div, id) in divs.iter_mut() {
            if let GameUiId::Sidebar = id.val() {
                info!("Set sidebar hidden to {}", sidebar.0.is_none());
                div.hidden(sidebar.0.is_none());
            }
        }
    }
}

fn update_buttons(
    mut buttons: Query<(&mut GameButton, &UiId<GameUiId>), With<GameButton>>,
    tools: Res<LevelTools>,
) {
    if tools.is_changed() {
        for (mut button, id) in buttons.iter_mut() {
            let hidden = !match id.val() {
                GameUiId::Dig => tools.canal,
                GameUiId::Lock => tools.lock,
                GameUiId::Aquaduct => tools.aquaduct,
                GameUiId::Demolish => tools.demolish,
                _ => true,
            };

            button.hidden(hidden);
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
        } else if event.0 == "aquaduct" {
            commands.insert_resource(NextState(GameActionMode::BuildAquaduct));
        } else if event.0 == "demolish" {
            commands.insert_resource(NextState(GameActionMode::Demolish));
        } else if event.0 == "choose-level" {
            commands.insert_resource(NextState(AppState::ChooseLevel));
        }
    }
}
