use bevy::{prelude::*, reflect::TypeUuid};
use serde::{Deserialize, Serialize};

use super::board::TileType;

#[derive(Resource, Component, Serialize, Deserialize, TypeUuid, Clone)]
#[uuid = "b9b5565a-a06a-4647-bc62-274f32ba6a5f"]
pub struct Level {
    pub tiles: Vec<Vec<TileInfo>>,
    pub title: Option<String>,
    pub width: usize,
    pub height: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LevelListing {
    pub name: String,
    pub file: String,
}

#[derive(Resource, Serialize, Deserialize, TypeUuid, Clone)]
#[uuid = "8cbd35d0-111c-4881-8d1f-bec0ff21da47"]
pub struct LevelList {
    pub levels: Vec<LevelListing>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TileInfo {
    pub tile_type: TileType,
    pub is_goal: bool,
    pub height: usize,
}
