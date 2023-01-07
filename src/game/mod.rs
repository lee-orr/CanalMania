mod board;
mod game_state;
mod in_game_ui;
pub mod level;

mod build_aquaduct;
mod demolish;
mod dig_canal;
mod dig_lock;
#[cfg(not(target_family = "wasm"))]
mod editor_ui;
mod game_complete_ui;
mod initial_description;
mod simulation;
mod tile_hover_ui;
pub mod tile_shader;

use bevy::prelude::*;
use iyes_loopless::{prelude::AppLooplessStateExt, state::NextState};

use crate::app_state::AppState;

use self::{
    board::BoardPlugin,
    build_aquaduct::BuildAquaductPlugin,
    demolish::DemolishPlugin,
    dig_canal::DigCanalPlugin,
    dig_lock::DigLockPlugin,
    game_complete_ui::GameCompleteUiPlugin,
    game_state::{GameActionMode, GameActions, GameResources, GameState},
    in_game_ui::InGameUiPlugin,
    initial_description::InitialDescriptionUiPlugin,
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
            .add_enter_system(GameState::Complete, disable_actions)
            .add_plugin(BoardPlugin)
            .add_plugin(TileHoverUi)
            .add_plugin(InGameUiPlugin)
            .add_plugin(InitialDescriptionUiPlugin)
            .add_plugin(GameCompleteUiPlugin)
            .add_plugin(DigCanalPlugin)
            .add_plugin(DigLockPlugin)
            .add_plugin(DemolishPlugin)
            .add_plugin(BuildAquaductPlugin)
            .add_plugin(SimulationPlugin)
            .add_plugin(MaterialPlugin::<TileMaterial>::default());
        #[cfg(not(target_family = "wasm"))]
        app.add_plugin(self::editor_ui::EditorUiPlugin);

        #[cfg(feature = "dev")]
        // app.add_plugin(bevy_inspector_egui::quick::AssetInspectorPlugin::<
        //     TileMaterial,
        // >::default());
        // app.add_plugin(bevy_inspector_egui::quick::ResourceInspectorPlugin::<
        //     BoardRuntimeAssets,
        // >::default());
        app.add_plugin(bevy_inspector_egui::quick::ResourceInspectorPlugin::<
            level::LevelTools,
        >::default());
    }
}

fn prepare_for_setup(mut commands: Commands) {
    commands.insert_resource(GameResources::default());
    commands.insert_resource(NextState(GameState::Setup));
    commands.insert_resource(NextState(GameActionMode::None));
}

fn disable_actions(mut commands: Commands) {
    commands.insert_resource(NextState(GameActionMode::None));
}
