use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::ui::*;

use super::game_state::GameState;

pub struct TurnStartUiPlugin;

impl Plugin for TurnStartUiPlugin {
    fn build(&self, app: &mut App) {
        clear_ui_system_set(app, GameState::TurnStart)
            .add_enter_system(GameState::TurnStart, display_ui)
            .add_system(button_pressed.run_in_state(GameState::TurnStart));
    }
}

fn display_ui(mut commands: Commands) {
    commands
        .ui_root()
        .position(Val::Px(0.), Val::Px(0.), Val::Auto, Val::Px(0.))
        .padding(0.)
        .spawn()
        .with_children(|parent| {
            parent
                .div()
                .opaque()
                .horizontal()
                .spawn()
                .with_children(|parent| {
                    parent.text("Your turn...").spawn();
                });
        });

    #[cfg(feature = "dev")]
    commands
        .ui_root()
        .position(Val::Auto, Val::Px(0.), Val::Px(0.), Val::Auto)
        .spawn()
        .with_children(|parent| {
            parent
                .button("editor", "Editor")
                .style(ButtonStyle::Small)
                .spawn();
        });
}

fn button_pressed(mut events: EventReader<ButtonClickEvent>, mut commands: Commands) {
    for event in events.iter() {
        if event.0 == "editor" {
            commands.insert_resource(NextState(GameState::Editor));
        }
    }
}
