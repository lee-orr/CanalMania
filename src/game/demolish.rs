use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use super::{
    board::*,
    game_state::{GameActionMode, GameActions, GameResources},
};
pub struct DemolishPlugin;

impl Plugin for DemolishPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(trigger_demolish.run_in_state(GameActionMode::Demolish))
            .add_system(demolish.run_in_state(GameActionMode::Demolish));
    }
}

fn trigger_demolish(
    mut event_writer: EventWriter<GameActions>,
    mut event_reader: EventReader<TileEvent>,
    buttons: Res<Input<MouseButton>>,
) {
    for event in event_reader.iter() {
        match event {
            TileEvent::Clicked(tile, _) => {
                event_writer.send(GameActions::Demolish(tile.clone()));
            }
            TileEvent::HoverStarted(tile, _) => {
                if buttons.pressed(MouseButton::Left) {
                    event_writer.send(GameActions::Demolish(tile.clone()));
                }
            }
            _ => (),
        }
    }
}

fn demolish(
    mut event_reader: EventReader<GameActions>,
    mut tiles: Query<&mut Tile>,
    board: Query<&Board>,
    mut resources: ResMut<GameResources>,
) {
    if let Ok(board) = board.get_single() {
        for event in event_reader.iter() {
            if let GameActions::Demolish(tile) = event {
                let my_position = (tile.x, tile.y);
                if let Some(entity) = board.children.get(&my_position) {
                    if let Ok(mut tile) = tiles.get_mut(*entity) {
                        resources.cost_so_far += tile.get_demolish_cost();
                        tile.contents = TileContents::None;
                        tile.is_wet = false;
                    }
                }
            }
        }
    }
}
