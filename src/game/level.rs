use std::collections::VecDeque;

use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use serde::{Deserialize, Serialize};

use super::{
    board::{TileContents, TileCostModifier, TileType},
    game_state::GameActionMode,
};

#[derive(Resource, Component, Serialize, Deserialize, TypeUuid, Clone, TypePath)]
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
    #[serde(default)]
    pub tools: LevelTools,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LevelListing {
    pub name: String,
    pub file: String,
}

#[derive(Resource, Serialize, Deserialize, TypeUuid, Clone, TypePath)]
#[uuid = "8cbd35d0-111c-4881-8d1f-bec0ff21da47"]
pub struct LevelList {
    pub levels: Vec<LevelListing>,
}

#[derive(Serialize, Deserialize, Clone, Default, Reflect)]
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

#[derive(Resource, Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct LevelTools {
    pub canal: bool,
    pub lock: bool,
    pub aquaduct: bool,
    pub demolish: bool,
}

impl Default for LevelTools {
    fn default() -> Self {
        Self {
            canal: true,
            lock: true,
            aquaduct: true,
            demolish: true,
        }
    }
}

#[derive(Clone, Debug, Resource, Default)]
pub struct PendingLevelEvents(pub VecDeque<LevelEvent>);

#[derive(Clone, Debug, Serialize, Deserialize, Event)]
pub struct LevelEvent(pub LevelEventType, pub Vec<EventAction>);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Reflect)]
pub enum LevelEventType {
    GoalReached,
    AnyActionsComplete(usize, bool),
    BuiltNofType(usize, GameActionMode, bool),
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub enum EventAction {
    DisplayText {
        text: String,
        title: Option<String>,
        continue_button: Option<String>,
    },
    SetSidebar(Option<String>),
    SetNewGoal(usize, usize),
    AdjustCost(usize, usize, TileCostModifier),
    AdjustContents(usize, usize, TileContents),
    SetHeight(usize, usize, usize),
    AdjustToolAccess(GameActionMode, bool),
}
