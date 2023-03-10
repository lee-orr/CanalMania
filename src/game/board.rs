mod board_runtime_assets;
mod tile;

use bevy::{
    prelude::*,
    render::render_resource::{
        AddressMode, Extent3d, FilterMode, SamplerDescriptor, TextureDimension, TextureFormat,
    },
    utils::HashMap,
};
use bevy_mod_picking::{Highlighting, HoverEvent, PickableBundle, PickingEvent};
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::{CurrentState, NextState},
};
use noisy_bevy::simplex_noise_3d;

use crate::{
    app_state::{AppLoadingState, AppState},
    assets::CanalManiaAssets,
};

use super::{
    game_state::GameState, initial_description::CurrentDescription, level::Level,
    tile_shader::TileMaterial,
};

pub use board_runtime_assets::*;

pub use tile::*;

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
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub children: HashMap<(usize, usize), Entity>,
}

impl Board {
    pub fn neighbour_ids(&self, x: usize, y: usize) -> [Option<(usize, usize)>; 4] {
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
            tile_position(center_x, above),
            tile_position(left, center_y),
            tile_position(right, center_y),
            tile_position(center_x, below),
        ]
    }

    pub fn neighbours(&self, x: usize, y: usize) -> [Option<Entity>; 4] {
        self.neighbour_ids(x, y).map(|p| match p {
            Some(p) => self.children.get(&p).cloned(),
            None => None,
        })
    }
}

fn build_board(
    mut commands: Commands,
    level: Res<Level>,
    boards: Query<Entity, With<Board>>,
    state: Res<CurrentState<GameState>>,
    board_assets: Res<BoardRuntimeAssets>,
    mut materials: ResMut<Assets<TileMaterial>>,
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

    let (offset_x, offset_y) = (board.width % 2 == 0, board.height % 2 == 0);

    if let Some(material) = materials.get_mut(&board_assets.tile_base_material) {
        material.settings.world_offset.x = if offset_x { 0.5 } else { 0. };
        material.settings.world_offset.z = if offset_y { 0.5 } else { 0. };
        material.settings.size = Vec4::new(board.width as f32, 0., board.height as f32, 0.);
    }

    if let Some(material) = materials.get_mut(&board_assets.decoration_material) {
        material.settings.world_offset.x = if offset_x { 0.5 } else { 0. };
        material.settings.world_offset.z = if offset_y { 0.5 } else { 0. };
        material.settings.size = Vec4::new(board.width as f32, 0., board.height as f32, 0.);
    }

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
                        wetness: if row.contents == TileContents::River
                            || row.tile_type == TileType::Sea
                        {
                            Wetness::WaterSource
                        } else {
                            Wetness::Dry
                        },
                        cost_modifier: row.cost_modifier,
                    };
                    let source = if tile.wetness == Wetness::WaterSource {
                        WetnessSource::Source(x, y)
                    } else {
                        WetnessSource::None
                    };
                    let entity = parent.spawn((tile, source)).id();
                    board.children.insert((x, y), entity);
                }
            }
        })
        .insert(board);

    match state.0 {
        GameState::Editor => {}
        _ => {
            if level.initial_description.is_none() {
                commands.insert_resource(NextState(GameState::InGame));
            } else {
                commands.insert_resource(CurrentDescription {
                    title: level.title.clone(),
                    text: level.initial_description.clone(),
                    continue_button: Some("Play".into()),
                });
                commands.insert_resource(NextState(GameState::Description));
            }
        }
    }
}

const TILE_HEIGHT: u8 = u8::MAX / 10;

