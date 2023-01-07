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

#[derive(Default)]
struct OnlyDry(bool);

fn run_water_simulation(
    mut commands: Commands,
    tiles: Query<(Entity, &Tile, &TileNeighbours)>,
    mut only_dry: Local<OnlyDry>,
) {
    only_dry.0 = !only_dry.0;
    for (entity, tile, neighbours) in tiles.iter() {
        if tile.wetness == Wetness::WaterSource {
            continue;
        }

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
        if tile.contents == TileContents::Canal {
            let z = tile.z;
            let neighbours = check_neighbours(&neighbours, |neighbour| {
                let (nz, diff) = match neighbour.contents {
                    TileContents::Aquaduct(h) => (h + neighbour.z, 1),
                    _ => (neighbour.z, 2),
                };
                neighbour.wetness != Wetness::Dry && z <= nz && z.abs_diff(nz) < diff
            });

            propogate_wetness(neighbours, tile, &mut commands, entity, only_dry.0);
        } else if tile.contents == TileContents::Lock {
            let z = tile.z;
            let neighbours = check_neighbours(&neighbours, |neighbour| {
                let (nz, diff) = match neighbour.contents {
                    TileContents::Aquaduct(h) => (h + neighbour.z, 1),
                    _ => (neighbour.z, 5),
                };
                neighbour.wetness != Wetness::Dry && z <= nz && z.abs_diff(nz) < diff
            });

            propogate_wetness(neighbours, tile, &mut commands, entity, only_dry.0);
        } else if let TileContents::Aquaduct(h) = tile.contents {
            let z = tile.z + h;
            let neighbours = check_neighbours(&neighbours, |neighbour| {
                let nz = match neighbour.contents {
                    TileContents::Aquaduct(h) => h + neighbour.z,
                    _ => neighbour.z,
                };
                neighbour.wetness != Wetness::Dry && z == nz
            });

            propogate_wetness(neighbours, tile, &mut commands, entity, only_dry.0);
        }
    }
}

fn propogate_wetness(
    neighbours: [NeighbourMatch<(TileContents, usize, Wetness)>; 8],
    tile: &Tile,
    commands: &mut Commands,
    entity: Entity,
    only_dry: bool,
) {
    let min_wetness = neighbours
        .iter()
        .enumerate()
        .fold(Wetness::Dry, |val, (id, n)| {
            if id != 1 && id != 3 && id != 4 && id != 6 {
                // Filter out diagonals...
                return val;
            }
            if let NeighbourMatch::Matches((_, _, wetness)) = n {
                match wetness {
                    Wetness::Dry => val,
                    Wetness::WaterSource => *wetness,
                    Wetness::Wet(a) => match val {
                        Wetness::Dry => *wetness,
                        Wetness::WaterSource => val,
                        Wetness::Wet(b) => Wetness::Wet(*a.min(&b)),
                    },
                }
            } else {
                val
            }
        });

    let (should_update, new_wetness) = match (tile.wetness, min_wetness) {
        (Wetness::Dry, Wetness::WaterSource) => (true, Wetness::Wet(1)),
        (Wetness::Dry, Wetness::Wet(a)) => (true, Wetness::Wet(a + 1)),
        (Wetness::Wet(_), Wetness::Dry) => (true, Wetness::Dry),
        (Wetness::Wet(a), Wetness::WaterSource) => {
            if a != 1 {
                (true, Wetness::Wet(1))
            } else {
                (false, Wetness::Wet(a))
            }
        }
        (Wetness::Wet(a), Wetness::Wet(b)) => {
            if a != b + 1 {
                (true, Wetness::Dry)
            } else {
                (false, Wetness::Dry)
            }
        }
        _ => (false, Wetness::Dry),
    };

    if only_dry && new_wetness != Wetness::Dry {
        return;
    }

    if should_update {
        info!(
            "Updating tile {}, {} wetness from {:?} to {new_wetness:?} {min_wetness:?}",
            tile.x, tile.y, tile.wetness
        );
        let mut tile = tile.clone();
        tile.wetness = new_wetness;
        commands.entity(entity).insert(tile);
    }
}

fn check_goals_for_sucess(tiles: Query<&Tile>, mut commands: Commands) {
    let mut found_goal = false;
    for tile in tiles.iter() {
        if tile.is_goal {
            found_goal = true;
            if matches!(tile.wetness, Wetness::Dry) {
                return;
            }
        }
    }
    if found_goal {
        commands.insert_resource(NextState(GameState::Complete));
    }
}
