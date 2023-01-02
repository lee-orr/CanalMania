mod board;
mod game_state;
mod turn_start_ui;
mod editor_ui;

use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use self::{board::BoardPlugin, game_state::GameState, turn_start_ui::TurnStartUiPlugin, editor_ui::EditorUiPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loopless_state(GameState::Setup)
            .add_plugin(BoardPlugin)
            .add_plugin(TurnStartUiPlugin)
            .add_plugin(EditorUiPlugin);
    }
}
