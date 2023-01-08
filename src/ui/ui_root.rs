use bevy::ui::FocusPolicy;
use bevy::{ecs::schedule::StateData, prelude::*};
use iyes_loopless::prelude::AppLooplessStateExt;

use std::fmt::Debug;
use std::hash::Hash;

use super::UiComponentSpawner;

#[derive(Component, Debug, Default, Clone)]
pub struct UiRoot {
    pub ui_root_type: UiRootType,
    pub background: Background,
    pub padding: f32,
    pub state_hash: Option<String>,
}

#[derive(Debug, Clone)]
pub enum UiRootType {
    Fill,
    Positioned(UiRect),
    World { track: Entity, camera: Entity },
}

#[derive(Debug, Clone)]
pub enum Background {
    Transparent,
    Opaque,
}

#[derive(Component)]
pub struct WorldUiRoot;

impl Default for UiRootType {
    fn default() -> Self {
        Self::Fill
    }
}

impl Default for Background {
    fn default() -> Self {
        Background::Transparent
    }
}

impl UiRoot {
    pub fn new() -> Self {
        Self {
            padding: 10.,
            ..Default::default()
        }
    }

    pub fn world_position(&mut self, track: Entity, camera: Entity) -> &mut Self {
        self.ui_root_type = UiRootType::World { track, camera };
        self
    }

    pub fn position(&mut self, left: Val, right: Val, top: Val, bottom: Val) -> &mut Self {
        self.ui_root_type = UiRootType::Positioned(UiRect::new(left, right, top, bottom));
        self
    }

    pub fn padding(&mut self, padding: f32) -> &mut Self {
        self.padding = padding;
        self
    }

    pub fn for_state<T: Debug>(&mut self, state: T) -> &mut Self {
        self.state_hash = Some(format!("{state:?}"));
        self
    }
}

pub trait UiRootSpawner {
    fn position(self, left: Val, right: Val, top: Val, bottom: Val) -> Self;

    fn padding(self, padding: f32) -> Self;

    fn world_position(self, track: Entity, camera: Entity) -> Self;

    fn for_state<T: Debug + Clone>(self, state: T) -> Self;
}

impl<T: UiComponentSpawner<UiRoot>> UiRootSpawner for T {
    fn position(self, left: Val, right: Val, top: Val, bottom: Val) -> Self {
        self.update_value(|v| v.position(left, right, top, bottom))
    }

    fn padding(self, padding: f32) -> Self {
        self.update_value(|v| v.padding(padding))
    }

    fn for_state<R: Debug + Clone>(self, state: R) -> Self {
        self.update_value(|v| v.for_state(state.clone()))
    }

    fn world_position(self, track: Entity, camera: Entity) -> Self {
        self.update_value(|v| v.world_position(track, camera))
    }
}

fn get_world_ui_position(
    entity: Entity,
    camera: Entity,
    transformables: &Query<&GlobalTransform>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
) -> UiRect {
    if let Ok(transform) = transformables.get(entity) {
        if let Ok((camera, camera_transform)) = cameras.get(camera) {
            if let Some(ndc) = camera.world_to_ndc(camera_transform, transform.translation()) {
                let ndc = (ndc + 1.) * 50.;
                return UiRect::new(
                    Val::Percent(ndc.x),
                    Val::Auto,
                    Val::Auto,
                    Val::Percent(ndc.y),
                );
            }
        }
    }
    UiRect::default()
}

pub fn spawn_ui_root(
    mut commands: Commands,
    roots: Query<(Entity, &UiRoot), Changed<UiRoot>>,
    transformables: Query<&GlobalTransform>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    for (entity, root) in roots.iter() {
        let mut commands = commands.entity(entity);
        commands.insert((NodeBundle {
            style: Style {
                size: match root.ui_root_type {
                    UiRootType::Fill => Size::new(Val::Percent(100.), Val::Percent(100.)),
                    UiRootType::Positioned(_) => Size::AUTO,
                    UiRootType::World {
                        track: _,
                        camera: _,
                    } => Size::UNDEFINED,
                },
                max_size: match root.ui_root_type {
                    UiRootType::Fill => Size::new(Val::Percent(100.), Val::Percent(100.)),
                    UiRootType::Positioned(_) => Size::AUTO,
                    UiRootType::World {
                        track: _,
                        camera: _,
                    } => Size::AUTO,
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(root.padding)),
                position_type: match root.ui_root_type {
                    UiRootType::Fill => PositionType::Relative,
                    _ => PositionType::Absolute,
                },
                position: match root.ui_root_type {
                    UiRootType::Fill => UiRect::default(),
                    UiRootType::Positioned(rect) => rect,
                    UiRootType::World { track, camera } => {
                        get_world_ui_position(track, camera, &transformables, &cameras)
                    }
                },
                ..Default::default()
            },
            background_color: match root.background {
                Background::Transparent => Color::rgba(0., 0., 0., 0.).into(),
                Background::Opaque => Color::rgb_u8(177, 162, 124).into(),
            },
            focus_policy: match root.ui_root_type {
                UiRootType::World {
                    track: _,
                    camera: _,
                } => bevy::ui::FocusPolicy::Pass,
                _ => FocusPolicy::default(),
            },
            z_index: match root.ui_root_type {
                UiRootType::World {
                    track: _,
                    camera: _,
                } => ZIndex::Global(-1),
                _ => ZIndex::Global(1),
            },
            ..Default::default()
        },));
        if let UiRootType::World {
            track: _,
            camera: _,
        } = root.ui_root_type
        {
            commands.insert(WorldUiRoot);
        }
    }
}

pub fn update_world_ui(
    mut roots: Query<(Entity, &UiRoot, &mut Style), With<WorldUiRoot>>,
    transformables: Query<&GlobalTransform>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    for (_entity, root, mut style) in roots.iter_mut() {
        if let UiRootType::World { track, camera } = root.ui_root_type {
            let position = get_world_ui_position(track, camera, &transformables, &cameras);
            style.position = position;
        }
    }
}

pub struct ClearUi;

fn clear_ui_on_exit(mut commands: Commands, query: Query<(Entity, &UiRoot)>, state_hash: String) {
    for (entity, root) in &query {
        if let Some(hash) = &root.state_hash {
            if hash != &state_hash {
                continue;
            }
        }
        commands.entity(entity).despawn_recursive();
    }
}

pub fn clear_ui_on_event(
    event: EventReader<ClearUi>,
    mut commands: Commands,
    query: Query<Entity, With<UiRoot>>,
) {
    if event.is_empty() {
        return;
    }
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn clear_ui_system_set<T: Debug + Clone + Eq + PartialEq + Hash + StateData>(
    app: &mut App,
    t: T,
) -> &mut App {
    let state_hash = format!("{t:?}");
    app.add_exit_system(
        t,
        move |commands: Commands, query: Query<(Entity, &UiRoot)>| {
            let state_hash = state_hash.clone();
            clear_ui_on_exit(commands, query, state_hash)
        },
    );
    app
}
