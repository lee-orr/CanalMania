#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AppLoadingState {
    Loading,
    Loaded,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    MainMenu,
    Credits,
    ChooseLevel,
    InGame,
}
