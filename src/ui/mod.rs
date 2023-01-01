mod text;
mod ui_root;
mod button;
use bevy::ecs::schedule::StateData;
use bevy::{prelude::*, ui::JustifyContent};
use iyes_loopless::prelude::IntoConditionalSystem;
use crate::{assets::CanalManiaAssets, app_state::AppLoadingState};
pub use text::*;
pub use ui_root::*;
pub use button::*;
use std::fmt::Debug;
use std::hash::Hash;


pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ButtonClickEvent>()
            .add_system(spawn_text.run_in_state(AppLoadingState::Loaded))
            .add_system(spawn_ui_root.run_in_state(AppLoadingState::Loaded))
            .add_system(spawn_button.run_in_state(AppLoadingState::Loaded))
            .add_system(button_events.run_in_state(AppLoadingState::Loaded));
    }
}
