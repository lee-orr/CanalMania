mod button;
mod text;
mod ui_root;

use crate::app_state::AppLoadingState;
use bevy::prelude::*;
pub use button::*;
use iyes_loopless::prelude::IntoConditionalSystem;
pub use text::*;
pub use ui_root::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonClickEvent>()
            .add_system(spawn_text.run_in_state(AppLoadingState::Loaded))
            .add_system(spawn_ui_root.run_in_state(AppLoadingState::Loaded))
            .add_system(spawn_button.run_in_state(AppLoadingState::Loaded))
            .add_system(button_events.run_in_state(AppLoadingState::Loaded));
    }
}
