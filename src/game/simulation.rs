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
        if tile.contents == TileContents::Canal && !tile.is_wet {
            let neighbours = check_neighbours(&neighbours, |neighbour| {
                neighbour.is_wet && tile.z <= neighbour.z && tile.z.abs_diff(neighbour.z) < 2
            });

            let n = neighbours[1];
            let w = neighbours[3];
            let e = neighbours[4];
            let s = neighbours[6];

            if n || w || s || e {
                let mut tile = tile.clone();
                tile.is_wet = true;
                commands.entity(entity).insert(tile);
            }
        } else if tile.contents == TileContents::Lock && !tile.is_wet {
            let neighbours = check_neighbours(&neighbours, |neighbour| {
                neighbour.is_wet && tile.z <= neighbour.z && tile.z.abs_diff(neighbour.z) < 5
            });

            let n = neighbours[1];
            let w = neighbours[3];
            let e = neighbours[4];
            let s = neighbours[6];

            if n || w || s || e {
                let mut tile = tile.clone();
                tile.is_wet = true;
                commands.entity(entity).insert(tile);
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
