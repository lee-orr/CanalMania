use bevy::prelude::States;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, States)]
pub enum AppLoadingState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    Credits,
    ChooseLevel,
    InGame,
}
