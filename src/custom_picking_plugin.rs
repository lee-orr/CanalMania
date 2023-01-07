use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_mod_picking::{
    mesh_events_system, mesh_focus, pause_for_picking_blockers, CustomHighlightPlugin,
    DefaultHighlighting, PausedForBlockers, PickingEvent, PickingPlugin, PickingPluginsState,
    PickingSystem,
};

pub struct CustomPickingPlugin;

fn simple_criteria(flag: bool) -> ShouldRun {
    if flag {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

impl Plugin for CustomPickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PickingPlugin)
            .add_plugin(CustomHighlightPlugin::<StandardMaterial> {
                highlighting_default: |mut assets| DefaultHighlighting {
                    hovered: assets.add(Color::rgb(0.35, 0.35, 0.35).into()),
                    pressed: assets.add(Color::rgb(0.35, 0.75, 0.35).into()),
                    selected: assets.add(Color::rgb(0.35, 0.35, 0.75).into()),
                },
            })
            .add_event::<PickingEvent>()
            .init_resource::<PausedForBlockers>()
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::new()
                    .with_run_criteria(|state: Res<PickingPluginsState>| {
                        simple_criteria(state.enable_interacting)
                    })
                    .with_system(
                        pause_for_picking_blockers
                            .label(PickingSystem::PauseForBlockers)
                            .after(PickingSystem::UpdateIntersections),
                    )
                    .with_system(
                        mesh_focus
                            .label(PickingSystem::Focus)
                            .after(PickingSystem::PauseForBlockers),
                    )
                    .with_system(
                        mesh_events_system
                            .label(PickingSystem::Events)
                            .after(PickingSystem::Selection),
                    ),
            );
    }
}