fn build_tile(
    mut commands: Commands,
    assets: Res<CanalManiaAssets>,
    board_assets: Res<BoardRuntimeAssets>,
    tiles: Query<(Entity, &Tile, Option<&TileNeighbours>), Changed<Tile>>,
    neighbour_tiles: Query<(Entity, &Tile, Option<&TileNeighbours>)>,
    boards: Query<&Board>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<TileMaterial>>,
) {
    let mut updated = false;
    for (entity, tile, neighbours) in tiles.iter() {
        updated = true;
        update_tile(
            &neighbours,
            &neighbour_tiles,
            &boards,
            tile,
            &mut commands,
            entity,
            &board_assets,
            &assets,
            true,
        );
    }
    if updated {
        if let Ok(board) = boards.get_single() {
            let width = board.width;
            let height = board.height;
            let mut content = vec![(0u8, false, false, false); width * height];

            for (_, tile, _) in neighbour_tiles.iter() {
                let x = tile.x;
                let y = tile.y;

                let height = if tile.tile_type == TileType::Sea {
                    0
                } else {
                    tile.z + 1
                };
                let is_wet = matches!(tile.wetness, Wetness::WaterSource | Wetness::Wet(_));

                let i = y * width + x;

                if let Some(content) = content.get_mut(i) {
                    content.0 = height as u8;
                    if is_wet {
                        content.1 = true;
                    }

                    match tile.cost_modifier {
                        TileCostModifier::None => {}
                        TileCostModifier::Multiplier => {
                            content.3 = true;
                        }
                        TileCostModifier::Blocked => {
                            content.2 = true;
                        }
                    }
                }
            }

            let size = Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            };
            let format = TextureFormat::Rgba8Unorm;
            let data = content
                .iter()
                .flat_map(|(height, is_wet, blocked, cost_modifier)| {
                    [
                        *height * TILE_HEIGHT,
                        if *is_wet { u8::MAX } else { u8::MIN },
                        if *blocked { u8::MAX } else { u8::MIN },
                        if *cost_modifier { u8::MAX } else { u8::MIN },
                    ]
                })
                .collect::<Vec<_>>();
            let mut image = Image::new(size, TextureDimension::D2, data, format);
            image.sampler_descriptor =
                bevy::render::texture::ImageSampler::Descriptor(SamplerDescriptor {
                    address_mode_u: AddressMode::ClampToEdge,
                    address_mode_v: AddressMode::ClampToEdge,
                    mag_filter: FilterMode::Linear,
                    min_filter: FilterMode::Linear,
                    mipmap_filter: FilterMode::Linear,
                    ..Default::default()
                });

            let result = images.set(board_assets.tile_info_map.clone(), image);

            let (offset_x, offset_y) = (board.width % 2 == 0, board.height % 2 == 0);

            if let Some(material) = materials.get_mut(&board_assets.tile_base_material) {
                material.settings.world_offset.x = if offset_x { 0.5 } else { 0. };
                material.settings.world_offset.z = if offset_y { 0.5 } else { 0. };
                material.settings.size =
                    Vec4::new(board.width as f32, 0., board.height as f32, 0.9);
                material.info_map = result.clone();
            }

            if let Some(material) = materials.get_mut(&board_assets.decoration_material) {
                material.settings.world_offset.x = if offset_x { 0.5 } else { 0. };
                material.settings.world_offset.z = if offset_y { 0.5 } else { 0. };
                material.settings.size =
                    Vec4::new(board.width as f32, 0., board.height as f32, 0.2);
                material.info_map = result;
            }
        }
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
    let base_material = materials.tile_base_material.clone();
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
        parent
            .spawn(SpatialBundle::from_transform(Transform::from_xyz(
                0.,
                -1. * center.y,
                0.,
            )))
            .with_children(|parent| {
                parent.spawn(MaterialMeshBundle {
                    mesh: match tile.tile_type {
                        TileType::Land => assets.land_tile.clone(),
                        TileType::City => assets.city_tile.clone(),
                        TileType::Farm => assets.farm_tile.clone(),
                        TileType::Sea => assets.sea_tile.clone(),
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
                            .with_rotation(Quat::from_rotation_y(rot.to_radians())),
                        ..Default::default()
                    });
                }

                spawn_content(
                    tile,
                    &neighbours,
                    assets,
                    parent,
                    materials.decoration_material.clone(),
                );
            });
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
    let is_wet = matches!(tile.wetness, Wetness::WaterSource | Wetness::Wet(_));
    match tile.contents {
        TileContents::None => {}
        TileContents::Road => {
            let neighbours = check_neighbours(
                neighbours,
                |t| t.contents == TileContents::Road,
                |t, _, _| t.wetness,
            );

            let n = neighbours[0];
            let w = neighbours[1];
            let e = neighbours[2];
            let s = neighbours[3];

            spawn_variant(tile, !is_wet, assets, n, w, e, s, parent, base_material);
        }
        TileContents::Canal => {
            let neighbours = check_neighbours(
                neighbours,
                |t| {
                    matches!(t.contents, TileContents::Canal | TileContents::River)
                        && tile.z.abs_diff(t.z) < 1
                        || matches!(t.contents, TileContents::Lock) && tile.z.abs_diff(t.z) < 5
                        || if let TileContents::Aquaduct(h) = t.contents {
                            tile.z == h + t.z
                        } else {
                            false
                        }
                },
                |t, _, _| t.wetness,
            );

            let n = neighbours[0];
            let w = neighbours[1];
            let e = neighbours[2];
            let s = neighbours[3];

            spawn_variant(tile, !is_wet, assets, n, w, e, s, parent, base_material);
        }
        TileContents::River => {
            let neighbours = check_neighbours(
                neighbours,
                |t| {
                    matches!(t.contents, TileContents::Canal) && tile.z.abs_diff(t.z) < 2
                        || matches!(t.contents, TileContents::Lock) && tile.z.abs_diff(t.z) < 5
                        || matches!(t.contents, TileContents::River)
                        || if let TileContents::Aquaduct(h) = t.contents {
                            tile.z == h + t.z
                        } else {
                            false
                        }
                },
                |t, _, _| t.wetness,
            );

            let n = neighbours[0];
            let w = neighbours[1];
            let e = neighbours[2];
            let s = neighbours[3];

            spawn_variant(tile, !is_wet, assets, n, w, e, s, parent, base_material);
        }
        TileContents::Lock => {
            let neighbours = check_neighbours(
                neighbours,
                |t| {
                    matches!(
                        t.contents,
                        TileContents::Canal | TileContents::River | TileContents::Lock
                    ) && tile.z.abs_diff(t.z) < 5
                        || if let TileContents::Aquaduct(h) = t.contents {
                            tile.z == h + t.z
                        } else {
                            false
                        }
                },
                |t, _, _| t.wetness,
            );

            let n = neighbours[0];
            let w = neighbours[1];
            let e = neighbours[2];
            let s = neighbours[3];

            let tmp = Tile {
                contents: TileContents::Canal,
                ..tile.clone()
            };

            spawn_variant(
                &tmp,
                !is_wet,
                assets,
                n,
                w,
                e,
                s,
                parent,
                base_material.clone(),
            );
            spawn_variant(tile, !is_wet, assets, n, w, e, s, parent, base_material);
        }
        TileContents::Aquaduct(h) => {
            let z = tile.z + h;
            let neighbours = check_neighbours(
                neighbours,
                |t| {
                    matches!(
                        t.contents,
                        TileContents::Canal | TileContents::River | TileContents::Lock
                    ) && z == t.z
                        || if let TileContents::Aquaduct(h) = t.contents {
                            z == h + t.z
                        } else {
                            false
                        }
                },
                |t, _, _| t.wetness,
            );

            let n = neighbours[0];
            let w = neighbours[1];
            let e = neighbours[2];
            let s = neighbours[3];

            spawn_variant(tile, !is_wet, assets, n, w, e, s, parent, base_material);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NeighbourMatch<T> {
    Matches(T),
    DoesntMatch,
    NoNeighbour,
}

pub fn check_neighbours<
    F: Fn(&Tile) -> bool,
    R,
    Z: std::fmt::Debug + Clone + Copy,
    S: Fn(&Tile, &R, &Entity) -> Z,
>(
    neighbours: &[Option<(Entity, &Tile, R)>],
    checked: F,
    selector: S,
) -> [NeighbourMatch<(TileContents, usize, Z)>; 4] {
    let mut result = [NeighbourMatch::NoNeighbour; 4];

    #[allow(clippy::needless_range_loop)]
    for i in 0..8 {
        if let Some(Some((e, neighbour, v))) = neighbours.get(i) {
            result[i] = if checked(neighbour) {
                NeighbourMatch::Matches((
                    neighbour.contents,
                    neighbour.z,
                    selector(neighbour, v, e),
                ))
            } else {
                NeighbourMatch::DoesntMatch
            };
        }
    }
    result
}

fn spawn_variant<T: Material>(
    tile: &Tile,
    _is_dry: bool,
    assets: &CanalManiaAssets,
    n: NeighbourMatch<(TileContents, usize, Wetness)>,
    w: NeighbourMatch<(TileContents, usize, Wetness)>,
    e: NeighbourMatch<(TileContents, usize, Wetness)>,
    s: NeighbourMatch<(TileContents, usize, Wetness)>,
    parent: &mut ChildBuilder,
    material: Handle<T>,
) {
    let content_type = tile.contents;
    let height = tile.z;
    let mut num_river_neighbours = 0usize;
    let results = [(n, 90f32), (w, 180.), (s, 270.), (e, 0.)]
        .into_iter()
        .filter_map(|(neighbour, angle)| {
            if let NeighbourMatch::Matches((content, z, _)) = neighbour {
                if content == TileContents::River {
                    num_river_neighbours += 1;
                }
                match (content_type, content) {
                    (TileContents::Canal, TileContents::Aquaduct(u)) => {
                        Some((TileContents::Aquaduct(u), angle, z, false))
                    }
                    (TileContents::River, TileContents::Aquaduct(u)) => {
                        Some((TileContents::Aquaduct(u), angle, z, false))
                    }
                    (TileContents::River, TileContents::Canal) => {
                        Some((TileContents::River, angle, z, true))
                    }
                    (TileContents::River, TileContents::Lock) => {
                        Some((TileContents::River, angle, z, true))
                    }
                    _ => Some((content_type, angle, height, false)),
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let map_edge = [(n, 90f32), (w, 180.), (s, 270.), (e, 0.)]
        .into_iter()
        .filter_map(|(neighbour, angle)| {
            if matches!(neighbour, NeighbourMatch::NoNeighbour) {
                Some(angle)
            } else {
                None
            }
        })
        .next();

    if let Some(angle) = map_edge {
        match content_type {
            TileContents::River => {
                if num_river_neighbours == 1 {
                    parent.spawn(MaterialMeshBundle {
                        mesh: assets.river_line.clone(),
                        material: material.clone(),
                        transform: Transform::from_rotation(Quat::from_rotation_y(
                            angle.to_radians(),
                        )),
                        ..Default::default()
                    });
                }
            }
            TileContents::Road => {
                parent.spawn(MaterialMeshBundle {
                    mesh: assets.road_line.clone(),
                    material: material.clone(),
                    transform: Transform::from_rotation(Quat::from_rotation_y(angle.to_radians())),
                    ..Default::default()
                });
            }
            _ => {}
        }
    }

    match results.len().cmp(&1) {
        std::cmp::Ordering::Less => {
            let position = match content_type {
                TileContents::Aquaduct(u) => Vec3::Y * (u as f32 - 1. + height as f32) / 6.,
                _ => Vec3::ZERO,
            };

            let mesh = content_type.center(assets);

            parent.spawn(MaterialMeshBundle {
                mesh,
                material,
                transform: Transform::from_translation(position),
                ..Default::default()
            });
        }
        std::cmp::Ordering::Equal => {
            for (content, rotation, height, transition) in results.iter() {
                let position = match content {
                    TileContents::Aquaduct(u) => Vec3::Y * (*u as f32 - 1. + *height as f32) / 6.,
                    _ => Vec3::ZERO,
                };

                let mesh = match content {
                    TileContents::River => {
                        if map_edge.is_some() {
                            if !transition {
                                content.line(assets)
                            } else {
                                assets.river_to_canal_line.clone()
                            }
                        } else if !transition {
                            content.end(assets)
                        } else {
                            assets.river_to_canal_end.clone()
                        }
                    }
                    TileContents::Road => {
                        if map_edge.is_some() {
                            content.line(assets)
                        } else {
                            content.end(assets)
                        }
                    }
                    _ => content.end(assets),
                };

                parent.spawn(MaterialMeshBundle {
                    mesh,
                    material: material.clone(),
                    transform: Transform::from_rotation(Quat::from_rotation_y(
                        rotation.to_radians(),
                    ))
                    .with_translation(position),
                    ..Default::default()
                });
            }
        }
        std::cmp::Ordering::Greater => {
            for (content, rotation, height, transition) in results.iter() {
                let position = match content {
                    TileContents::Aquaduct(u) => Vec3::Y * (*u as f32 - 1. + *height as f32) / 6.,
                    _ => Vec3::ZERO,
                };

                let mesh = if !transition {
                    content.line(assets)
                } else {
                    assets.river_to_canal_line.clone()
                };

                parent.spawn(MaterialMeshBundle {
                    mesh,
                    material: material.clone(),
                    transform: Transform::from_rotation(Quat::from_rotation_y(
                        rotation.to_radians(),
                    ))
                    .with_translation(position),
                    ..Default::default()
                });
            }
        }
    }
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
