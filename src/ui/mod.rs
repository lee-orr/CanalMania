mod button;
mod div;
mod text;
mod ui_id;
mod ui_root;

use crate::app_state::AppLoadingState;

use bevy::{ecs::system::EntityCommands, prelude::*};
pub use button::*;
pub use div::*;
use iyes_loopless::prelude::IntoConditionalSystem;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
pub use text::*;
pub use ui_id::*;
pub use ui_root::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonClickEvent>()
            .add_system(spawn_text.run_in_state(AppLoadingState::Loaded))
            .add_system(spawn_ui_root.run_in_state(AppLoadingState::Loaded))
            .add_system(spawn_button.run_in_state(AppLoadingState::Loaded))
            .add_system(spawn_div.run_in_state(AppLoadingState::Loaded))
            .add_system(button_events.run_in_state(AppLoadingState::Loaded));
    }
}

pub struct UiComponent<'w, 's, 'a, T: WithUiId + Component, S: InternalUiSpawner<'w, 's>> {
    pub value: T,
    spawner: &'a mut S,
    phantom: PhantomData<&'w T>,
    phantom_2: PhantomData<&'s T>,
}

pub struct UiComponentWithId<
    'w,
    's,
    'a,
    T: WithUiId + Component,
    S: InternalUiSpawner<'w, 's>,
    R: Debug + PartialEq + Eq + Hash,
> {
    pub value: T,
    pub id: R,
    spawner: &'a mut S,
    phantom: PhantomData<&'w T>,
    phantom_2: PhantomData<&'s T>,
}

impl<'w, 's, 'a, T: WithUiId + Component, S: InternalUiSpawner<'w, 's>>
    UiComponent<'w, 's, 'a, T, S>
{
    pub fn new(value: T, spawner: &'a mut S) -> Self {
        Self {
            value,
            spawner,
            phantom: PhantomData,
            phantom_2: PhantomData,
        }
    }

    pub fn id<R: Debug + PartialEq + Eq + Hash>(
        self,
        id: R,
    ) -> UiComponentWithId<'w, 's, 'a, T, S, R> {
        UiComponentWithId {
            value: self.value,
            id,
            spawner: self.spawner,
            phantom: PhantomData,
            phantom_2: PhantomData,
        }
    }
}

pub trait UiComponentSpawner<T: WithUiId + Component> {
    fn update_value<F: FnMut(T) -> T>(self, updator: F) -> Self;
}

pub trait UiComponentSpawnerActivator<'w, 's, 'a, T, S> {
    fn spawn(self) -> EntityCommands<'w, 's, 'a>;
    fn with_children<F: FnOnce(&mut bevy::prelude::ChildBuilder<'_, '_, '_>)>(
        self,
        f: F,
    ) -> EntityCommands<'w, 's, 'a>
    where
        Self: Sized,
    {
        let mut commands = self.spawn();
        commands.with_children(move |builder| f(builder));
        commands
    }
}

pub trait InternalUiSpawner<'w, 's> {
    fn spawn_ui_component<'a, T: Component>(&'a mut self, value: T) -> EntityCommands<'w, 's, 'a>;
    fn spawn_ui_component_with_id<'a, T: Component, R: Debug + PartialEq + Eq + Hash>(
        &'a mut self,
        value: T,
        id: R,
    ) -> EntityCommands<'w, 's, 'a>
    where
        (T, ui_id::UiId<R>): Bundle;

    fn ui_root<'a>(&'a mut self) -> UiComponent<'w, 's, 'a, UiRoot, Self>
    where
        Self: Sized,
    {
        UiComponent::new(UiRoot::new(), self)
    }
    fn div<'a>(&'a mut self) -> UiComponent<'w, 's, 'a, Div, Self>
    where
        Self: Sized,
    {
        UiComponent::new(Div::new(), self)
    }
    fn text<'a, T: Into<String>>(&'a mut self, text: T) -> UiComponent<'w, 's, 'a, GameText, Self>
    where
        Self: Sized,
    {
        UiComponent::new(GameText::new(text), self)
    }
    fn button<'a, R: Into<String>, T: Into<String>>(
        &'a mut self,
        name: R,
        text: T,
    ) -> UiComponent<'w, 's, 'a, GameButton, Self>
    where
        Self: Sized,
    {
        UiComponent::new(GameButton::new(name, text), self)
    }
}

impl<'w, 's, 'a, T: WithUiId + Component, S: InternalUiSpawner<'w, 's>> UiComponentSpawner<T>
    for UiComponent<'w, 's, 'a, T, S>
{
    fn update_value<F: FnMut(T) -> T>(mut self, mut updator: F) -> Self {
        self.value = updator(self.value);
        self
    }
}

impl<'w, 's, 'a, T: WithUiId + Component, S: InternalUiSpawner<'w, 's>>
    UiComponentSpawnerActivator<'w, 's, 'a, T, S> for UiComponent<'w, 's, 'a, T, S>
{
    fn spawn(self) -> EntityCommands<'w, 's, 'a> {
        self.spawner.spawn_ui_component(self.value)
    }
}

impl<
        'w,
        's,
        'a,
        T: WithUiId + Component,
        S: InternalUiSpawner<'w, 's>,
        R: Debug + PartialEq + Eq + Hash,
    > UiComponentSpawner<T> for UiComponentWithId<'w, 's, 'a, T, S, R>
{
    fn update_value<F: FnMut(T) -> T>(mut self, mut updator: F) -> Self {
        self.value = updator(self.value);
        self
    }
}

impl<
        'w,
        's,
        'a,
        T: WithUiId + Component,
        S: InternalUiSpawner<'w, 's>,
        R: Debug + PartialEq + Eq + Hash,
    > UiComponentSpawnerActivator<'w, 's, 'a, T, S> for UiComponentWithId<'w, 's, 'a, T, S, R>
where
    (T, ui_id::UiId<R>): Bundle,
{
    fn spawn(self) -> EntityCommands<'w, 's, 'a> {
        self.spawner.spawn_ui_component_with_id(self.value, self.id)
    }
}

impl<'w, 's> InternalUiSpawner<'w, 's> for Commands<'w, 's> {
    fn spawn_ui_component<'a, T: Component>(&'a mut self, value: T) -> EntityCommands<'w, 's, 'a> {
        self.spawn(value)
    }

    fn spawn_ui_component_with_id<'a, T: Component, R: Debug + PartialEq + Eq + Hash>(
        &'a mut self,
        value: T,
        id: R,
    ) -> EntityCommands<'w, 's, 'a>
    where
        (T, ui_id::UiId<R>): Bundle,
    {
        let component = value;
        let id = UiId::new(id);
        self.spawn((component, id))
    }
}

impl<'w, 's> InternalUiSpawner<'w, 's> for ChildBuilder<'w, 's, '_> {
    fn spawn_ui_component<'a, T: Component>(&'a mut self, value: T) -> EntityCommands<'w, 's, 'a> {
        self.spawn(value)
    }

    fn spawn_ui_component_with_id<'a, T: Component, R: Debug + PartialEq + Eq + Hash>(
        &'a mut self,
        value: T,
        id: R,
    ) -> EntityCommands<'w, 's, 'a>
    where
        (T, ui_id::UiId<R>): Bundle,
    {
        let component = value;
        let id = UiId::new(id);
        self.spawn((component, id))
    }
}
