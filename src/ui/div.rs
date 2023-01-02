use bevy::prelude::*;

use std::fmt::Debug;

use super::ui_id::WithUiId;
use super::Background;

#[derive(Component, Debug, Default)]
pub struct Div {
    pub div_type: DivType,
    pub background: Background,
    pub direction: Direction,
    pub padding: f32,
}

#[derive(Debug)]
pub enum DivType {
    Auto,
    Positioned(UiRect),
}

impl Default for DivType {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Debug)]
pub enum Direction {
    Vertical,
    Horizontal,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Vertical
    }
}
impl WithUiId for Div {}

impl Div {
    pub fn new() -> Self {
        Self {
            padding: 5.,
            ..Div::default()
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
            div_type: DivType::Positioned(UiRect::new(left, right, top, bottom)),
            ..self
        }
    }

    pub fn horizontal(self) -> Self {
        Self {
            direction: Direction::Horizontal,
            ..self
        }
    }

    pub fn padding(self, padding: f32) -> Self {
        Self { padding, ..self }
    }
}

pub fn spawn_div(mut commands: Commands, roots: Query<(Entity, &Div), Added<Div>>) {
    for (entity, div) in roots.iter() {
        commands.entity(entity).insert((NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: match div.direction {
                    Direction::Vertical => FlexDirection::Column,
                    Direction::Horizontal => FlexDirection::Row,
                },
                padding: UiRect::all(Val::Px(div.padding)),
                position_type: match div.div_type {
                    DivType::Auto => PositionType::Relative,
                    DivType::Positioned(_) => PositionType::Absolute,
                },
                position: match div.div_type {
                    DivType::Auto => UiRect::default(),
                    DivType::Positioned(rect) => rect,
                },
                ..Default::default()
            },
            background_color: match div.background {
                Background::Transparent => Color::rgba(0., 0., 0., 0.).into(),
                Background::Opaque => Color::rgb_u8(177, 162, 124).into(),
            },
            ..Default::default()
        },));
    }
}
