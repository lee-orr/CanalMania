use bevy::prelude::Resource;

use super::board::Tile;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Setup,
    InGame,
    Editor,
    Complete,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameActionMode {
    None,
    DigCanal,
    ConstructLock,
    BuildAquaduct,
    Demolish,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct GameResources {
    pub cost_so_far: usize,
}

#[derive(Debug, Clone)]
pub enum GameActions {
    DigCanal(Tile),
    ConstructLock(Tile),
    BuildAquaduct(Tile, usize),
    Demolish(Tile),
}
