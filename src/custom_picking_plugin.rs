use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub struct CustomPickingPlugin;

impl Plugin for CustomPickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins)
            .add_systems(Startup, setup);
        // .init_resource::<PausedForBlockers>()
        // .add_system_set_to_stage(
        //     CoreStage::First,
        //     SystemSet::new()
        //         .with_run_criteria(|state: Res<PickingPluginsState>| {
        //             simple_criteria(state.enable_interacting)
        //         })
        //         .with_system(
        //             pause_for_picking_blockers
        //                 .label(PickingSystem::PauseForBlockers)
        //                 .after(PickingSystem::UpdateIntersections),
        //         )
        //         .with_system(
        //             mesh_focus
        //                 .label(PickingSystem::Focus)
        //                 .after(PickingSystem::PauseForBlockers),
        //         )
        //         .with_system(
        //             mesh_events_system
        //                 .label(PickingSystem::Events)
        //                 .after(PickingSystem::Selection),
        //         ),
        // );
    }
}

fn setup(mut assets: ResMut<Assets<StandardMaterial>>, mut commands: Commands) {
    commands.insert_resource(GlobalHighlight {
        hovered: assets.add(Color::rgb(0.35, 0.35, 0.35).into()),
        pressed: assets.add(Color::rgb(0.35, 0.75, 0.35).into()),
        selected: assets.add(Color::rgb(0.35, 0.35, 0.75).into()),
    });
}
