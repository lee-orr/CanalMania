pub mod button;
pub mod div;
pub mod icon;
pub mod text;
pub mod ui_id;
pub mod ui_root;

use crate::app_state::AppLoadingState;

use bevy::{ecs::system::EntityCommands, prelude::*};
use iyes_loopless::prelude::IntoConditionalSystem;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

pub use button::*;
pub use div::*;
pub use icon::*;
pub use text::*;
pub use ui_id::*;
pub use ui_root::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonClickEvent>()
            .add_event::<ClearUi>()
            .add_system(spawn_text.run_in_state(AppLoadingState::Loaded))
            .add_system(spawn_ui_root.run_in_state(AppLoadingState::Loaded))
            .add_system(update_world_ui.run_in_state(AppLoadingState::Loaded))
            .add_system(spawn_button.run_in_state(AppLoadingState::Loaded))
            .add_system(spawn_div.run_in_state(AppLoadingState::Loaded))
            .add_system(button_events.run_in_state(AppLoadingState::Loaded))
            .add_system(spawn_icon.run_in_state(AppLoadingState::Loaded))
            .add_system(clear_ui_on_event);
    }
}

pub struct UiComponent<'w, 's, 'a, T: Component + Clone, S: InternalUiSpawner<'w, 's>> {
    pub value: T,
    #[allow(clippy::type_complexity)]
    pub id_function: Box<dyn Fn(&'a mut S, &T) -> EntityCommands<'w, 's, 'a>>,
    pub should_have_id: bool,
    spawner: Option<&'a mut S>,
    phantom: PhantomData<&'w T>,
    phantom_2: PhantomData<&'s T>,
}

impl<'w, 's, 'a, T: Component + Clone + Debug, S: InternalUiSpawner<'w, 's>>
    UiComponent<'w, 's, 'a, T, S>
{
    pub fn new(value: T, spawner: &'a mut S) -> Self {
        Self {
            value,
            id_function: Box::new(|spawner, value| spawner.spawn_ui_component(value.clone())),
            should_have_id: false,
            spawner: Some(spawner),
            phantom: PhantomData,
            phantom_2: PhantomData,
        }
    }

    pub fn id<R: Debug + PartialEq + Eq + Hash + Clone + Send>(mut self, id: R) -> Self
    where
        (T, ui_id::UiId<R>): Bundle,
    {
        let id = id;
        self.should_have_id = true;
        self.id_function = Box::new(move |spawner, value| {
            spawner.spawn_ui_component_with_id(value.clone(), id.clone())
        });
        self
    }
}

pub trait UiComponentSpawner<T: Component + Clone> {
    fn update_value<F: FnMut(&mut T) -> &mut T>(self, updator: F) -> Self;
}

pub trait UiComponentSpawnerActivator<'w, 's, 'a, T, S> {
    fn spawn(self) -> Option<EntityCommands<'w, 's, 'a>>;
    fn with_children<F: FnOnce(&mut bevy::prelude::ChildBuilder<'_, '_, '_>)>(
        self,
        f: F,
    ) -> Option<EntityCommands<'w, 's, 'a>>
    where
        Self: Sized,
    {
        let mut commands = self.spawn();
        if let Some(commands) = &mut commands {
            commands.with_children(move |builder| f(builder));
        }
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
    fn button<'a, N: Into<String>, T: Into<String>>(
        &'a mut self,
        name: N,
        text: T,
    ) -> UiComponent<'w, 's, 'a, GameButton, Self>
    where
        Self: Sized,
    {
        UiComponent::new(GameButton::new(name, text), self)
    }

    fn icon<'a>(&'a mut self, icon: Handle<Image>) -> UiComponent<'w, 's, 'a, GameIcon, Self>
    where
        Self: Sized,
    {
        UiComponent::new(GameIcon::new(icon), self)
    }
}

impl<'w, 's, 'a, T: Component + Clone, S: InternalUiSpawner<'w, 's>> UiComponentSpawner<T>
    for UiComponent<'w, 's, 'a, T, S>
{
    fn update_value<F: FnMut(&mut T) -> &mut T>(mut self, mut updator: F) -> Self {
        updator(&mut self.value);
        self
    }
}

impl<'w, 's, 'a, T: Component + Clone, S: InternalUiSpawner<'w, 's>>
    UiComponentSpawnerActivator<'w, 's, 'a, T, S> for UiComponent<'w, 's, 'a, T, S>
{
    fn spawn(mut self) -> Option<EntityCommands<'w, 's, 'a>> {
        let spawner = self.spawner.take();
        spawner.map(|spawner| {
            let spawn = self.id_function.as_mut();
            spawn(spawner, &self.value)
        })
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

impl<'w, 's, 'a, T: Component + Clone, S: InternalUiSpawner<'w, 's>> Drop
    for UiComponent<'w, 's, 'a, T, S>
{
    fn drop(&mut self) {
        if let Some(spawner) = self.spawner.take() {
            let spawn = self.id_function.as_mut();
            spawn(spawner, &self.value);
        }
    }
}
