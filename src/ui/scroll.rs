use bevy::prelude::*;

use std::fmt::Debug;

use super::ui_id::WithUiId;
use super::{Background, Direction};

#[derive(Component, Debug, Default)]
pub struct Scroll {
    pub direction: super::div::Direction,
    pub padding: f32,
    pub size: Size,
}

impl WithUiId for Scroll {}

impl Scroll {
    pub fn new() -> Self {
        Self {
            padding: 5.,
            ..Scroll::default()
        }
    }

    pub fn horizontal(self) -> Self {
        Self {
            direction: super::div::Direction::Horizontal,
            ..self
        }
    }

    pub fn padding(self, padding: f32) -> Self {
        Self { padding, ..self }
    }

    pub fn size(self, size: Size) -> Self {
        Self {
            size,
            ..self
        }
    }
}

pub fn spawn_scroll(mut commands: Commands, roots: Query<(Entity, &Scroll), Added<Scroll>>) {
    for (entity, div) in roots.iter() {
        commands.entity(entity).insert(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Center,
                size: div.size,
                overflow: Overflow::Hidden,
                ..Default::default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.1).into(),
            ..Default::default()
        });
    }
}
