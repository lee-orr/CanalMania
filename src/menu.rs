use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::app_state::*;
use crate::ui::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, mut app: &mut bevy::prelude::App) {
        clear_ui_system_set(&mut app, AppState::MainMenu)
            .add_enter_system(AppState::MainMenu,display_main_menu);        
    }
}

fn display_main_menu(mut commands: Commands) {
    println!("Display Main Menu");
    commands
        .spawn(UiRoot)
        .with_children(|parent| {
            parent.spawn(GameText::new("Canal Mania").size(100.).style(FontStyle::Italic));
            parent.spawn(GameButton::new("start_game", "Start Game"));
            parent.spawn(GameButton::new("exit_game", "Exit Game").style(ButtonStyle::Secondary));
        });
}