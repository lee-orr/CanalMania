use bevy::{prelude::*, utils::HashMap};
use iyes_loopless::{prelude::IntoConditionalSystem, state::NextState};

use super::{
    board::*,
    game_state::{GameActionMode, GameActions, GameState},
    in_game_ui::SidebarText,
    initial_description::CurrentDescription,
    level::{EventAction, Level, LevelEvent, LevelEventType, LevelTools, PendingLevelEvents},
};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelEvent>()
            .init_resource::<PendingLevelEvents>()
            .init_resource::<ActionTracker>()
            .init_resource::<LevelTools>()
            .add_system(setup_level_events.run_in_state(GameState::InGame))
            .add_system(run_water_simulation.run_in_state(GameState::InGame))
            .add_system(propogate_water_sources.run_in_state(GameState::InGame))
            .add_system(track_actions.run_in_state(GameState::InGame))
            .add_system(
                check_goals_for_sucess
                    .run_in_state(GameState::InGame)
                    .label("check_goal"),
            )
            .add_system(
                process_level_event
                    .run_in_state(GameState::InGame)
                    .after("check_gaol"),
            );
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
            let neighbours = check_neighbours(
                &neighbours,
                |neighbour| {
                    let (nz, diff) = match neighbour.contents {
                        TileContents::Aquaduct(h) => (h + neighbour.z, 1),
                        _ => (neighbour.z, 1),
                    };
                    neighbour.wetness != Wetness::Dry && z <= nz && z.abs_diff(nz) < diff
                },
                |t, _, _e| t.wetness,
            );

            propogate_wetness(neighbours, tile, &mut commands, entity, only_dry.0);
        } else if tile.contents == TileContents::Lock {
            let z = tile.z;
            let neighbours = check_neighbours(
                &neighbours,
                |neighbour| {
                    let (nz, diff) = match neighbour.contents {
                        TileContents::Aquaduct(h) => (h + neighbour.z, 1),
                        _ => (neighbour.z, 5),
                    };
                    neighbour.wetness != Wetness::Dry && z <= nz && z.abs_diff(nz) < diff
                },
                |t, _, _e| t.wetness,
            );

            propogate_wetness(neighbours, tile, &mut commands, entity, only_dry.0);
        } else if let TileContents::Aquaduct(h) = tile.contents {
            let z = tile.z + h;
            let neighbours = check_neighbours(
                &neighbours,
                |neighbour| {
                    let nz = match neighbour.contents {
                        TileContents::Aquaduct(h) => h + neighbour.z,
                        _ => neighbour.z,
                    };
                    neighbour.wetness != Wetness::Dry && z == nz
                },
                |t, _, _e| t.wetness,
            );

            propogate_wetness(neighbours, tile, &mut commands, entity, only_dry.0);
        }
    }
}

