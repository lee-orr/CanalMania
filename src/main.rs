mod app_state;
mod assets;
mod menu;
mod ui;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_vfx_bag::{image::mask::*, BevyVfxBagPlugin, PostProcessingInput};
use iyes_loopless::prelude::*;

use app_state::*;
use menu::MainMenuPlugin;
use ui::GameUiPlugin;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    let mut app = App::new();

    // #[cfg(feature = "dev")]
    // app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            fit_canvas_to_parent: true,
            ..Default::default()
        },
        ..Default::default()
    }))
    .add_plugin(ShapePlugin)
    .add_plugin(BevyVfxBagPlugin)
    .insert_resource(Mask::new_vignette())
    .add_plugin(MaskPlugin)
    .insert_resource(ClearColor(Color::hex("e7d2a4").unwrap_or_default()))
    .add_loopless_state(AppLoadingState::Loading)
    .add_loopless_state(AppState::Loading)
    .add_loading_state(
        LoadingState::new(AppLoadingState::Loading)
            .continue_to_state(AppLoadingState::Loaded)
            .with_dynamic_collections::<StandardDynamicAssetCollection>(vec![
                "dynamic_assets.assets",
            ])
            .with_collection::<assets::CanalManiaAssets>(),
    );

    app.add_plugin(GameUiPlugin)
        .add_plugin(MainMenuPlugin)
        .add_startup_system(setup)
        .add_enter_system(AppLoadingState::Loaded, on_loaded);

    app.run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PostProcessingInput);
}

fn on_loaded(mut commands: Commands) {
    println!("Moving to main menu state");
    commands.insert_resource(NextState(AppState::MainMenu));
}
