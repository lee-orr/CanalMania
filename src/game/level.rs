use std::collections::VecDeque;

use bevy::{prelude::*, reflect::TypeUuid};
use serde::{Deserialize, Serialize};

use super::board::{TileContents, TileCostModifier, TileType};

#[derive(Resource, Component, Serialize, Deserialize, TypeUuid, Clone)]
#[uuid = "b9b5565a-a06a-4647-bc62-274f32ba6a5f"]
pub struct Level {
    pub tiles: Vec<Vec<TileInfo>>,
    pub title: Option<String>,
    pub initial_description: Option<String>,
    pub sidebar_text: Option<String>,
    pub width: usize,
    pub height: usize,
    #[serde(default)]
    pub events: Vec<LevelEvent>,
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
    #[serde(default)]
    pub tile_type: TileType,
    #[serde(default)]
    pub contents: TileContents,
    #[serde(default)]
    pub is_goal: bool,
    #[serde(default)]
    pub cost_modifier: TileCostModifier,
    pub height: usize,
}

#[derive(Clone, Debug, Resource, Default)]
pub struct PendingLevelEvents(pub VecDeque<LevelEvent>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LevelEvent(pub LevelEventType, pub Vec<EventAction>);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum LevelEventType {
    GoalReached,
    AnyActionsComplete(usize, bool),
    BuiltNofType(usize, TileContents, bool),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventAction {
    DisplayText {
        text: String,
        title: Option<String>,
        continue_button: Option<String>,
    },
    SetNewGoal(usize, usize),
    AdjustCost(usize, usize, TileCostModifier),
    AdjustContents(usize, usize, TileContents),
    SetHeight(usize, usize, usize),
}