fn propogate_wetness(
    neighbours: [NeighbourMatch<(TileContents, usize, Wetness)>; 4],
    tile: &Tile,
    commands: &mut Commands,
    entity: Entity,
    only_dry: bool,
) {
    let min_wetness = neighbours.iter().fold(Wetness::Dry, |wetness_val, n| {
        if let NeighbourMatch::Matches((_, _, wetness)) = n {
            match wetness {
                Wetness::Dry => wetness_val,
                Wetness::WaterSource => *wetness,
                Wetness::Wet(a) => match wetness_val {
                    Wetness::Dry => *wetness,
                    Wetness::WaterSource => wetness_val,
                    Wetness::Wet(b) => Wetness::Wet(*a.min(&b)),
                },
            }
        } else {
            wetness_val
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

        let mut cmd = commands.entity(entity);
        cmd.insert(tile);
    }
}

fn propogate_water_sources(
    mut commands: Commands,
    tiles: Query<(Entity, &Tile, (&TileNeighbours, &WetnessSource))>,
) {
    let mut changes = HashMap::new();
    for (entity, tile, (neighbours, source)) in tiles.iter() {
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

        let neighbours = if tile.wetness == Wetness::Dry {
            None
        } else if tile.wetness == Wetness::WaterSource {
            let z = tile.z;
            let neighbours = check_neighbours(
                &neighbours,
                |neighbour| {
                    let (nz, diff) = match neighbour.contents {
                        TileContents::River => (neighbour.z, 15),
                        TileContents::Aquaduct(h) => (h + neighbour.z, 1),
                        _ => (neighbour.z, 1),
                    };
                    neighbour.wetness != Wetness::Dry && z.abs_diff(nz) < diff
                },
                |t, (_, n_source), e| (**n_source, *e, t.x, t.y),
            );
            Some(neighbours)
        } else if tile.contents == TileContents::Canal {
            let z = tile.z;
            let neighbours = check_neighbours(
                &neighbours,
                |neighbour| {
                    let (nz, diff) = match neighbour.contents {
                        TileContents::Aquaduct(h) => (h + neighbour.z, 1),
                        _ => (neighbour.z, 1),
                    };
                    neighbour.wetness != Wetness::Dry && z.abs_diff(nz) < diff
                },
                |t, (_, n_source), e| (**n_source, *e, t.x, t.y),
            );

            Some(neighbours)
        } else if tile.contents == TileContents::Lock {
            let z = tile.z;
            let neighbours = check_neighbours(
                &neighbours,
                |neighbour| {
                    let (nz, diff) = match neighbour.contents {
                        TileContents::Aquaduct(h) => (h + neighbour.z, 1),
                        _ => (neighbour.z, 5),
                    };
                    neighbour.wetness != Wetness::Dry && z.abs_diff(nz) < diff
                },
                |t, (_, n_source), e| (**n_source, *e, t.x, t.y),
            );

            Some(neighbours)
        } else if let TileContents::Aquaduct(h) = tile.contents {
            let z = tile.z + h;
            let neighbours = check_neighbours(
                &neighbours,
                |neighbour| {
                    let nz = match neighbour.contents {
                        TileContents::Aquaduct(h) => h + neighbour.z,
                        _ => neighbour.z,
                    };
                    neighbour.wetness != Wetness::Dry && z == nz
                },
                |t, (_, n_source), e| (**n_source, *e, t.x, t.y),
            );
            Some(neighbours)
        } else {
            None
        };

        if let Some(neighbours) = neighbours {
            let mut min_source = *source;

            for n in neighbours.iter() {
                if let NeighbourMatch::Matches((_, _, (s, e, _x, _y))) = n {
                    let s = if let Some(c) = changes.get(e) { *c } else { *s };
                    min_source = min_source.min(s);
                }
            }

            if min_source != *source {
                info!(
                    "Updating Min Source for {:?},{:?} - from {source:?} to {min_source:?}",
                    tile.x, tile.y
                );
                if let Some(value) = changes.get(&entity) {
                    changes.insert(entity, min_source.min(*value));
                } else {
                    changes.insert(entity, min_source);
                }
            }
            for n in neighbours.iter() {
                if let NeighbourMatch::Matches((_, _, (source, e, x, y))) = n {
                    if *source != min_source {
                        info!("Updating Min Source for neighbour {:?},{:?} - from {source:?} to {min_source:?}", x,y);
                        if let Some(value) = changes.get(e) {
                            changes.insert(*e, min_source.min(*value));
                        } else {
                            changes.insert(*e, min_source);
                        }
                    }
                }
            }
        }
    }

    for (entity, source) in changes.iter() {
        info!("Setting {entity:?} to {source:?}");
        commands.entity(*entity).insert(*source);
    }
}

fn setup_level_events(
    level: Res<Level>,
    mut level_events: ResMut<PendingLevelEvents>,
    mut commands: Commands,
) {
    if !level.is_changed() {
        return;
    }
    level_events.0 = level.events.iter().cloned().collect();
    commands.insert_resource(level.tools.clone());
    commands.insert_resource(SidebarText(level.sidebar_text.clone()));
}

fn check_goals_for_sucess(
    mut tiles: Query<(&mut Tile, &WetnessSource)>,
    mut commands: Commands,
    mut level_events: ResMut<PendingLevelEvents>,
    mut events: EventWriter<LevelEvent>,
) {
    let mut found_goal = false;
    let mut min_wetness_source = WetnessSource::None;
    for (tile, source) in tiles.iter() {
        if tile.is_goal {
            found_goal = true;
            if tile.wetness == Wetness::Dry || *source == WetnessSource::None {
                return;
            }

            info!("Found a goal that can be complete! {min_wetness_source:?} {source:?}");
            if min_wetness_source == WetnessSource::None {
                min_wetness_source = *source;
                info!("We have no wetness yet");
            } else if min_wetness_source != *source {
                return;
            }
        }
    }
    if found_goal {
        for (mut tile, _) in tiles.iter_mut() {
            if tile.is_goal {
                tile.is_goal = false;
            }
        }
        let mut pop = false;
        if let Some(event) = level_events.0.front() {
            if event.0 == LevelEventType::GoalReached {
                pop = true;
            }
        }
        if pop {
            if let Some(event) = level_events.0.pop_front() {
                info!("Goal Reached Event {event:?}");
                events.send(event);
                return;
            }
        }
        commands.insert_resource(NextState(GameState::Complete));
    }
}

#[derive(Resource, Default, Debug)]
pub struct ActionTracker {
    pub canals: usize,
    pub locks: usize,
    pub aquaducts: usize,
    pub demolished: usize,
    pub total: usize,

    pub canals_since_last_event: usize,
    pub locks_since_last_event: usize,
    pub aquaducts_since_last_event: usize,
    pub demolished_since_last_event: usize,
    pub total_since_last_event: usize,
}

fn track_actions(
    mut event_reader: EventReader<GameActions>,
    mut action_tracker: ResMut<ActionTracker>,
    mut level_events: ResMut<PendingLevelEvents>,
    mut events: EventWriter<LevelEvent>,
) {
    for event in event_reader.iter() {
        match event {
            GameActions::DigCanal(_) => {
                action_tracker.canals += 1;
                action_tracker.canals_since_last_event += 1;
            }
            GameActions::ConstructLock(_) => {
                action_tracker.locks += 1;
                action_tracker.locks_since_last_event += 1;
            }
            GameActions::BuildAquaduct(_, _) => {
                action_tracker.aquaducts += 1;
                action_tracker.aquaducts_since_last_event += 1;
            }
            GameActions::Demolish(_) => {
                action_tracker.demolished += 1;
                action_tracker.demolished_since_last_event += 1;
            }
        }
        action_tracker.total += 1;
        action_tracker.total_since_last_event += 1;

        let mut pop = false;
        if let Some(event) = level_events.0.front() {
            pop = match event.0 {
                LevelEventType::AnyActionsComplete(x, since_last_event) => {
                    x < if since_last_event {
                        action_tracker.total_since_last_event
                    } else {
                        action_tracker.total
                    }
                }
                LevelEventType::BuiltNofType(x, content, since_last_event) => {
                    x < if since_last_event {
                        match content {
                            GameActionMode::Demolish => action_tracker.demolished_since_last_event,
                            GameActionMode::DigCanal => action_tracker.canals_since_last_event,
                            GameActionMode::ConstructLock => action_tracker.locks_since_last_event,
                            GameActionMode::BuildAquaduct => {
                                action_tracker.aquaducts_since_last_event
                            }
                            _ => 0,
                        }
                    } else {
                        match content {
                            GameActionMode::Demolish => action_tracker.demolished,
                            GameActionMode::DigCanal => action_tracker.canals,
                            GameActionMode::ConstructLock => action_tracker.locks,
                            GameActionMode::BuildAquaduct => action_tracker.aquaducts,
                            _ => 0,
                        }
                    }
                }
                _ => false,
            }
        }
        if pop {
            if let Some(event) = level_events.0.pop_front() {
                info!("Reached Event {event:?}");
                events.send(event);
                return;
            }
        }
    }
}

fn process_level_event(
    mut events: EventReader<LevelEvent>,
    mut tiles: Query<&mut Tile>,
    mut commands: Commands,
    mut action_tracker: ResMut<ActionTracker>,
    mut tools: ResMut<LevelTools>,
) {
    for event in events.iter() {
        action_tracker.total_since_last_event = 0;
        action_tracker.demolished_since_last_event = 0;
        action_tracker.aquaducts_since_last_event = 0;
        action_tracker.locks_since_last_event = 0;
        action_tracker.canals_since_last_event = 0;

        for action in event.1.iter() {
            match action {
                EventAction::DisplayText {
                    text,
                    title,
                    continue_button,
                } => {
                    commands.insert_resource(CurrentDescription {
                        text: Some(text.clone()),
                        title: title.clone(),
                        continue_button: continue_button.clone(),
                    });
                    commands.insert_resource(NextState(GameState::Description));
                }
                EventAction::SetNewGoal(x, y) => {
                    for mut tile in tiles.iter_mut() {
                        if tile.x == *x && tile.y == *y {
                            tile.is_goal = true;
                            break;
                        }
                    }
                }
                EventAction::AdjustCost(x, y, modifier) => {
                    for mut tile in tiles.iter_mut() {
                        if tile.x == *x && tile.y == *y {
                            tile.cost_modifier = *modifier;
                            break;
                        }
                    }
                }
                EventAction::AdjustContents(x, y, contents) => {
                    for mut tile in tiles.iter_mut() {
                        if tile.x == *x && tile.y == *y {
                            tile.contents = *contents;
                            tile.wetness = match *contents {
                                TileContents::River => Wetness::WaterSource,
                                _ => Wetness::Dry,
                            };
                            break;
                        }
                    }
                }
                EventAction::SetHeight(x, y, h) => {
                    for mut tile in tiles.iter_mut() {
                        if tile.x == *x && tile.y == *y {
                            tile.z = *h;
                            break;
                        }
                    }
                }
                EventAction::AdjustToolAccess(action_mode, action) => {
                    info!("Setting the action mode {action_mode:?} {action:?}");
                    match action_mode {
                        GameActionMode::None => {}
                        GameActionMode::DigCanal => tools.canal = *action,
                        GameActionMode::ConstructLock => tools.lock = *action,
                        GameActionMode::BuildAquaduct => tools.aquaduct = *action,
                        GameActionMode::Demolish => tools.demolish = *action,
                    }
                }
                EventAction::SetSidebar(text) => {
                    commands.insert_resource(SidebarText(text.clone()))
                }
            }
        }
    }
}
