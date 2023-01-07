use crate::assets::CanalManiaAssets;

use bevy::prelude::*;
use bevy::ui::{JustifyContent, FocusPolicy};

use super::UiComponentSpawner;
use super::div::Direction;

#[derive(Clone, Component, Debug)]
pub struct GameButton {
    pub(crate) text: String,
    pub name: String,
    pub style: ButtonStyle,
    pub icon: Option<Handle<Image>>,
    pub hover_direction: Direction,
}

#[derive(Clone, Debug)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Small,
}

impl Default for GameButton {
    fn default() -> Self {
        Self {
            text: Default::default(),
            name: Default::default(),
            style: ButtonStyle::Primary,
            icon: None,
            hover_direction: Direction::Vertical,
        }
    }
}

impl GameButton {
    pub fn new<R: Into<String>, T: Into<String>>(name: R, text: T) -> Self {
        Self {
            name: name.into(),
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn style(&mut self, style: ButtonStyle) -> &mut Self {
        self.style = style;
        self
    }

    pub fn icon(&mut self, icon: Handle<Image>) -> &mut Self {
        self.icon = Some(icon);
        self
    }

    pub fn hover_direction(&mut self, hover_direction: Direction) -> &mut Self {
        self.hover_direction = hover_direction;
        self
    }
}

pub trait ButtonSpawner {
    fn style(self, style: ButtonStyle) -> Self;
    fn icon(self, image: Handle<Image>) -> Self;
    fn hover_direction(self, hover_direction: Direction) -> Self;
}

impl<T: UiComponentSpawner<GameButton>> ButtonSpawner for T {
    fn style(self, style: ButtonStyle) -> Self {
        self.update_value(move |v| v.style(style.clone()))
    }

    fn icon(self, icon: Handle<Image>) -> Self {
        self.update_value(move |v| v.icon(icon.clone()))
    }

    fn hover_direction(self, hover_direction: Direction) -> Self {
        self.update_value(move |v| v.hover_direction(hover_direction.clone()))
    }
}

impl ButtonStyle {
    fn main_color(&self) -> Color {
        match self {
            Self::Primary => Color::rgb_u8(213, 194, 125),
            Self::Secondary => Color::rgb_u8(244, 235, 201),
            Self::Small => Color::rgb_u8(213, 194, 125),
        }
    }

    fn hover_color(&self) -> Color {
        match self {
            Self::Primary => Color::rgb_u8(162, 147, 95),
            Self::Secondary => Color::rgb_u8(193, 185, 158),
            Self::Small => Color::rgb_u8(162, 147, 95),
        }
    }

    fn click_color(&self) -> Color {
        match self {
            Self::Primary => Color::rgb_u8(110, 100, 65),
            Self::Secondary => Color::rgb_u8(193, 185, 158),
            Self::Small => Color::rgb_u8(110, 100, 65),
        }
    }

    fn padding(&self) -> f32 {
        match self {
            Self::Small => 5.,
            _ => 20.,
        }
    }

    fn text_size(&self) -> f32 {
        match self {
            Self::Small => 15.,
            _ => 25.,
        }
    }
}

pub(crate) fn spawn_button(
    mut commands: Commands,
    assets: Res<CanalManiaAssets>,
    buttons: Query<(Entity, &GameButton), Changed<GameButton>>,
) {
    for (entity, button) in buttons.iter() {
        println!("Spawning button: {button:?}");
        let text = button.text.clone();
        let size = button.style.text_size();
        let style = TextStyle {
            font: assets.font.clone(),
            font_size: size,
            color: Color::rgb_u8(94, 87, 71),
        };
        commands.entity(entity).despawn_descendants();
        commands.entity(entity).insert(ButtonBundle {
            background_color: button.style.main_color().into(),
            style: Style {
                padding: UiRect::all(Val::Px(button.style.padding())),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.)),
                margin: UiRect::all(Val::Px(5.)),
                overflow: Overflow::Hidden,
                ..Default::default()
            },
            ..Default::default()
        });
        commands.entity(entity).with_children(|parent| {
            if let Some(icon) = &button.icon {
                parent.spawn(ImageBundle {
                    image: icon.clone().into(),
                    focus_policy: FocusPolicy::Pass,
                    style: Style {
                        size: Size::new(Val::Px(button.style.text_size()), Val::Px(button.style.text_size())),
                        ..Default::default()
                    },
                    ..Default::default()
                });

                parent.spawn(NodeBundle {
                    background_color: Color::rgba_u8(253, 231, 192, 150).into(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: match button.hover_direction {
                            Direction::Vertical =>UiRect::bottom(Val::Percent(100.)),
                            Direction::Horizontal => UiRect::left(Val::Percent(100.)),
                        },
                        padding: UiRect::all(Val::Px(5.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.)),
                        margin: UiRect::all(Val::Px(3.)),
                        overflow: Overflow::Hidden,
                        ..Default::default()
                    },
                    focus_policy: FocusPolicy::Pass,
                    ..Default::default()
                }).with_children(|parent|{
                    parent.spawn(
                        TextBundle::from_section(text, style.clone()).with_style(Style {
                            max_size: Size::new(Val::Undefined, Val::Px(size)),
                            margin: UiRect::all(Val::Px(4.)),
                            ..Default::default()
                        }),
                    );
                });
            } else {
                parent.spawn(
                    TextBundle::from_section(text, style.clone()).with_style(Style {
                        max_size: Size::new(Val::Undefined, Val::Px(size)),
                        margin: UiRect::all(Val::Px(4.)),
                        ..Default::default()
                    }),
                );
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct ButtonClickEvent(pub String, pub Entity);

pub fn button_events(
    mut buttons: Query<
        (Entity, &Interaction, &mut BackgroundColor, &GameButton, &mut Style),
        Changed<Interaction>,
    >,
    mut click_event: EventWriter<ButtonClickEvent>,
) {
    for (entity, interaction, mut background, button, mut style) in &mut buttons {
        match *interaction {
            Interaction::Hovered => {
                *background = button.style.hover_color().into();
                style.overflow = Overflow::Visible;
            }
            Interaction::Clicked => {
                *background = button.style.click_color().into();
                style.overflow = Overflow::Hidden;
                info!("Clicked on {} - {:?}", &button.name, &entity);
                click_event.send(ButtonClickEvent(button.name.clone(), entity))
            }
            Interaction::None => {
                style.overflow = Overflow::Hidden;
                *background = button.style.main_color().into();
            }
        }
    }
}
