use bevy::prelude::*;
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::CurrentState,
};

use crate::{assets::CanalManiaAssets, ui::*};

use super::{
    board::TileEvent,
    game_state::{GameActionMode, GameState},
};

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

fn setup_tooltip(mut commands: Commands, asset: Res<CanalManiaAssets>) {
    commands
        .ui_root()
        .for_state(GameState::InGame)
        .id(HoverUiId::Root)
        .position(Val::Px(-1000.), Val::Auto, Val::Px(-1000.), Val::Auto)
        .with_children(|parent| {
            parent
                .div()
                .size(Size::new(Val::Px(150.), Val::Auto))
                .position(Val::Auto, Val::Auto, Val::Auto, Val::Px(20.))
                .opaque()
                .with_children(|parent| {
                    parent.text("").size(15.).id(HoverUiId::MainText);
                    parent.div().horizontal().with_children(|parent| {
                        parent
                            .icon(asset.coin_icon.clone())
                            .size(GameIconSize::Small);
                        parent.text("").size(12.).id(HoverUiId::SecondaryText);
                    });
                });
        });
}

fn update_tile_hover_ui(
    mut events: EventReader<TileEvent>,
    cameras: Query<Entity, With<Camera>>,
    mut tooltip_root: Query<(&mut UiRoot, &UiId<HoverUiId>)>,
    mut tooltip_text: Query<(&mut GameText, &UiId<HoverUiId>)>,
    operation: Res<CurrentState<GameActionMode>>,
) {
    if let (Ok(camera), Ok((mut root, _))) = (cameras.get_single(), tooltip_root.get_single_mut()) {
        for event in events.iter() {
            match event {
                TileEvent::HoverStarted(tile, entity) => {
                    root.world_position(*entity, camera);

                    let cost = match operation.0 {
                        GameActionMode::None => None,
                        GameActionMode::DigCanal => Some(tile.get_dig_cost()),
                        GameActionMode::ConstructLock => Some(tile.get_lock_cost()),
                        GameActionMode::BuildAquaduct => Some(tile.get_aquaduct_cost()),
                        GameActionMode::Demolish => Some(tile.get_demolish_cost()),
                    };

                    let tile_type = match tile.tile_type {
                        super::board::TileType::Land => "A Plot of Land",
                        super::board::TileType::City => "A Constructed Area",
                        super::board::TileType::Farm => "Farmland",
                    };

                    for (mut text, id) in tooltip_text.iter_mut() {
                        match id.val() {
                            HoverUiId::Root => {}
                            HoverUiId::MainText => {
                                text.text(tile_type);
                            }
                            HoverUiId::SecondaryText => {
                                text.text(match cost {
                                    Some(cost) => cost.to_string(),
                                    None => 0.to_string(),
                                });
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
