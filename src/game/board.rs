mod board_runtime_assets;
mod tile;

use bevy::{
    prelude::*,
    render::{
        render_resource::{
            AddressMode, Extent3d, FilterMode, SamplerDescriptor, TextureDimension, TextureFormat,
        },
        texture::{TextureFormatPixelInfo, Volume},
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

use super::{game_state::GameState, level::Level, tile_shader::TileMaterial};

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
        //#[cfg(feature = "dev")]
        // app.add_plugin(bevy_inspector_egui::quick::AssetInspectorPlugin::<
        //     TileMaterial,
        // >::default());
        // .add_plugin(bevy_inspector_egui::quick::ResourceInspectorPlugin::<
        //     BoardRuntimeAssets,
        // >::default());
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
            if level.initial_description.is_none() {
                commands.insert_resource(NextState(GameState::InGame));
            } else {
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
        info!("Updating tile");
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
        info!("Updated at least one entity!");
        if let Ok(board) = boards.get_single() {
            info!("We got a board!");
            let width = board.width;
            let height = board.height;
            let mut content = vec![(0u8, false); width * height];

            for (_, tile, _) in neighbour_tiles.iter() {
                let x = tile.x;
                let y = tile.y;

                let height = tile.z;
                let is_wet = tile.is_wet;

                let i = y * width + x;

                if let Some(content) = content.get_mut(i) {
                    content.0 = height as u8;
                    if is_wet {
                        content.1 = true;
                    }
                }
            }

            let size = Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            };
            let format = TextureFormat::Rg8Unorm;
            let data = content
                .iter()
                .flat_map(|(height, is_wet)| {
                    [
                        *height * TILE_HEIGHT,
                        if *is_wet { u8::MAX } else { u8::MIN },
                    ]
                })
                .collect::<Vec<_>>();

            info!(
                "Size: {size:?} - {}, format : {format:?} - {}, data: {}, content_len: {}\n{data:?}",
                size.volume(),
                format.pixel_size(),
                data.len(),
                content.len()
            );

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
            info!(
                "Set the image to {result:?} from {:?}",
                board_assets.tile_info_map
            );

            info!("Board width {} height {}", &board.width, &board.height);

            let (offset_x, offset_y) = (board.width % 2 == 0, board.height % 2 == 0);

            if let Some(material) = materials.get_mut(&board_assets.tile_base_material) {
                material.settings.world_offset.x = if offset_x { 0.5 } else { 0. };
                material.settings.world_offset.z = if offset_y { 0.5 } else { 0. };
                material.settings.size = Vec4::new(board.width as f32, 0., board.height as f32, 0.);
                material.info_map = result.clone();
            }

            if let Some(material) = materials.get_mut(&board_assets.decoration_material) {
                material.settings.world_offset.x = if offset_x { 0.5 } else { 0. };
                material.settings.world_offset.z = if offset_y { 0.5 } else { 0. };
                material.settings.size = Vec4::new(board.width as f32, 0., board.height as f32, 0.);
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
                tile.z,
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
                tile.z,
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
                tile.z,
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
                tile.z,
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
                tile.z,
            );
        }
    }
}

pub fn check_neighbours<F: Fn(&Tile) -> bool, R>(
    neighbours: &[Option<(Entity, &Tile, R)>],
    checked: F,
) -> [Option<(TileContents, usize)>; 8] {
    let mut result = [None; 8];

    #[allow(clippy::needless_range_loop)]
    for i in 0..8 {
        if let Some(Some((_, neighbour, _))) = neighbours.get(i) {
            result[i] = if checked(neighbour) {
                Some((neighbour.contents, neighbour.z))
            } else {
                None
            };
        }
    }
    result
}

fn spawn_variant<T: Material>(
    content_type: TileContents,
    _is_dry: bool,
    assets: &CanalManiaAssets,
    n: Option<(TileContents, usize)>,
    w: Option<(TileContents, usize)>,
    e: Option<(TileContents, usize)>,
    s: Option<(TileContents, usize)>,
    parent: &mut ChildBuilder,
    material: Handle<T>,
    height: usize,
) {
    let results = [(n, 90f32), (w, 180.), (s, 270.), (e, 0.)]
        .into_iter()
        .filter_map(|(neighbour, angle)| {
            if let Some((content, z)) = neighbour {
                match (content_type, content) {
                    (TileContents::Canal, TileContents::Aquaduct(u)) => {
                        Some((TileContents::Aquaduct(u), angle, z))
                    }
                    _ => Some((content_type, angle, height)),
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

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
            for (content, rotation, height) in results.iter() {
                let position = match content {
                    TileContents::Aquaduct(u) => Vec3::Y * (*u as f32 - 1. + *height as f32) / 6.,
                    _ => Vec3::ZERO,
                };

                let mesh = content.end(assets);

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
            for (content, rotation, height) in results.iter() {
                let position = match content {
                    TileContents::Aquaduct(u) => Vec3::Y * (*u as f32 - 1. + *height as f32) / 6.,
                    _ => Vec3::ZERO,
                };

                let mesh = content.line(assets);

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
