use bevy::{ecs::schedule::StateData, prelude::*};
use iyes_loopless::prelude::AppLooplessStateExt;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Component, Debug)]
pub struct UiRoot;

pub fn spawn_ui_root(mut commands: Commands, roots: Query<Entity, Added<UiRoot>>) {
    for entity in roots.iter() {
        commands.entity(entity).insert((NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                max_size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.)),
                ..Default::default()
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
