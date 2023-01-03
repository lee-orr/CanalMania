mod board;
mod game_state;
mod in_game_ui;
pub mod level;

#[cfg(not(target_family = "wasm"))]
mod editor_ui;

use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use self::{board::BoardPlugin, game_state::GameState, in_game_ui::InGameUiPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::Setup)
            .add_plugin(BoardPlugin)
            .add_plugin(InGameUiPlugin);
        #[cfg(not(target_family = "wasm"))]
        app.add_plugin(self::editor_ui::EditorUiPlugin);
    }
}
