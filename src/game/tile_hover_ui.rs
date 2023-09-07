use bevy::prelude::*;

use crate::{assets::CanalManiaAssets, ui::*};

use super::{
    board::TileEvent,
    game_state::{GameActionMode, GameState},
};

pub struct TileHoverUi;

impl Plugin for TileHoverUi {
    fn build(&self, app: &mut App) {
        clear_ui_system_set(app, GameState::InGame)
            .add_systems(OnEnter(GameState::InGame), setup_tooltip)
            .add_systems(
                Update,
                update_tile_hover_ui.run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
enum HoverUiId {
    Root,
    MainText,
    SecondaryText,
    CoinIcon,
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
                .size((Val::Px(300.), Val::Auto))
                .position(Val::Auto, Val::Auto, Val::Auto, Val::Px(20.))
                .opaque()
                .with_children(|parent| {
                    parent.div().horizontal().with_children(|parent| {
                        parent
                            .icon(asset.coin_icon.clone())
                            .size(GameIconSize::Small)
                            .id(HoverUiId::CoinIcon);
                        parent.text("").size(18.).id(HoverUiId::MainText);
                    });
                    parent.text("").size(12.).id(HoverUiId::SecondaryText);
                });
        });
}

fn update_tile_hover_ui(
    mut events: EventReader<TileEvent>,
    cameras: Query<Entity, With<Camera>>,
    mut tooltip_root: Query<(&mut UiRoot, &UiId<HoverUiId>)>,
    mut tooltip_text: Query<(&mut GameText, &UiId<HoverUiId>)>,
    mut coin_icon: Query<(&mut Style, &GameIcon, &UiId<HoverUiId>)>,
    operation: Res<State<GameActionMode>>,
) {
    if let (Ok(camera), Ok((mut root, _))) = (cameras.get_single(), tooltip_root.get_single_mut()) {
        for event in events.iter() {
            match event {
                TileEvent::HoverStarted(tile, entity) => {
                    root.world_position(*entity, camera);

                    let cost = match operation.get() {
                        GameActionMode::None => None,
                        GameActionMode::DigCanal => Some(tile.get_dig_cost()),
                        GameActionMode::ConstructLock => Some(tile.get_lock_cost()),
                        GameActionMode::BuildAquaduct => Some(tile.get_aquaduct_cost()),
                        GameActionMode::Demolish => Some(tile.get_demolish_cost()),
                    };

                    let tile_type = match tile.tile_type {
                        super::board::TileType::Land => "Open Land",
                        super::board::TileType::City => "Town",
                        super::board::TileType::Farm => "Farmland",
                        super::board::TileType::Sea => "Water",
                    };

                    let secondary_text = format!(
                        "{}{} with {}",
                        match tile.contents {
                            super::board::TileContents::None => "",
                            super::board::TileContents::Road => "A Road on ",
                            super::board::TileContents::Canal => "A Canal on ",
                            super::board::TileContents::Lock => "A Lock on ",
                            super::board::TileContents::Aquaduct(_) => "An Aquaduct on ",
                            super::board::TileContents::River => "A River on ",
                        },
                        tile_type,
                        match tile.wetness {
                            super::board::Wetness::Dry => "No Water Flow",
                            super::board::Wetness::WaterSource => "a Water Source",
                            super::board::Wetness::Wet(_) => "Running Water",
                        }
                    );
                    #[cfg(feature = "dev")]
                    let secondary_text = format!("{secondary_text}\n{},{}", tile.x, tile.y);

                    let primary_text = format!(
                        "{} {:?} Meters High",
                        match cost {
                            Some(Some(v)) => v.to_string(),
                            _ => "".to_string(),
                        },
                        tile.z * 20
                    );

                    for (mut text, id) in tooltip_text.iter_mut() {
                        match id.val() {
                            HoverUiId::MainText => {
                                text.text(&primary_text);
                            }
                            HoverUiId::SecondaryText => {
                                text.text(&secondary_text);
                            }
                            _ => {}
                        }
                    }

                    for (mut style, _icon, id) in coin_icon.iter_mut() {
                        if &HoverUiId::CoinIcon == id.val() {
                            match cost {
                                Some(Some(_)) => {
                                    style.display = Display::Flex;
                                }
                                _ => {
                                    style.display = Display::None;
                                }
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
