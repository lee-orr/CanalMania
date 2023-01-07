mod app_state;
mod assets;
mod camera_control;
mod choose_level;
mod credits;
mod custom_picking_plugin;
mod game;
mod menu;
mod ui;

use assets::CanalManiaAssets;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::{json::JsonAssetPlugin, yaml::YamlAssetPlugin};

use bevy_mod_picking::PickingCameraBundle;

use camera_control::CameraControlPlugin;
use choose_level::ChooseLevelPlugin;
use credits::CreditsPlugin;
use custom_picking_plugin::CustomPickingPlugin;
use game::{
    level::{Level, LevelList},
    GamePlugin,
};
use iyes_loopless::prelude::*;

use app_state::*;
use menu::MainMenuPlugin;
use noisy_bevy::NoisyShaderPlugin;
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    *,
};
use ui::GameUiPlugin;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    let mut app = App::new();

    app.insert_resource(Msaa { samples: 4 })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        fit_canvas_to_parent: true,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                }),
        )
        .add_plugin(NoisyShaderPlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin {
            override_input_system: true,
        })
        .add_plugin(JsonAssetPlugin::<Level>::new(&["lvl.json"]))
        .add_plugin(YamlAssetPlugin::<LevelList>::new(&["levels.yml"]));

    app.insert_resource(ClearColor(Color::hex("e7d2a4").unwrap_or_default()))
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
        .add_plugin(ChooseLevelPlugin)
        .add_plugin(CreditsPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(CameraControlPlugin)
        .add_plugin(CustomPickingPlugin)
        .add_startup_system(setup)
        .add_enter_system(AppLoadingState::Loaded, on_loaded);

    app.run();
}

fn setup(mut commands: Commands) {
    let eye = Vec3::new(5., 10., 5.);
    let target = Vec3::default();
    commands
        .spawn(Camera3dBundle::default())
        .insert(OrbitCameraBundle::new(
            OrbitCameraController {
                enabled: false,
                mouse_translate_sensitivity: Vec2::splat(0.5),
                mouse_rotate_sensitivity: Vec2::splat(0.08),
                mouse_wheel_zoom_sensitivity: 1.5,
                ..Default::default()
            },
            eye,
            target,
        ))
        .insert(PickingCameraBundle::default());
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1., 1.2, 0.)),
        ..Default::default()
    });
}

fn on_loaded(
    mut commands: Commands,
    assets: Res<CanalManiaAssets>,
    level_list_asset: Res<Assets<LevelList>>,
) {
    println!("Moving to main menu state");
    commands.insert_resource(NextState(AppState::MainMenu));
    if let Some(list) = level_list_asset.get(&assets.level_list) {
        let list = list.clone();
        commands.insert_resource(list);
    }
}
