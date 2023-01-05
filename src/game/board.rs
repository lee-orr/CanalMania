use bevy::{prelude::*, utils::HashMap};
use bevy_mod_picking::{Highlighting, HoverEvent, PickableBundle, PickingEvent};
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::{CurrentState, NextState},
};
use noisy_bevy::simplex_noise_3d;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::{AppLoadingState, AppState},
    assets::CanalManiaAssets,
};

use super::{
    game_state::GameState,
    level::Level,
    tile_shader::{InkSettings, TileMaterial},
};

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
            .add_system(animate_goal.run_in_state(AppState::InGame))
            .add_system(process_selection_events.run_in_state(AppState::InGame))
            .add_exit_system(AppState::InGame, clear_board);
        // #[cfg(feature = "dev")]
        // app.add_plugin(bevy_inspector_egui::quick::AssetInspectorPlugin::<
        //     TileMaterial,
        // >::default());
        // .add_plugin(bevy_inspector_egui::quick::ResourceInspectorPlugin::<
        //     BoardRuntimeAssets,
        // >::default());
    }
}

#[derive(Resource, Reflect)]
struct BoardRuntimeAssets {
    pub tile_base_material: Handle<TileMaterial>,
    pub tile_offset_w_material: Handle<TileMaterial>,
    pub tile_offset_h_material: Handle<TileMaterial>,
    pub tile_offset_wh_material: Handle<TileMaterial>,
    pub decoration_material: Handle<TileMaterial>,
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
    pub contents: TileContents,
    #[serde(default)]
    pub is_goal: bool,
    #[serde(default)]
    pub is_wet: bool,
}

#[derive(Component, Default, Clone, Debug)]
pub struct TileNeighbours(pub [Option<Entity>; 8]);

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileType {
    Land,
    Farm,
    City,
}

impl Default for TileType {
    fn default() -> Self {
        Self::Land
    }
}

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileContents {
    None,
    Road,
    Canal,
    Lock,
    Aquaduct(usize),
}

impl Default for TileContents {
    fn default() -> Self {
        Self::None
    }
}

impl TileContents {
    fn center(&self, assets: &CanalManiaAssets, is_dry: bool) -> Handle<Mesh> {
        match self {
            TileContents::None => Handle::default(),
            TileContents::Road => assets.road_center.clone(),
            TileContents::Canal => {
                if is_dry {
                    assets.canal_dry_center.clone()
                } else {
                    assets.canal_center.clone()
                }
            }
            TileContents::Lock => Handle::default(),
            TileContents::Aquaduct(_) => {
                if is_dry {
                    assets.aquaducts_dry_center.clone()
                } else {
                    assets.aquaduct_center.clone()
                }
            }
        }
    }
    fn corner(&self, assets: &CanalManiaAssets, is_dry: bool) -> Handle<Mesh> {
        match self {
            TileContents::None => Handle::default(),
            TileContents::Road => assets.road_corner.clone(),
            TileContents::Canal => {
                if is_dry {
                    assets.canal_dry_corner.clone()
                } else {
                    assets.canal_corner.clone()
                }
            }
            TileContents::Lock => assets.lock_corner.clone(),
            TileContents::Aquaduct(_) => {
                if is_dry {
                    assets.aquaducts_dry_corner.clone()
                } else {
                    assets.aquaduct_corner.clone()
                }
            }
        }
    }
    fn crossing(&self, assets: &CanalManiaAssets, is_dry: bool) -> Handle<Mesh> {
        info!("Checking {self:?}");
        match self {
            TileContents::None => {
                info!("Providing default handle...");
                Handle::default()
            }
            TileContents::Road => assets.road_crossing.clone(),
            TileContents::Canal => {
                if is_dry {
                    info!("Found Dry Canal Mesh");
                    assets.canal_dry_crossing.clone()
                } else {
                    info!("Found Canal Mesh");
                    assets.canal_crossing.clone()
                }
            }
            TileContents::Lock => assets.lock_crossing.clone(),
            TileContents::Aquaduct(_) => {
                if is_dry {
                    assets.aquaducts_dry_crossing.clone()
                } else {
                    assets.aquaduct_crossing.clone()
                }
            }
        }
    }
    fn t(&self, assets: &CanalManiaAssets, is_dry: bool) -> Handle<Mesh> {
        match self {
            TileContents::None => Handle::default(),
            TileContents::Road => assets.road_t.clone(),
            TileContents::Canal => {
                if is_dry {
                    assets.canal_dry_t.clone()
                } else {
                    assets.canal_t.clone()
                }
            }
            TileContents::Lock => assets.lock_t.clone(),
            TileContents::Aquaduct(_) => {
                if is_dry {
                    assets.aquaducts_dry_t.clone()
                } else {
                    assets.aquaduct_t.clone()
                }
            }
        }
    }
    fn line(&self, assets: &CanalManiaAssets, is_dry: bool) -> Handle<Mesh> {
        match self {
            TileContents::None => Handle::default(),
            TileContents::Road => assets.road_line.clone(),
            TileContents::Canal => {
                if is_dry {
                    assets.canal_dry_line.clone()
                } else {
                    assets.canal_line.clone()
                }
            }
            TileContents::Lock => assets.lock_line.clone(),
            TileContents::Aquaduct(_) => {
                if is_dry {
                    assets.aquaducts_dry_line.clone()
                } else {
                    assets.aquaduct_line.clone()
                }
            }
        }
    }
    fn end(&self, assets: &CanalManiaAssets, is_dry: bool) -> Handle<Mesh> {
        match self {
            TileContents::None => Handle::default(),
            TileContents::Road => assets.road_end.clone(),
            TileContents::Canal => {
                if is_dry {
                    assets.canal_dry_end.clone()
                } else {
                    assets.canal_end.clone()
                }
            }
            TileContents::Lock => assets.lock_end.clone(),
            TileContents::Aquaduct(_) => {
                if is_dry {
                    assets.aquaducts_dry_end.clone()
                } else {
                    assets.aquaduct_end.clone()
                }
            }
        }
    }
}

