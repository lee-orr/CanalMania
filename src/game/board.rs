use bevy::{prelude::*, utils::HashMap};
use bevy_mod_picking::{Highlighting, NoDeselect, PickableBundle, PickingEvent};
use iyes_loopless::{prelude::{AppLooplessStateExt, IntoConditionalSystem}, state::NextState};
use bevy_inspector_egui::{prelude::*};

use crate::{
    app_state::{AppLoadingState, AppState},
    assets::CanalManiaAssets,
};

use super::game_state::GameState;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Board>()
            .register_type::<Tile>()
            .register_type::<TileType>()
            .add_event::<TileEvent>()
            .add_enter_system(AppLoadingState::Loaded, setup_board_materials)
            .add_enter_system(AppState::InGame, build_board)
            .add_system(build_tile.run_in_state(AppState::InGame))
            .add_system(process_selection_events.run_in_state(AppState::InGame));
    }
}

#[derive(Resource)]
struct BoardRuntimeAssets {
    pub tile_base_material: Handle<StandardMaterial>,
    pub selector: Handle<Mesh>,
    pub selector_base: Handle<StandardMaterial>,
    pub selector_hovered: Handle<StandardMaterial>,
    pub selector_pressed: Handle<StandardMaterial>,
    pub selector_selected: Handle<StandardMaterial>,
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
struct Board {
    pub children: HashMap<(usize, usize), Entity>
}

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub tile_type: TileType,
}

#[derive(Debug, Clone, Copy, Reflect)]
pub enum TileType {
    Land,
    City,
    Canal,
}

impl Default for TileType {
    fn default() -> Self {
        Self::Land
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
    let selector = meshes.add(shape::Cube::default().into());

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
        selector,
        selector_base,
        selector_hovered,
        selector_pressed,
        selector_selected,
    });
}

fn build_board(
    mut commands: Commands,
) {
    commands.spawn((Board::default(), SpatialBundle::default())).with_children(|parent| {
    for x in 0..10 {
        for y in 0..10 {
            let z = (x + y) % 3;
            parent.spawn((
                Tile {
                    x,
                    y,
                    z,
                    tile_type: TileType::Canal,
                },
                SpatialBundle::default(),
            ));
        }
    }});
    commands.insert_resource(NextState(GameState::TurnStart));
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
            parent.children.insert((tile.x, tile.y), entity.clone());
        }
        let center = Vec3::new(tile.x as f32, (tile.z as f32) / 6. , tile.y as f32);
        let mut entity = commands.entity(entity);
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
                    TileType::Canal => assets.canal_center.clone(),
                },
                material: materials.tile_base_material.clone(),
                ..Default::default()
            });
            for i in 0..4 {
                parent.spawn(PbrBundle {
                    mesh: match tile.tile_type {
                        TileType::Land => assets.tile_corner.clone(),
                        TileType::City => assets.city_corner.clone(),
                        TileType::Canal => assets.canal_corner.clone(),
                    },
                    material: materials.tile_base_material.clone(),
                    transform: Transform::from_rotation(Quat::from_rotation_y(((i as f32) * 90.).to_radians())),
                    ..Default::default()
                });
                parent.spawn(PbrBundle {
                    mesh: match tile.tile_type {
                        TileType::Land => assets.tile_edge.clone(),
                        TileType::City => assets.city_edge.clone(),
                        TileType::Canal => assets.canal_edge.clone(),
                    },
                    material: materials.tile_base_material.clone(),
                    transform: Transform::from_rotation(Quat::from_rotation_y(((i as f32) * 90.).to_radians())),
                    ..Default::default()
                });
            }
        });
    }
}

#[derive(Clone)]
pub enum TileEvent {
    Clicked(Tile, Entity),
}

pub(crate) fn process_selection_events(
    mut events: EventReader<PickingEvent>,
    mut out_events: EventWriter<TileEvent>,
    tiles: Query<&Tile>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
            PickingEvent::Clicked(e) => {
                if let Ok(tile) = tiles.get(e.clone()) {
                    out_events.send(TileEvent::Clicked(tile.clone(), e.clone()));
                }
            }
        }
    }
}
