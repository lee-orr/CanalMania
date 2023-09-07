use bevy::prelude::*;

use super::{
    board::*,
    game_state::{GameActionMode, GameActions, GameResources, GameState},
};
pub struct DemolishPlugin;

impl Plugin for DemolishPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (trigger_demolish, demolish).run_if(
                in_state(GameActionMode::Demolish).and_then(not(in_state(GameState::Description))),
            ),
        );
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
                        if let Some(cost) = tile.get_demolish_cost() {
                            resources.cost_so_far += cost;
                            tile.contents = TileContents::None;
                            tile.wetness = Wetness::Dry;
                        }
                    }
                }
            }
        }
    }
}