impl Tile {
    pub fn get_dig_cost(&self) -> usize {
        let type_cost = match self.tile_type {
            TileType::Land => 1000,
            TileType::Farm => 1500,
            TileType::City => 3000,
        };
        let road_cost = if self.contents == TileContents::Road {
            100usize
        } else {
            0
        };
        type_cost + road_cost
    }
    pub fn get_lock_cost(&self) -> usize {
        let type_cost = match self.tile_type {
            TileType::Land => 5000,
            TileType::Farm => 6000,
            TileType::City => 7000,
        };
        let road_cost = if self.contents == TileContents::Road {
            100usize
        } else {
            0
        };
        type_cost + road_cost
    }

    pub fn get_aquaduct_cost(&self) -> usize {
        match self.tile_type {
            TileType::Land => 8000,
            TileType::Farm => 8000,
            TileType::City => 9000,
        }
    }

    pub fn get_demolish_cost(&self) -> usize {
        let construction_cost = match self.contents {
            TileContents::None => 0,
            TileContents::Road => 100,
            TileContents::Canal => 300,
            TileContents::Lock => 400,
            TileContents::Aquaduct(h) => 500 * h,
        };
        let type_multiplier = match self.tile_type {
            TileType::Land => 1,
            TileType::Farm => 2,
            TileType::City => 3,
        };
        construction_cost * type_multiplier
    }

