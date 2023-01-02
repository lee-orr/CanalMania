use bevy::{ecs::schedule::StateData, prelude::*};
use iyes_loopless::prelude::AppLooplessStateExt;
use std::fmt::Debug;
use std::hash::Hash;

use super::ui_id::WithUiId;

#[derive(Component, Debug, Default)]
pub struct UiRoot {
    pub ui_root_type: UiRootType,
    pub background: Background,
    pub padding: f32,
}

#[derive(Debug)]
pub enum UiRootType {
    Fill,
    Positioned(UiRect),
}

#[derive(Debug)]
pub enum Background {
    Transparent,
    Opaque
}

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
impl WithUiId for UiRoot {}

impl UiRoot {
    pub fn new() -> Self {
        Self {
            padding: 10., 
            ..Default::default()
        }
    }

    pub fn opaque(self) -> Self {
        Self {
            background: Background::Opaque,
            ..self
        }
    }

    pub fn position(self, left: Val, right: Val, top: Val, bottom: Val) -> Self {
        Self {
            ui_root_type: UiRootType::Positioned(UiRect::new(left, right, top, bottom)),
            ..self
        }
    }

    pub fn padding(self, padding: f32) -> Self {
        Self {
            padding,
            ..self
        }
    }
}

pub fn spawn_ui_root(mut commands: Commands, roots: Query<(Entity, &UiRoot), Added<UiRoot>>) {
    for (entity, root) in roots.iter() {
        commands.entity(entity).insert((NodeBundle {
            style: Style {
                size: match root.ui_root_type {
                    UiRootType::Fill => Size::new(Val::Percent(100.), Val::Percent(100.)),
                    UiRootType::Positioned(_) => Size::AUTO,
                },
                max_size: match root.ui_root_type {
                    UiRootType::Fill => Size::new(Val::Percent(100.), Val::Percent(100.)),
                    UiRootType::Positioned(_) => Size::AUTO,
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(root.padding)),
                position_type: match root.ui_root_type {
                    UiRootType::Fill => PositionType::Relative,
                    UiRootType::Positioned(_) => PositionType::Absolute,
                },
                position: match root.ui_root_type {
                    UiRootType::Fill => UiRect::default(),
                    UiRootType::Positioned(rect) => rect.clone(),
                },
                ..Default::default()
            },
            background_color: match root.background {
                Background::Transparent => Color::rgba(0., 0., 0., 0.).into(),
                Background::Opaque => Color::rgb_u8(177,162,124).into(),
            },
            ..Default::default()
        },));
    }
}

fn clear_ui(mut commands: Commands, query: Query<Entity, With<UiRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn clear_ui_system_set<T: Debug + Clone + Eq + PartialEq + Hash + StateData>(
    app: &mut App,
    t: T,
) -> &mut App {
    app.add_exit_system(t, clear_ui);
    app
}
