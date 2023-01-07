use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use super::{
    board::*,
    game_state::{GameActionMode, GameActions, GameResources},
};
pub struct BuildAquaductPlugin;

impl Plugin for BuildAquaductPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LastAquaductHeight(0))
            .add_system(trigger_build_aquaduct.run_in_state(GameActionMode::BuildAquaduct))
            .add_system(build_aquaduct.run_in_state(GameActionMode::BuildAquaduct));
    }
}

#[derive(Resource)]
struct LastAquaductHeight(usize);

fn trigger_build_aquaduct(
    mut event_writer: EventWriter<GameActions>,
    mut event_reader: EventReader<TileEvent>,
    buttons: Res<Input<MouseButton>>,
    mut aquaduct_height: ResMut<LastAquaductHeight>,
) {
    for event in event_reader.iter() {
        match event {
            TileEvent::Clicked(tile, _) => {
                if let TileContents::Aquaduct(h) = tile.contents {
                    event_writer.send(GameActions::BuildAquaduct(tile.clone(), h + 1));
                    aquaduct_height.0 = h + 1 + tile.z;
                } else {
                    event_writer.send(GameActions::BuildAquaduct(tile.clone(), 1));
                    aquaduct_height.0 = 1 + tile.z;
                }
            }
            TileEvent::HoverStarted(tile, _) => {
                if buttons.pressed(MouseButton::Left) && tile.z < aquaduct_height.0 {
                    event_writer.send(GameActions::BuildAquaduct(
                        tile.clone(),
                        aquaduct_height.0 - tile.z,
                    ));
                }
            }
            _ => (),
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
                        tile.is_wet = false;
                        tile.contents = TileContents::Aquaduct(*height);
                    }
                }
            }
        }
    }
}