    pub fn get_decorations(&self, assets: &CanalManiaAssets) -> Vec<Handle<Mesh>> {
        let count = match self.contents {
            TileContents::None => match self.tile_type {
                TileType::Land => 3.,
                TileType::Farm => 3.,
                TileType::City => 8.,
            },
            _ => match self.tile_type {
                TileType::Land => 1.,
                TileType::Farm => 1.,
                TileType::City => 4.,
            },
        };

        let mut pos = Vec3::new(self.x as f32, self.y as f32, self.z as f32);

        let amount = (simplex_noise_3d(pos).abs() * (count + 1.)).floor() as usize;

        (0..amount)
            .map(|i| {
                let i = i as f32;
                pos = Vec3::new(-1. * i, 2. * i, 0.24 * i) * pos;
                match self.tile_type {
                    TileType::Land => {
                        let index = (simplex_noise_3d(pos).abs() * 4.).floor() as usize;
                        match index {
                            1 => assets.tree2.clone(),
                            2 => assets.tree3.clone(),
                            3 => assets.tree4.clone(),
                            _ => assets.tree1.clone(),
                        }
                    }
                    TileType::Farm => {
                        let index = (simplex_noise_3d(pos).abs() * 6.).floor() as usize;
                        match index {
                            1 => assets.house2.clone(),
                            2 => assets.house3.clone(),
                            3 => assets.house4.clone(),
                            4 => assets.tree2.clone(),
                            5 => assets.tree3.clone(),
                            _ => assets.house.clone(),
                        }
                    }
                    TileType::City => {
                        let index = (simplex_noise_3d(pos).abs() * 4.).floor() as usize;
                        match index {
                            1 => assets.house2.clone(),
                            2 => assets.house3.clone(),
                            3 => assets.house4.clone(),
                            _ => assets.house.clone(),
                        }
                    }
                }
            })
            .collect()
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
    _assets: Res<CanalManiaAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut tile_materials: ResMut<Assets<TileMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let tile_base_material = tile_materials.add(TileMaterial::default());
    let tile_offset_w_material = tile_materials.add(TileMaterial {
        settings: InkSettings { world_offset: Vec4::new(0.5, 0., 0., 0.), ..Default::default() }
    });
    let tile_offset_h_material = tile_materials.add(TileMaterial {
        settings: InkSettings { world_offset: Vec4::new(0., 0.5, 0., 0.), ..Default::default() }
    });
    let tile_offset_wh_material = tile_materials.add(TileMaterial {
        settings: InkSettings { world_offset: Vec4::new(0.5, 0.5, 0., 0.), ..Default::default() }
    });
    let decoration_material = tile_materials.add(TileMaterial {
        settings: InkSettings {
            added_params: Vec4::new(0., 0.7, 0.5, 0.1),
            ..Default::default()
        },
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
        tile_offset_w_material,
        tile_offset_h_material,
        tile_offset_wh_material,
        decoration_material,
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
    let mut board = Board {
        width: level.width,
        height: level.height,
        ..Default::default()
    };
    commands
        .spawn((SpatialBundle {
            transform: Transform::from_translation(center),
            ..Default::default()
        },))
        .with_children(|parent| {
            for (x, column) in level.tiles.iter().enumerate() {
                for (y, row) in column.iter().enumerate() {
                    let tile = Tile {
                        x,
                        y,
                        z: row.height,
                        tile_type: row.tile_type,
                        is_goal: row.is_goal,
                        contents: row.contents,
                        is_wet: row.is_wet,
                    };
                    let entity = parent.spawn(tile).id();
                    board.children.insert((x, y), entity);
                }
            }
        })
        .insert(board);

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
    tiles: Query<(Entity, &Tile, Option<&TileNeighbours>), Changed<Tile>>,
    neighbour_tiles: Query<(Entity, &Tile, Option<&TileNeighbours>)>,
    boards: Query<&Board>,
) {
    let (offset_x, offset_y) = if let Ok(board) = boards.get_single() {
     (board.width % 2 == 0, board.height % 2 == 0)
    } else {
        (false, false)
    };
    for (entity, tile, neighbours) in tiles.iter() {
        update_tile(
            &neighbours,
            &neighbour_tiles,
            &boards,
            tile,
            &mut commands,
            entity,
            &materials,
            &assets,
            true,
            offset_x, 
            offset_y
        );
    }
}

#[derive(Component)]
pub struct Goal;

fn update_tile(
    neighbours: &Option<&TileNeighbours>,
    neighbour_tiles: &Query<(Entity, &Tile, Option<&TileNeighbours>)>,
    boards: &Query<&Board>,
    tile: &Tile,
    commands: &mut Commands,
    entity: Entity,
    materials: &BoardRuntimeAssets,
    assets: &CanalManiaAssets,
    primary: bool,
    offset_x: bool,
    offset_y: bool
) {
    let neighbours = if let Some(n) = neighbours {
        n.0.iter()
            .map(|e| {
                if let Some(e) = e {
                    neighbour_tiles.get(*e).ok()
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    } else if let Ok(board) = boards.get_single() {
        let n = board.neighbours(tile.x, tile.y);
        commands.entity(entity).insert(TileNeighbours(n));
        n.iter()
            .map(|e| {
                if let Some(e) = e {
                    neighbour_tiles.get(*e).ok()
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    } else {
        (0..8)
            .map(|_| Option::<(Entity, &Tile, Option<&TileNeighbours>)>::None)
            .collect::<Vec<_>>()
    };
    let center = Vec3::new(tile.x as f32, (tile.z as f32) / 6., tile.y as f32);
    let mut entity = commands.entity(entity);
    let base_material = match (offset_x, offset_y) {
        (true, true) => materials.tile_offset_wh_material.clone(),
        (true, false) => materials.tile_offset_w_material.clone(),
        (false, true) => materials.tile_offset_h_material.clone(),
        (false, false) => materials.tile_base_material.clone(),
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
        parent.spawn(MaterialMeshBundle {
            mesh: match tile.tile_type {
                TileType::Land => assets.land_tile.clone(),
                TileType::City => assets.city_tile.clone(),
                TileType::Farm => assets.farm_tile.clone(),
            },
            material: base_material.clone(),
            ..Default::default()
        });

        if tile.is_goal {
            parent.spawn((
                Goal,
                MaterialMeshBundle {
                    mesh: assets.goal.clone(),
                    material: materials.decoration_material.clone(),
                    ..Default::default()
                },
            ));
        }

        let decorations = tile.get_decorations(assets);

        let mut pos = Vec3::new(tile.x as f32, tile.y as f32, tile.z as f32);
        pos *= 4295.;
        for (i, decoration) in decorations.into_iter().enumerate() {
            let i = i as f32;

            pos = Vec3::new(i * 235., -141.5 * i, 9998. * i) + pos;

            let x = simplex_noise_3d(pos);
            pos /= 325.;

            let y = simplex_noise_3d(pos);

            pos = pos / 213. + 5935.;
            let rot = simplex_noise_3d(pos) * 360.;

            let position = match tile.contents {
                TileContents::None => {
                    let x = x.clamp(0.1, 0.9);
                    let y = y.clamp(0.1, 0.9);
                    let x = x - 0.5;
                    let y = y - 0.5;
                    Vec3::new(x, 0., y)
                }
                _ => {
                    let x = if x > 0.5 {
                        x.clamp(0.75, 0.9)
                    } else {
                        x.clamp(0.1, 0.25)
                    };
                    let y = if y > 0.5 {
                        y.clamp(0.75, 0.9)
                    } else {
                        y.clamp(0.1, 0.25)
                    };
                    let x = x - 0.5;
                    let y = y - 0.5;
                    Vec3::new(x, 0., y)
                }
            };
            parent.spawn(MaterialMeshBundle {
                mesh: decoration,
                material: materials.decoration_material.clone(),
                transform: Transform::from_translation(position)
                    .with_rotation(Quat::from_rotation_y(rot.to_radians()))
                    .with_scale(Vec3::new(0.3, 0.3, 0.3)),
                ..Default::default()
            });
        }

        spawn_content(tile, &neighbours, assets, parent, base_material.clone());
    });

    if primary {
        for (entity, tile, neighbours) in neighbours.iter().flatten() {
            update_tile(
                neighbours,
                neighbour_tiles,
                boards,
                tile,
                commands,
                *entity,
                materials,
                assets,
                false,
                offset_x,
                offset_y
            );
        }
    };
}

fn spawn_content(
    tile: &Tile,
    neighbours: &[Option<(Entity, &Tile, Option<&TileNeighbours>)>],
    assets: &CanalManiaAssets,
    parent: &mut ChildBuilder,
    base_material: Handle<TileMaterial>,
) {
    match tile.contents {
        TileContents::None => {}
        TileContents::Road => {
            println!("Setting up road!");
            let neighbours = check_neighbours(neighbours, |t| t.contents == TileContents::Road);

            let n = neighbours[1];
            let w = neighbours[3];
            let e = neighbours[4];
            let s = neighbours[6];

            spawn_variant(
                TileContents::Road,
                !tile.is_wet,
                assets,
                n,
                w,
                e,
                s,
                parent,
                base_material,
            );
        }
        TileContents::Canal => {
            println!("Setting up canal!");
            let neighbours = check_neighbours(neighbours, |t| {
                matches!(t.contents, TileContents::Canal) && tile.z.abs_diff(t.z) < 2
                    || matches!(t.contents, TileContents::Lock) && tile.z.abs_diff(t.z) < 5
                    || if let TileContents::Aquaduct(h) = t.contents {
                        tile.z == h + t.z
                    } else {
                        false
                    }
            });

            let n = neighbours[1];
            let w = neighbours[3];
            let e = neighbours[4];
            let s = neighbours[6];

            spawn_variant(
                TileContents::Canal,
                !tile.is_wet,
                assets,
                n,
                w,
                e,
                s,
                parent,
                base_material,
            );
        }
        TileContents::Lock => {
            println!("Setting up lock!");
            let neighbours = check_neighbours(neighbours, |t| {
                matches!(t.contents, TileContents::Canal | TileContents::Lock)
                    && tile.z.abs_diff(t.z) < 5
                    || if let TileContents::Aquaduct(h) = t.contents {
                        tile.z == h + t.z
                    } else {
                        false
                    }
            });

            let n = neighbours[1];
            let w = neighbours[3];
            let e = neighbours[4];
            let s = neighbours[6];

            spawn_variant(
                TileContents::Canal,
                !tile.is_wet,
                assets,
                n,
                w,
                e,
                s,
                parent,
                base_material.clone(),
            );
            spawn_variant(
                TileContents::Lock,
                !tile.is_wet,
                assets,
                n,
                w,
                e,
                s,
                parent,
                base_material,
            );
        }
        TileContents::Aquaduct(h) => {
            println!("Setting up aquaduct {h:?}");
            let z = tile.z + h;
            let neighbours = check_neighbours(neighbours, |t| {
                matches!(t.contents, TileContents::Canal | TileContents::Lock) && z == t.z
                    || if let TileContents::Aquaduct(h) = t.contents {
                        z == h + t.z
                    } else {
                        false
                    }
            });

            let n = neighbours[1];
            let w = neighbours[3];
            let e = neighbours[4];
            let s = neighbours[6];

            spawn_variant(
                TileContents::Aquaduct(h),
                !tile.is_wet,
                assets,
                n,
                w,
                e,
                s,
                parent,
                base_material,
            );
        }
    }
}

pub fn check_neighbours<F: Fn(&Tile) -> bool, R>(
    neighbours: &[Option<(Entity, &Tile, R)>],
    checked: F,
) -> [bool; 8] {
    let mut result = [false; 8];

    #[allow(clippy::needless_range_loop)]
    for i in 0..8 {
        if let Some(Some((_, neighbour, _))) = neighbours.get(i) {
            result[i] = checked(neighbour);
        }
    }
    result
}

fn spawn_variant<T: Material>(
    content_type: TileContents,
    is_dry: bool,
    assets: &CanalManiaAssets,
    n: bool,
    w: bool,
    e: bool,
    s: bool,
    parent: &mut ChildBuilder,
    material: Handle<T>,
) {
    let (mesh, rotation) = match (n, w, e, s) {
        (true, true, true, true) => (content_type.crossing(assets, is_dry), 0f32),
        (true, true, true, false) => (content_type.t(assets, is_dry), 180.),
        (true, true, false, true) => (content_type.t(assets, is_dry), 270.),
        (true, true, false, false) => (content_type.corner(assets, is_dry), 180.),
        (true, false, true, true) => (content_type.t(assets, is_dry), 90.),
        (true, false, true, false) => (content_type.corner(assets, is_dry), 90.),
        (true, false, false, true) => (content_type.line(assets, is_dry), 0.),
        (true, false, false, false) => (content_type.end(assets, is_dry), 180.),
        (false, true, true, true) => (content_type.t(assets, is_dry), 0.),
        (false, true, true, false) => (content_type.line(assets, is_dry), 90.),
        (false, true, false, true) => (content_type.corner(assets, is_dry), 270.),
        (false, true, false, false) => (content_type.end(assets, is_dry), 270.),
        (false, false, true, true) => (content_type.corner(assets, is_dry), 0.),
        (false, false, true, false) => (content_type.end(assets, is_dry), 90.),
        (false, false, false, true) => (content_type.end(assets, is_dry), 0.),
        (false, false, false, false) => (content_type.center(assets, is_dry), 0.),
    };

    println!("Spawning {content_type:?}");
    let position = match content_type {
        TileContents::Aquaduct(u) => Vec3::Y * (u as f32 - 1.) / 6.,
        _ => Vec3::ZERO,
    };
    parent.spawn(MaterialMeshBundle {
        mesh,
        material,
        transform: Transform::from_rotation(Quat::from_rotation_y(rotation.to_radians()))
            .with_translation(position),
        ..Default::default()
    });
}

#[derive(Clone)]
pub enum TileEvent {
    Clicked(Tile, Entity),
    HoverStarted(Tile, Entity),
    HoverEnded(Tile, Entity),
}

pub(crate) fn process_selection_events(
    mut events: EventReader<PickingEvent>,
    mut out_events: EventWriter<TileEvent>,
    tiles: Query<&Tile>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(_) => {}
            PickingEvent::Hover(e) => match e {
                HoverEvent::JustEntered(e) => {
                    if let Ok(tile) = tiles.get(*e) {
                        out_events.send(TileEvent::HoverStarted(tile.clone(), *e));
                    }
                }
                HoverEvent::JustLeft(e) => {
                    if let Ok(tile) = tiles.get(*e) {
                        out_events.send(TileEvent::HoverEnded(tile.clone(), *e));
                    }
                }
            },
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

fn animate_goal(mut goals: Query<&mut Transform, With<Goal>>, time: Res<Time>) {
    for (i, mut goal) in goals.iter_mut().enumerate() {
        let y = (time.elapsed_seconds_f64() * std::f64::consts::PI / 2. + (i as f64)).sin() as f32;
        let y = y * 0.3 + 0.3;
        goal.translation = Vec3::new(0., y, 0.);
    }
}
