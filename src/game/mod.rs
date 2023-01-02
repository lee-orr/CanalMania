mod board;

use bevy::prelude::*;

use self::board::BoardPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BoardPlugin);
    }
}
