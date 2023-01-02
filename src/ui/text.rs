use crate::assets::CanalManiaAssets;

use bevy::prelude::*;
use bevy::ui::JustifyContent;

use super::ui_id::WithUiId;

#[derive(Clone, Component, Debug)]
pub struct GameText {
    pub(crate) text: String,
    pub(crate) size: f32,
    pub(crate) alignment: JustifyContent,
    pub(crate) style: FontStyle,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FontStyle {
    Regular,
    Italic,
}

impl Default for GameText {
    fn default() -> Self {
        Self {
            text: Default::default(),
            size: 30.,
            alignment: JustifyContent::FlexStart,
            style: FontStyle::Regular,
        }
    }
}


impl WithUiId for GameText {}

impl GameText {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn size(self, size: f32) -> Self {
        Self { size, ..self }
    }

    pub fn style(self, style: FontStyle) -> Self {
        Self { style, ..self }
    }
}

pub(crate) fn spawn_text(
    mut commands: Commands,
    assets: Res<CanalManiaAssets>,
    texts: Query<(Entity, &GameText), Changed<GameText>>,
) {
    let main_text_root_style: NodeBundle = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        ..Default::default()
    };

    for (entity, main_text) in texts.iter() {
        println!("Spawning text: {main_text:?}");
        let text = main_text.text.clone();
        let size = main_text.size;
        let justify_content = main_text.alignment;

        let style = TextStyle {
            font: match main_text.style {
                FontStyle::Regular => assets.font.clone(),
                FontStyle::Italic => assets.font_italic.clone(),
            },
            font_size: size,
            color: Color::rgb_u8(94, 87, 71),
        };
        commands.entity(entity).insert(main_text_root_style.clone());
        commands.entity(entity).despawn_descendants();
        commands.entity(entity).with_children(|parent| {
            for line in text.lines() {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            flex_wrap: FlexWrap::Wrap,
                            align_content: AlignContent::FlexStart,
                            justify_content,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        for word in line.split_whitespace() {
                            parent.spawn(
                                TextBundle::from_section(format!("{word} "), style.clone())
                                    .with_style(Style {
                                        max_size: Size::new(Val::Undefined, Val::Px(size)),
                                        margin: UiRect::all(Val::Px(4.)),
                                        ..Default::default()
                                    }),
                            );
                        }
                    });
            }
        });
    }
}
