#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Setup,
    TurnStart,
    Editor,
}
