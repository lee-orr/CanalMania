use bevy::{
    prelude::{Event, Resource, States},
    reflect::Reflect,
};
use serde::{Deserialize, Serialize};

use super::board::Tile;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, States, Default)]
pub enum GameState {
    #[default]
    Setup,
    Description,
    InGame,
    Editor,
    Complete,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect, States, Default,
)]
pub enum GameActionMode {
    #[default]
    None,
    DigCanal,
    ConstructLock,
    BuildAquaduct,
    Demolish,
}

#[derive(Resource, Debug, Default, Clone, Reflect)]
pub struct GameResources {
    pub cost_so_far: usize,
}

#[derive(Debug, Clone, Event)]
pub enum GameActions {
    DigCanal(Tile),
    ConstructLock(Tile),
    BuildAquaduct(Tile, usize),
    Demolish(Tile),
}
