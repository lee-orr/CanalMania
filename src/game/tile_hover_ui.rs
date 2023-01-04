use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};

use crate::ui::*;

use super::{board::TileEvent, game_state::GameState};

pub struct TileHoverUi;

impl Plugin for TileHoverUi {
    fn build(&self, app: &mut App) {
        clear_ui_system_set(app, GameState::InGame)
            .add_enter_system(GameState::InGame, setup_tooltip)
            .add_system(update_tile_hover_ui.run_in_state(GameState::InGame));
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
enum HoverUiId {
    Root,
    MainText,
    SecondaryText,
}

fn setup_tooltip(mut commands: Commands) {
    commands
        .ui_root()
        .for_state(GameState::InGame)
        .id(HoverUiId::Root)
        .position(Val::Px(-1000.), Val::Auto, Val::Px(-1000.), Val::Auto)
        .with_children(|parent| {
            parent
                .div()
                .size(Size::new(Val::Px(400.), Val::Auto))
                .position(Val::Auto, Val::Auto, Val::Auto, Val::Px(20.))
                .opaque()
                .with_children(|parent| {
                    parent.text("").size(15.).id(HoverUiId::MainText);
                    parent.text("").size(12.).id(HoverUiId::SecondaryText);
                });
        });
}

fn update_tile_hover_ui(
    _commands: Commands,
    mut events: EventReader<TileEvent>,
    cameras: Query<Entity, With<Camera>>,
    mut tooltip_root: Query<(&mut UiRoot, &UiId<HoverUiId>)>,
    mut tooltip_text: Query<(&mut GameText, &UiId<HoverUiId>)>,
) {
    if let (Ok(camera), Ok((mut root, _))) = (cameras.get_single(), tooltip_root.get_single_mut()) {
        for event in events.iter() {
            match event {
                TileEvent::HoverStarted(tile, entity) => {
                    root.world_position(*entity, camera);

                    let cost_to_dig = tile.get_dig_cost();
                    let cost_to_lock = tile.get_lock_cost();

                    let (tile_type, display_dig, display_lock) = match tile.tile_type {
                        super::board::TileType::Land => ("A Plot of Land", true, true),
                        super::board::TileType::City => ("A Constructed Area", true, true),
                        super::board::TileType::CanalDry => ("A Dry Canal", false, true),
                        super::board::TileType::CanalWet => {
                            ("A Functioning Waterway", false, false)
                        }
                        super::board::TileType::LockDry => ("A Dry Lock", false, false),
                        super::board::TileType::LockWet => ("An Active Lock", false, false),
                        super::board::TileType::Farm => ("Farmland", true, true),
                        super::board::TileType::Road => ("A Road", true, true),
                    };

                    let cost_to_dig = if display_dig {
                        format!("Dig Cost: {cost_to_dig} Pounds")
                    } else {
                        "".to_string()
                    };

                    let cost_to_lock = if display_lock {
                        format!("Lock Cost: {cost_to_lock} Pounds")
                    } else {
                        "".to_string()
                    };

                    let height = if tile.z > 0 {
                        format!("Height: {}0 Meters", tile.z)
                    } else {
                        String::new()
                    };

                    for (mut text, id) in tooltip_text.iter_mut() {
                        match id.val() {
                            HoverUiId::Root => {}
                            HoverUiId::MainText => {
                                text.text(tile_type);
                            }
                            HoverUiId::SecondaryText => {
                                text.text(format!("{cost_to_dig} {cost_to_lock} {height}"));
                            }
                        }
                    }
                }
                TileEvent::HoverEnded(_tile, entity) => {
                    if let UiRootType::World { track, camera: _ } = root.ui_root_type {
                        if track != *entity {
                            continue;
                        }
                    }
                    root.position(Val::Px(-100.), Val::Auto, Val::Px(-100.), Val::Auto);
                }
                _ => {}
            }
        }
    }
}
