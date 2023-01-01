use crate::assets::CanalManiaAssets;

use bevy::ui::{JustifyContent, FocusPolicy};
use bevy::prelude::*;

#[derive(Clone, Component, Debug)]
pub struct GameButton {
    pub(crate) text: String,
    pub name: String,
    pub style: ButtonStyle,
}

#[derive(Clone, Debug)]
pub enum ButtonStyle {
    Primary,
    Secondary
}

impl Default for GameButton {
    fn default() -> Self {
        Self {
            text: Default::default(),
            name: Default::default(),
            style: ButtonStyle::Primary
        }
    }
}

impl GameButton {
    pub fn new<R: Into<String>,T: Into<String>>(name: R,text: T) -> Self {
        Self {
            name: name.into(),
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn style(self, style: ButtonStyle) -> Self {
        Self { style, ..self}
    }
}

impl ButtonStyle {
    fn main_color(&self) -> Color {
        match self {
            Self::Primary => Color::rgb_u8(213, 194, 125),
            Self::Secondary => Color::rgb_u8(244, 235, 201),
        }
    }

    fn hover_color(&self) -> Color {
        match self {
            Self::Primary => Color::rgb_u8(162, 147, 95),
            Self::Secondary => Color::rgb_u8(193, 185, 158),
        }
    }

    fn click_color(&self) -> Color {
        match self {
            Self::Primary => Color::rgb_u8(110, 100, 65),
            Self::Secondary => Color::rgb_u8(193, 185, 158),
        }
    }
}

pub(crate) fn spawn_button(mut commands: Commands, assets: Res<CanalManiaAssets>, buttons: Query<(Entity, &GameButton), Changed<GameButton>>) {
    for (entity, button) in buttons.iter() {
        println!("Spawning button: {button:?}");
        let text = button.text.clone();
        let size = 25.;
        let style = TextStyle {
            font:assets.font.clone(),
            font_size: size,
            color: Color::rgb_u8(94, 87, 71),
        };
        commands.entity(entity).despawn_descendants();
        commands.entity(entity).insert(ButtonBundle {
            background_color: button.style.main_color().into(),
            style: Style {
                padding: UiRect::all(Val::Px(20.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.)),
                margin: UiRect::all(Val::Px(5.)),
                ..Default::default()
            },
            ..Default::default()
        });
        commands.entity(entity).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(text, style.clone())
                    .with_style(Style {
                        max_size: Size::new(Val::Undefined, Val::Px(size)),
                        margin: UiRect::all(Val::Px(4.)),
                        ..Default::default()
                    }),
            );
        });
    }
}


#[derive(Debug, Clone)]
pub struct ButtonClickEvent(pub String, pub Entity);

pub fn button_events(
    mut buttons: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &GameButton,
        ),
        Changed<Interaction>,
    >,
    mut click_event: EventWriter<ButtonClickEvent>,
) {
    for (entity, interaction, mut background, button) in &mut buttons {
        match *interaction {
            Interaction::Hovered => {
                *background = button.style.hover_color().into();
            }
            Interaction::Clicked => {
                *background = button.style.click_color().into();
                info!("Clicked on {} - {:?}", &button.name, &entity);
                click_event.send(ButtonClickEvent(button.name.clone(), entity))
            }
            Interaction::None => {
                *background = button.style.main_color().into();
            }
        }
    }
}