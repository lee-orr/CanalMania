use bevy::prelude::*;
use iyes_loopless::prelude::{ConditionHelpers, IntoConditionalSystem};

use super::{
    board::*,
    game_state::{GameActionMode, GameActions, GameResources, GameState},
};
pub struct DigCanalPlugin;

impl Plugin for DigCanalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            trigger_dig_canal
                .run_in_state(GameActionMode::DigCanal)
                .run_not_in_state(GameState::Description),
        )
        .add_system(
            dig_canal
                .run_in_state(GameActionMode::DigCanal)
                .run_not_in_state(GameState::Description),
        );
    }
}

fn trigger_dig_canal(
    mut event_writer: EventWriter<GameActions>,
    mut event_reader: EventReader<TileEvent>,
    buttons: Res<Input<MouseButton>>,
) {
    for event in event_reader.iter() {
        match event {
            TileEvent::Clicked(tile, _) => {
                event_writer.send(GameActions::DigCanal(tile.clone()));
            }
            TileEvent::HoverStarted(tile, _) => {
                if buttons.pressed(MouseButton::Left) {
                    event_writer.send(GameActions::DigCanal(tile.clone()));
                }
            }
            _ => (),
        }
    }
}

fn dig_canal(
    mut event_reader: EventReader<GameActions>,
    mut tiles: Query<&mut Tile>,
    board: Query<&Board>,
    mut resources: ResMut<GameResources>,
) {
    if let Ok(board) = board.get_single() {
        for event in event_reader.iter() {
            if let GameActions::DigCanal(tile) = event {
                if !matches!(
                    tile.contents,
                    TileContents::Canal | TileContents::Lock | TileContents::River
                ) {
                    let my_position = (tile.x, tile.y);
                    if let Some(entity) = board.children.get(&my_position) {
                        if let Ok(mut tile) = tiles.get_mut(*entity) {
                            if let Some(cost) = tile.get_dig_cost() {
                                resources.cost_so_far += cost;
                                tile.contents = TileContents::Canal;
                                tile.wetness = Wetness::Dry;
                            }
                        }
                    }
                }
            }
        }
    }
}
