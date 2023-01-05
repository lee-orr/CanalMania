use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use super::{
    board::*,
    game_state::{GameActionMode, GameActions, GameResources},
};
pub struct BuildAquaductPlugin;

impl Plugin for BuildAquaductPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(trigger_build_aquaduct.run_in_state(GameActionMode::BuildAquaduct))
            .add_system(build_aquaduct.run_in_state(GameActionMode::BuildAquaduct));
    }
}

fn trigger_build_aquaduct(
    mut event_writer: EventWriter<GameActions>,
    mut event_reader: EventReader<TileEvent>,
) {
    for event in event_reader.iter() {
        if let TileEvent::Clicked(tile, _) = event {
            if let TileContents::Aquaduct(h) = tile.contents {
                event_writer.send(GameActions::BuildAquaduct(tile.clone(), h + 1));
            } else {
                event_writer.send(GameActions::BuildAquaduct(tile.clone(), 1));
            }
        }
    }
}

fn build_aquaduct(
    mut event_reader: EventReader<GameActions>,
    mut tiles: Query<&mut Tile>,
    board: Query<&Board>,
    mut resources: ResMut<GameResources>,
) {
    if let Ok(board) = board.get_single() {
        for event in event_reader.iter() {
            if let GameActions::BuildAquaduct(tile, height) = event {
                let my_position = (tile.x, tile.y);
                if let Some(entity) = board.children.get(&my_position) {
                    if let Ok(mut tile) = tiles.get_mut(*entity) {
                        resources.cost_so_far += tile.get_aquaduct_cost();
                        tile.contents = TileContents::Aquaduct(*height);
                    }
                }
            }
        }
    }
}
