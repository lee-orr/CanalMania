use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use iyes_loopless::prelude::IntoConditionalSystem;
use iyes_loopless::state::NextState;

use crate::ui::*;

use super::game_state::GameActionMode;
use super::game_state::GameState;

pub struct InitialDescriptionUiPlugin;

impl Plugin for InitialDescriptionUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        clear_ui_system_set(app, GameState::Description)
            .init_resource::<CurrentDescription>()
            .add_enter_system(GameState::Description, display_ui)
            .add_system(button_pressed.run_in_state(GameState::Description));
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub struct CurrentDescription {
    pub title: Option<String>,
    pub text: Option<String>,
    pub continue_button: Option<String>,
}

fn display_ui(mut commands: Commands, current_description: Res<CurrentDescription>) {
    if !current_description.is_changed() {
        return;
    }
    commands.insert_resource(NextState(GameActionMode::None));
    if let Some(description) = &current_description.text {
        commands
            .ui_root()
            .for_state(GameState::Description)
            .with_children(|parent| {
                parent.div().opaque().padding(5.).with_children(|parent| {
                    if let Some(title) = current_description.title.as_ref() {
                        parent.text(title).size(100.).style(FontStyle::Italic);
                    }
                    parent.text(description);
                    parent.div().padding(5.);

                    let button_text = current_description
                        .continue_button
                        .as_ref()
                        .unwrap_or(&"Continue".to_string())
                        .clone();
                    parent.button("play", button_text);
                });
            });
    }
}

fn button_pressed(mut events: EventReader<ButtonClickEvent>, mut commands: Commands) {
    for event in events.iter() {
        if event.0 == "play" {
            commands.insert_resource(NextState(GameState::InGame));
        }
    }
}
