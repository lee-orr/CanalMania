mod board;
mod game_state;
mod in_game_ui;
pub mod level;

mod dig_canal;
mod dig_lock;
#[cfg(not(target_family = "wasm"))]
mod editor_ui;
mod game_complete_ui;
mod simulation;
mod tile_hover_ui;
mod tile_shader;

use bevy::prelude::*;
use iyes_loopless::{prelude::AppLooplessStateExt, state::NextState};

use crate::app_state::AppState;

use self::{
    board::BoardPlugin,
    dig_canal::DigCanalPlugin,
    dig_lock::DigLockPlugin,
    game_complete_ui::GameCompleteUiPlugin,
    game_state::{GameActionMode, GameActions, GameResources, GameState},
    in_game_ui::InGameUiPlugin,
    simulation::SimulationPlugin,
    tile_hover_ui::TileHoverUi,
    tile_shader::TileMaterial,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameActions>()
            .init_resource::<GameResources>()
            .add_loopless_state(GameState::Setup)
            .add_loopless_state(GameActionMode::None)
            .add_enter_system(AppState::InGame, prepare_for_setup)
            .add_exit_system(AppState::InGame, prepare_for_setup)
            .add_plugin(BoardPlugin)
            .add_plugin(TileHoverUi)
            .add_plugin(InGameUiPlugin)
            .add_plugin(GameCompleteUiPlugin)
            .add_plugin(DigCanalPlugin)
            .add_plugin(DigLockPlugin)
            .add_plugin(SimulationPlugin)
            .add_plugin(MaterialPlugin::<TileMaterial>::default());
        #[cfg(not(target_family = "wasm"))]
        app.add_plugin(self::editor_ui::EditorUiPlugin);
    }
}

fn prepare_for_setup(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Setup));
}
