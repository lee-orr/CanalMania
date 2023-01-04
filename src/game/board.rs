use bevy::{prelude::*, utils::HashMap};
use bevy_mod_picking::{Highlighting, HoverEvent, PickableBundle, PickingEvent};
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::{CurrentState, NextState},
};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::{AppLoadingState, AppState},
    assets::CanalManiaAssets,
};

use super::{game_state::GameState, level::Level};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Board>()
            .register_type::<Tile>()
            .register_type::<TileType>()
            .add_event::<TileEvent>()
            .add_enter_system(AppLoadingState::Loaded, setup_board_materials)
            .add_system(build_board.run_in_state(AppState::InGame))
            .add_system(build_tile.run_in_state(AppState::InGame))
            .add_system(process_selection_events.run_in_state(AppState::InGame))
            .add_enter_system(GameState::Complete, clear_board)
            .add_exit_system(AppState::InGame, clear_board);
    }
}

#[derive(Resource)]
struct BoardRuntimeAssets {
    pub tile_base_material: Handle<StandardMaterial>,
    pub goal_base_material: Handle<StandardMaterial>,
    pub selector: Handle<Mesh>,
    pub selector_base: Handle<StandardMaterial>,
    pub selector_hovered: Handle<StandardMaterial>,
    pub selector_pressed: Handle<StandardMaterial>,
    pub selector_selected: Handle<StandardMaterial>,
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub children: HashMap<(usize, usize), Entity>,
}

impl Board {
    pub fn neighbour_ids(&self, x: usize, y: usize) -> [Option<(usize, usize)>; 8] {
        let above = y.checked_sub(1);
        let left = x.checked_sub(1);
        let center_x = Some(x);
        let center_y = Some(y);
        let below = if y + 1 < self.height {
            Some(y + 1)
        } else {
            None
        };
        let right = if x + 1 < self.width {
            Some(x + 1)
        } else {
            None
        };

        [
            tile_position(left, above),
            tile_position(center_x, above),
            tile_position(right, above),
            tile_position(left, center_y),
            tile_position(right, center_y),
            tile_position(left, below),
            tile_position(center_x, below),
            tile_position(right, below),
        ]
    }

    pub fn neighbours(&self, x: usize, y: usize) -> [Option<Entity>; 8] {
        self.neighbour_ids(x, y).map(|p| match p {
            Some(p) => self.children.get(&p).cloned(),
            None => None,
        })
    }
}

#[derive(Component, Default, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    #[serde(default)]
    pub z: usize,
    #[serde(default)]
    pub tile_type: TileType,
    #[serde(default)]
    pub is_goal: bool,
}

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileType {
    Land,
    City,
    CanalDry,
    CanalWet,
    LockDry,
    LockWet,
}

impl Default for TileType {
    fn default() -> Self {
        Self::Land
    }
}

impl Tile {
    pub fn get_dig_cost(&self) -> usize {
        match self.tile_type {
            TileType::Land => 1,
            TileType::City => 3,
            TileType::CanalDry => 0,
            TileType::CanalWet => 0,
            TileType::LockDry => 5,
            TileType::LockWet => 5,
        }
    }
    pub fn get_lock_cost(&self) -> usize {
        match self.tile_type {
            TileType::Land => 5,
            TileType::City => 7,
            TileType::CanalDry => 5,
            TileType::CanalWet => 5,
            TileType::LockDry => 0,
            TileType::LockWet => 0,
        }
    }
}

fn tile_position(x: Option<usize>, y: Option<usize>) -> Option<(usize, usize)> {
    if let (Some(x), Some(y)) = (x, y) {
        Some((x, y))
    } else {
        None
    }
}

