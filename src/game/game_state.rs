use bevy::{
    prelude::Resource,
    reflect::{FromReflect, Reflect},
};
use serde::{Deserialize, Serialize};

use super::board::Tile;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum GameState {
    Setup,
    Description,
    InGame,
    Editor,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect, FromReflect)]
pub enum GameActionMode {
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

#[derive(Debug, Clone)]
pub enum GameActions {
    DigCanal(Tile),
    ConstructLock(Tile),
    BuildAquaduct(Tile, usize),
    Demolish(Tile),
}
