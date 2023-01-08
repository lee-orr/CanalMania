use crate::assets::CanalManiaAssets;

use bevy::prelude::*;
use bevy::ui::{FocusPolicy, JustifyContent};

use super::div::Direction;
use super::UiComponentSpawner;

#[derive(Clone, Component, Debug)]
pub struct GameButton {
    pub(crate) text: String,
    pub name: String,
    pub style: ButtonStyle,
    pub icon: Option<Handle<Image>>,
    pub hover_direction: Direction,
    pub hidden: bool,
    pub selected: bool,
}

#[derive(Clone, Debug)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Small,
    Action,
}

impl Default for GameButton {
    fn default() -> Self {
        Self {
            text: Default::default(),
            name: Default::default(),
            style: ButtonStyle::Primary,
            icon: None,
            hover_direction: Direction::Vertical,
            hidden: false,
            selected: false,
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

    pub fn hidden(&mut self, hidden: bool) -> &mut Self {
        self.hidden = hidden;
        self
    }

    pub fn selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }
}

pub trait ButtonSpawner {
    fn style(self, style: ButtonStyle) -> Self;
    fn icon(self, image: Handle<Image>) -> Self;
    fn hover_direction(self, hover_direction: Direction) -> Self;
    fn hidden(self, hidden: bool) -> Self;
    fn selected(self, selected: bool) -> Self;
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

    fn hidden(self, hidden: bool) -> Self {
        self.update_value(move |v| v.hidden(hidden))
    }

    fn selected(self, selected: bool) -> Self {
        self.update_value(move |v| v.selected(selected))
    }
}

impl ButtonStyle {
    fn main_color(&self) -> Color {
        match self {
            Self::Primary => Color::rgb_u8(213, 194, 125),
            Self::Secondary => Color::rgb_u8(244, 235, 201),
            Self::Small => Color::rgb_u8(213, 194, 125),
            Self::Action => Color::rgb_u8(208, 170, 89),
        }
    }

    fn hover_color(&self) -> Color {
        match self {
            Self::Primary => Color::rgb_u8(162, 147, 95),
            Self::Secondary => Color::rgb_u8(193, 185, 158),
            Self::Small => Color::rgb_u8(162, 147, 95),
            Self::Action => Color::rgb_u8(233, 190, 99),
        }
    }

    fn selected_color(&self) -> Color {
        match self {
            Self::Primary => Color::rgb_u8(162, 147, 95),
            Self::Secondary => Color::rgb_u8(193, 185, 158),
            Self::Small => Color::rgb_u8(162, 147, 95),
            Self::Action => Color::rgb_u8(255, 217, 108),
        }
    }

    fn click_color(&self) -> Color {
        match self {
            Self::Primary => Color::rgb_u8(110, 100, 65),
            Self::Secondary => Color::rgb_u8(193, 185, 158),
            Self::Small => Color::rgb_u8(110, 100, 65),
            Self::Action => Color::rgb_u8(157, 128, 67),
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
            Self::Action => 35.,
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
        let text = button.text.clone();
        let size = button.style.text_size();
        let style = TextStyle {
            font: assets.font.clone(),
            font_size: size,
            color: Color::rgb_u8(94, 87, 71),
        };
        commands.entity(entity).despawn_descendants();
        commands.entity(entity).insert(ButtonBundle {
            background_color: if button.selected {
                button.style.selected_color().into()
            } else {
                button.style.main_color().into()
            },
            style: Style {
                padding: UiRect::all(Val::Px(button.style.padding())),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.)),
                margin: UiRect::all(Val::Px(5.)),
                overflow: Overflow::Hidden,
                display: if button.hidden {
                    Display::None
                } else {
                    Display::Flex
                },
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
                        size: Size::new(
                            Val::Px(button.style.text_size()),
                            Val::Px(button.style.text_size()),
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                });

                parent
                    .spawn(NodeBundle {
                        background_color: Color::rgba_u8(253, 231, 192, 150).into(),
                        style: Style {
                            position_type: PositionType::Absolute,
                            position: match button.hover_direction {
                                Direction::Vertical => UiRect::bottom(Val::Percent(100.)),
                                Direction::Horizontal => UiRect::new(
                                    Val::Percent(100.),
                                    Val::Auto,
                                    Val::Px(0.),
                                    Val::Auto,
                                ),
                            },
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(5.)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(1.)),
                            margin: UiRect::all(Val::Px(3.)),
                            overflow: Overflow::Visible,
                            size: Size::new(Val::Px(300.), Val::Auto),
                            ..Default::default()
                        },
                        focus_policy: FocusPolicy::Pass,
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        for line in text.lines() {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Row,
                                        flex_wrap: FlexWrap::Wrap,
                                        align_content: AlignContent::FlexStart,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    for word in line.split_whitespace() {
                                        parent.spawn(
                                            TextBundle::from_section(
                                                format!("{word} "),
                                                style.clone(),
                                            )
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
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &GameButton,
            &mut Style,
        ),
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
                click_event.send(ButtonClickEvent(button.name.clone(), entity))
            }
            Interaction::None => {
                style.overflow = Overflow::Hidden;
                *background = if button.selected {
                    button.style.selected_color().into()
                } else {
                    button.style.main_color().into()
                };
            }
        }
    }
}