fn setup_board_materials(
    mut commands: Commands,
    assets: Res<CanalManiaAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let tile_base_material = materials.add(StandardMaterial {
        base_color_texture: Some(assets.tile_texture.clone()),
        ..Default::default()
    });
    let goal_base_material = materials.add(StandardMaterial {
        base_color_texture: Some(assets.tile_texture.clone()),
        base_color: Color::rgb(0.7, 0.2, 0.1),
        ..Default::default()
    });
    let selector = meshes.add(shape::Box::new(1., 0.1, 1.).into());

    let selector_base = materials.add(StandardMaterial {
        base_color: Color::rgba(0., 0., 0., 0.0),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    let selector_hovered = materials.add(StandardMaterial {
        base_color: Color::rgba(0., 0.7, 0.5, 0.5),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    let selector_pressed = materials.add(StandardMaterial {
        base_color: Color::rgba(0., 0.7, 0.5, 0.7),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    let selector_selected = materials.add(StandardMaterial {
        base_color: Color::rgba(0., 0.9, 0.7, 0.5),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    commands.insert_resource(BoardRuntimeAssets {
        tile_base_material,
        goal_base_material,
        selector,
        selector_base,
        selector_hovered,
        selector_pressed,
        selector_selected,
    });
}

fn build_board(
    mut commands: Commands,
    level: Res<Level>,
    boards: Query<Entity, With<Board>>,
    state: Res<CurrentState<GameState>>,
) {
    if !level.is_changed() {
        return;
    }
    for board in boards.iter() {
        commands.entity(board).despawn_recursive();
    }
    let center = Vec3::new(
        -1. * (level.width as f32) / 2.,
        0.,
        -1. * (level.height as f32) / 2.,
    );
    commands
        .spawn((
            Board {
                width: level.width,
                height: level.height,
                ..Default::default()
            },
            SpatialBundle {
                transform: Transform::from_translation(center),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            for (x, column) in level.tiles.iter().enumerate() {
                for (y, row) in column.iter().enumerate() {
                    let tile = Tile {
                        x,
                        y,
                        z: row.height,
                        tile_type: row.tile_type,
                        is_goal: row.is_goal,
                    };
                    parent.spawn(tile);
                }
            }
        });

    match state.0 {
        GameState::Editor => {}
        _ => {
            commands.insert_resource(NextState(GameState::InGame));
        }
    }
}

fn build_tile(
    mut commands: Commands,
    assets: Res<CanalManiaAssets>,
    materials: Res<BoardRuntimeAssets>,
    tiles: Query<(Entity, &Tile, &Parent), Changed<Tile>>,
    mut boards: Query<&mut Board>,
) {
    for (entity, tile, parent) in tiles.iter() {
        if let Ok(mut parent) = boards.get_mut(parent.get()) {
            parent.children.insert((tile.x, tile.y), entity);
        }
        let center = Vec3::new(tile.x as f32, (tile.z as f32) / 6., tile.y as f32);
        let mut entity = commands.entity(entity);

        let base_material = if tile.is_goal {
            materials.goal_base_material.clone()
        } else {
            materials.tile_base_material.clone()
        };

        entity.insert((
            PickableBundle::default(),
            Highlighting {
                initial: materials.selector_base.clone(),
                hovered: Some(materials.selector_hovered.clone()),
                pressed: Some(materials.selector_pressed.clone()),
                selected: Some(materials.selector_selected.clone()),
            },
            PbrBundle {
                mesh: materials.selector.clone(),
                material: materials.selector_base.clone(),
                transform: Transform::from_translation(center),
                ..Default::default()
            },
        ));
        entity.despawn_descendants();
        entity.with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: match tile.tile_type {
                    TileType::Land => assets.tile_center.clone(),
                    TileType::City => assets.city_center.clone(),
                    TileType::CanalDry => assets.canal_center.clone(),
                    TileType::CanalWet => assets.canal_wet_center.clone(),
                    TileType::LockDry => assets.lock_center.clone(),
                    TileType::LockWet => assets.lock_wet_center.clone(),
                },
                material: base_material.clone(),
                ..Default::default()
            });
            for i in 0..4 {
                parent.spawn(PbrBundle {
                    mesh: match tile.tile_type {
                        TileType::Land => assets.tile_corner.clone(),
                        TileType::City => assets.city_corner.clone(),
                        TileType::CanalDry => assets.canal_corner.clone(),
                        TileType::CanalWet => assets.canal_wet_corner.clone(),
                        TileType::LockDry => assets.lock_corner.clone(),
                        TileType::LockWet => assets.lock_wet_corner.clone(),
                    },
                    material: base_material.clone(),
                    transform: Transform::from_rotation(Quat::from_rotation_y(
                        ((i as f32) * 90.).to_radians(),
                    )),
                    ..Default::default()
                });
                parent.spawn(PbrBundle {
                    mesh: match tile.tile_type {
                        TileType::Land => assets.tile_edge.clone(),
                        TileType::City => assets.city_edge.clone(),
                        TileType::CanalDry => assets.canal_edge.clone(),
                        TileType::CanalWet => assets.canal_wet_edge.clone(),
                        TileType::LockDry => assets.lock_edge.clone(),
                        TileType::LockWet => assets.lock_wet_edge.clone(),
                    },
                    material: base_material.clone(),
                    transform: Transform::from_rotation(Quat::from_rotation_y(
                        ((i as f32) * 90.).to_radians(),
                    )),
                    ..Default::default()
                });
            }
        });
    }
}

#[derive(Clone)]
pub enum TileEvent {
    Clicked(Tile, Entity),
    HoverStarted(Tile, Entity),
}

pub(crate) fn process_selection_events(
    mut events: EventReader<PickingEvent>,
    mut out_events: EventWriter<TileEvent>,
    tiles: Query<&Tile>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => {
                if let HoverEvent::JustEntered(e) = e {
                    if let Ok(tile) = tiles.get(*e) {
                        out_events.send(TileEvent::HoverStarted(tile.clone(), *e));
                    }
                }
            }
            PickingEvent::Clicked(e) => {
                if let Ok(tile) = tiles.get(*e) {
                    out_events.send(TileEvent::Clicked(tile.clone(), *e));
                }
            }
        }
    }
}

fn clear_board(mut commands: Commands, boards: Query<Entity, With<Board>>) {
    for board in boards.iter() {
        commands.entity(board).despawn_recursive();
    }
}
