use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use super::div::Direction;
use super::UiComponentSpawner;

#[derive(Clone, Component, Debug)]
pub struct GameIcon {
    pub icon: Handle<Image>,
    pub hover_direction: Direction,
    pub size: GameIconSize,
}

impl Default for GameIcon {
    fn default() -> Self {
        Self {
            icon: Handle::default(),
            hover_direction: Direction::Vertical,
            size: GameIconSize::Normal,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameIconSize {
    Normal,
    Small,
}

impl GameIcon {
    pub fn new(icon: Handle<Image>) -> Self {
        Self {
            icon,
            ..Default::default()
        }
    }

    pub fn hover_direction(&mut self, hover_direction: Direction) -> &mut Self {
        self.hover_direction = hover_direction;
        self
    }

    pub fn size(&mut self, size: GameIconSize) -> &mut Self {
        self.size = size;
        self
    }
}

pub trait IconSpawner {
    fn hover_direction(self, hover_direction: Direction) -> Self;
    fn size(self, size: GameIconSize) -> Self;
}

impl<T: UiComponentSpawner<GameIcon>> IconSpawner for T {
    fn hover_direction(self, hover_direction: Direction) -> Self {
        self.update_value(move |v| v.hover_direction(hover_direction.clone()))
    }

    fn size(self, size: GameIconSize) -> Self {
        self.update_value(move |v| v.size(size))
    }
}

pub(crate) fn spawn_icon(
    mut commands: Commands,
    icons: Query<(Entity, &GameIcon), Changed<GameIcon>>,
) {
    for (entity, icon) in icons.iter() {
        let size = match icon.size {
            GameIconSize::Normal => 30.,
            GameIconSize::Small => 20.,
        };
        commands.entity(entity).insert(ImageBundle {
            image: icon.icon.clone().into(),
            focus_policy: FocusPolicy::Pass,
            style: Style {
                width: Val::Px(size),
                height: Val::Px(size),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
