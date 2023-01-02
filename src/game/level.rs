use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::board::Tile;

#[derive(Resource, Component, Serialize, Deserialize)]
pub struct Level {
    pub tiles: Vec<Tile>,
}
