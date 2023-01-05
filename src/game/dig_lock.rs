use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use super::{
    board::*,
    game_state::{GameActionMode, GameActions, GameResources},
};
pub struct DigLockPlugin;

impl Plugin for DigLockPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(trigger_dig_lock.run_in_state(GameActionMode::ConstructLock))
            .add_system(dig_lock.run_in_state(GameActionMode::ConstructLock));
    }
}

fn trigger_dig_lock(
    mut event_writer: EventWriter<GameActions>,
    mut event_reader: EventReader<TileEvent>,
) {
    for event in event_reader.iter() {
        if let TileEvent::Clicked(tile, _) = event {
            event_writer.send(GameActions::ConstructLock(tile.clone()));
        }
    }
}

fn dig_lock(
    mut event_reader: EventReader<GameActions>,
    mut tiles: Query<&mut Tile>,
    board: Query<&Board>,
    mut resources: ResMut<GameResources>,
) {
    if let Ok(board) = board.get_single() {
        for event in event_reader.iter() {
            if let GameActions::ConstructLock(tile) = event {
                if !matches!(tile.contents, TileContents::Lock) {
                    let my_position = (tile.x, tile.y);
                    if let Some(entity) = board.children.get(&my_position) {
                        if let Ok(mut tile) = tiles.get_mut(*entity) {
                            resources.cost_so_far += tile.get_lock_cost();
                            tile.contents = TileContents::Lock;
                        }
                    }
                }
            }
        }
    }
}
