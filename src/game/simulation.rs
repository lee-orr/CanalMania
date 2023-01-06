use bevy::prelude::*;
use iyes_loopless::{prelude::IntoConditionalSystem, state::NextState};

use super::{board::*, game_state::GameState};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(run_water_simulation.run_in_state(GameState::InGame))
            .add_system(check_goals_for_sucess.run_in_state(GameState::InGame));
    }
}

fn run_water_simulation(
    mut commands: Commands,
    tiles: Query<(Entity, &Tile, &TileNeighbours)>,
    _board: Query<&Board>,
) {
    for (entity, tile, neighbours) in tiles.iter() {
        let neighbours = neighbours
            .0
            .iter()
            .map(|e| {
                if let Some(e) = e {
                    tiles.get(*e).ok()
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if !tile.is_wet {
            if tile.contents == TileContents::Canal {
                let z = tile.z;
                let neighbours = check_neighbours(&neighbours, |neighbour| {
                    let (nz, diff) = match neighbour.contents {
                        TileContents::Aquaduct(h) => (h + neighbour.z, 1),
                        _ => (neighbour.z, 2),
                    };
                    neighbour.is_wet && z <= nz && z.abs_diff(nz) < diff
                });

                let n = neighbours[1].is_some();
                let w = neighbours[3].is_some();
                let e = neighbours[4].is_some();
                let s = neighbours[6].is_some();

                if n || w || s || e {
                    let mut tile = tile.clone();
                    tile.is_wet = true;
                    commands.entity(entity).insert(tile);
                }
            } else if tile.contents == TileContents::Lock {
                let z = tile.z;
                let neighbours = check_neighbours(&neighbours, |neighbour| {
                    let (nz, diff) = match neighbour.contents {
                        TileContents::Aquaduct(h) => (h + neighbour.z, 1),
                        _ => (neighbour.z, 5),
                    };
                    neighbour.is_wet && z <= nz && z.abs_diff(nz) < diff
                });

                let n = neighbours[1].is_some();
                let w = neighbours[3].is_some();
                let e = neighbours[4].is_some();
                let s = neighbours[6].is_some();

                if n || w || s || e {
                    let mut tile = tile.clone();
                    tile.is_wet = true;
                    commands.entity(entity).insert(tile);
                }
            } else if let TileContents::Aquaduct(h) = tile.contents {
                let z = tile.z + h;
                let neighbours = check_neighbours(&neighbours, |neighbour| {
                    let nz = match neighbour.contents {
                        TileContents::Aquaduct(h) => h + neighbour.z,
                        _ => neighbour.z,
                    };
                    neighbour.is_wet && z == nz
                });

                let n = neighbours[1].is_some();
                let w = neighbours[3].is_some();
                let e = neighbours[4].is_some();
                let s = neighbours[6].is_some();

                if n || w || s || e {
                    let mut tile = tile.clone();
                    tile.is_wet = true;
                    commands.entity(entity).insert(tile);
                }
            }
        }
    }
}

fn check_goals_for_sucess(tiles: Query<&Tile>, mut commands: Commands) {
    let mut found_goal = false;
    for tile in tiles.iter() {
        if tile.is_goal {
            found_goal = true;
            if !tile.is_wet {
                return;
            }
        }
    }
    if found_goal {
        commands.insert_resource(NextState(GameState::Complete));
    }
}
