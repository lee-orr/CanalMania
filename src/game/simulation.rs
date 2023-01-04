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
    tiles: Query<(Entity, &Tile)>,
    board: Query<&Board>,
) {
    if let Ok(board) = board.get_single() {
        for (entity, tile) in tiles.iter() {
            if tile.tile_type == TileType::CanalDry {
                let neighbours = board.neighbours(tile.x, tile.y);
                let has_water_neighbour = neighbours.iter().find(|neighbour| match neighbour {
                    Some(entity) => {
                        if let Ok((_, neighbour)) = tiles.get(*entity) {
                            if neighbour.tile_type == TileType::CanalWet
                                && tile.z <= neighbour.z
                                && tile.z.abs_diff(neighbour.z) < 2
                            {
                                return true;
                            }
                        }
                        false
                    }
                    None => false,
                });
                if has_water_neighbour.is_some() {
                    let mut tile = tile.clone();
                    tile.tile_type = TileType::CanalWet;
                    commands.entity(entity).insert(tile);
                }
            } else if tile.tile_type == TileType::LockDry {
                let neighbours = board.neighbours(tile.x, tile.y);
                let has_water_neighbour = neighbours.iter().find(|neighbour| match neighbour {
                    Some(entity) => {
                        if let Ok((_, neighbour)) = tiles.get(*entity) {
                            if neighbour.tile_type == TileType::CanalWet
                                && tile.z <= neighbour.z
                                && tile.z.abs_diff(neighbour.z) < 5
                            {
                                return true;
                            }
                        }
                        false
                    }
                    None => false,
                });
                if has_water_neighbour.is_some() {
                    let mut tile = tile.clone();
                    tile.tile_type = TileType::CanalWet;
                    commands.entity(entity).insert(tile);
                }
            }
        }
    }
}

fn check_goals_for_sucess(tiles: Query<&Tile>, board: Query<&Board>, mut commands: Commands) {
    if let Ok(board) = board.get_single() {
        let mut found_goal = false;
        for tile in tiles.iter() {
            if tile.is_goal {
                found_goal = true;
                let neighbours = board.neighbours(tile.x, tile.y);
                let has_water_neighbour = neighbours.iter().find(|neighbour| match neighbour {
                    Some(entity) => {
                        if let Ok(neighbour) = tiles.get(*entity) {
                            if neighbour.tile_type == TileType::CanalWet
                                && tile.z <= neighbour.z
                                && tile.z.abs_diff(neighbour.z) < 2
                            {
                                return true;
                            }
                        }
                        false
                    }
                    None => false,
                });
                if has_water_neighbour.is_none() {
                    return;
                }
            }
        }
        if found_goal {
            commands.insert_resource(NextState(GameState::Complete));
        }
    }
}
