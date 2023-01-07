use bevy::prelude::*;

use std::fmt::Debug;

use super::{Background, UiComponentSpawner};

#[derive(Component, Debug, Default, Clone)]
pub struct Div {
    pub div_type: DivType,
    pub background: Background,
    pub direction: Direction,
    pub padding: f32,
    pub size: Size,
}

#[derive(Debug, Clone)]
pub enum DivType {
    Auto,
    Positioned(UiRect),
}

impl Default for DivType {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Vertical,
    Horizontal,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Vertical
    }
}

impl Div {
    pub fn new() -> Self {
        Self {
            padding: 5.,
            ..Div::default()
        }
    }

    pub fn opaque(&mut self) -> &mut Self {
        self.background = Background::Opaque;
        self
    }

    pub fn position(&mut self, left: Val, right: Val, top: Val, bottom: Val) -> &mut Self {
        self.div_type = DivType::Positioned(UiRect::new(left, right, top, bottom));
        self
    }

    pub fn horizontal(&mut self) -> &mut Self {
        self.direction = Direction::Horizontal;
        self
    }

    pub fn padding(&mut self, padding: f32) -> &mut Self {
        self.padding = padding;
        self
    }

    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = size;
        self
    }
}

pub trait DivSpawner {
    fn position(self, left: Val, right: Val, top: Val, bottom: Val) -> Self;

    fn padding(self, padding: f32) -> Self;
    fn size(self, size: Size) -> Self;

    fn horizontal(self) -> Self;

    fn opaque(self) -> Self;
}

impl<T: UiComponentSpawner<Div>> DivSpawner for T {
    fn position(self, left: Val, right: Val, top: Val, bottom: Val) -> Self {
        self.update_value(|v| v.position(left, right, top, bottom))
    }

    fn padding(self, padding: f32) -> Self {
        self.update_value(|v| v.padding(padding))
    }

    fn horizontal(self) -> Self {
        self.update_value(|v| v.horizontal())
    }

    fn opaque(self) -> Self {
        self.update_value(|v| v.opaque())
    }

    fn size(self, size: Size) -> Self {
        self.update_value(|v| v.size(size))
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
                size: div.size,
                ..Default::default()
            },
            background_color: match div.background {
                Background::Transparent => Color::rgba(0., 0., 0., 0.).into(),
                Background::Opaque => Color::rgba_u8(177, 162, 124, 150).into(),
            },
            ..Default::default()
        },));
    }
}
