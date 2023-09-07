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
            .add_state::<GameState>()
            .add_state::<GameActionMode>()
            .add_systems(OnEnter(AppState::InGame), prepare_for_setup)
            .add_systems(OnExit(AppState::InGame), prepare_for_setup)
            .add_systems(OnEnter(GameState::Complete), disable_actions)
            .add_plugins(BoardPlugin)
            .add_plugins(TileHoverUi)
            .add_plugins(InGameUiPlugin)
            .add_plugins(InitialDescriptionUiPlugin)
            .add_plugins(GameCompleteUiPlugin)
            .add_plugins(DigCanalPlugin)
            .add_plugins(DigLockPlugin)
            .add_plugins(DemolishPlugin)
            .add_plugins(BuildAquaductPlugin)
            .add_plugins(SimulationPlugin)
            .add_plugins(MaterialPlugin::<TileMaterial>::default());
        #[cfg(not(target_family = "wasm"))]
        app.add_plugins(self::editor_ui::EditorUiPlugin);

        #[cfg(feature = "dev")]
        {
            // app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin);
            // app.add_plugins(bevy_inspector_egui::quick::AssetInspectorPlugin::<
            //     TileMaterial,
            // >::default());
            // app.add_plugins(bevy_inspector_egui::quick::ResourceInspectorPlugin::<
            //     BoardRuntimeAssets,
            // >::default());
            // app.add_plugins(bevy_inspector_egui::quick::ResourceInspectorPlugin::<
            //     in_game_ui::SidebarText,
            // >::default());
        }
    }
}

fn prepare_for_setup(mut commands: Commands) {
    commands.insert_resource(GameResources::default());
    commands.insert_resource(NextState(Some(GameState::Setup)));
    commands.insert_resource(NextState(Some(GameActionMode::None)));
}

fn disable_actions(mut commands: Commands) {
    commands.insert_resource(NextState(Some(GameActionMode::None)));
}
