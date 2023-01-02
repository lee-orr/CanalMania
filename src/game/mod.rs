mod board;
mod editor_ui;
mod game_state;
pub mod level;
mod turn_start_ui;

use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use self::{
    board::BoardPlugin, editor_ui::EditorUiPlugin, game_state::GameState,
    turn_start_ui::TurnStartUiPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::Setup)
            .add_plugin(BoardPlugin)
            .add_plugin(TurnStartUiPlugin)
            .add_plugin(EditorUiPlugin);
    }
}
