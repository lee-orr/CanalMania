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
use bevy::{asset::ChangeWatcher, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::{json::JsonAssetPlugin, yaml::YamlAssetPlugin};

use bevy_mod_picking::prelude::RaycastPickCamera;

use app_state::*;
use camera_control::CameraControlPlugin;
use choose_level::ChooseLevelPlugin;
use credits::CreditsPlugin;
use custom_picking_plugin::CustomPickingPlugin;
use game::{
    level::{Level, LevelList},
    GamePlugin,
};
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

    app.insert_resource(Msaa::Sample4)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(std::time::Duration::from_secs(1)),
                    ..Default::default()
                }),
        )
        .add_plugins(NoisyShaderPlugin)
        .add_plugins(LookTransformPlugin)
        .add_plugins(OrbitCameraPlugin {
            override_input_system: true,
        })
        .add_plugins(JsonAssetPlugin::<Level>::new(&["lvl.json"]))
        .add_plugins(YamlAssetPlugin::<LevelList>::new(&["levels.yml"]));

    app.insert_resource(ClearColor(Color::hex("e7d2a4").unwrap_or_default()))
        .add_state::<AppLoadingState>()
        .add_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppLoadingState::Loading)
                .continue_to_state(AppLoadingState::Loaded)
                .set_standard_dynamic_asset_collection_file_endings(vec!["assets"]),
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppLoadingState::Loading,
            "dynamic_assets.assets",
        )
        .add_collection_to_loading_state::<_, assets::CanalManiaAssets>(AppLoadingState::Loading);

    app.add_plugins(GameUiPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(ChooseLevelPlugin)
        .add_plugins(CreditsPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(CameraControlPlugin)
        .add_plugins(CustomPickingPlugin)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppLoadingState::Loaded), on_loaded);

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
                pixels_per_line: 100.,
                ..Default::default()
            },
            eye,
            target,
            Vec3::Y,
        ))
        .insert(RaycastPickCamera::default());
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1., 1.2, 0.)),
        ..Default::default()
    });
}

fn on_loaded(
    mut commands: Commands,
    _assets: Res<CanalManiaAssets>,
    _level_list_asset: Res<Assets<LevelList>>,
) {
    commands.insert_resource(NextState(Some(AppState::MainMenu)));
}
