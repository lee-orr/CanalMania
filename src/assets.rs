use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::game::level::LevelList;

#[derive(AssetCollection, Resource)]
pub struct CanalManiaAssets {
    #[asset(key = "font")]
    pub font: Handle<Font>,
    #[asset(key = "font_italic")]
    pub font_italic: Handle<Font>,
    #[asset(key = "level_list")]
    pub level_list: Handle<LevelList>,

    #[asset(key = "aqueduct_icon")]
    pub aqueduct_icon: Handle<Image>,
    #[asset(key = "lock_icon")]
    pub lock_icon: Handle<Image>,
    #[asset(key = "demolish_icon")]
    pub demolish_icon: Handle<Image>,
    #[asset(key = "dig_canal_icon")]
    pub dig_canal_icon: Handle<Image>,
    #[asset(key = "menu_icon")]
    pub menu_icon: Handle<Image>,
    #[asset(key = "coin_icon")]
    pub coin_icon: Handle<Image>,
    #[asset(key = "coins_icon")]
    pub coins_icon: Handle<Image>,
    #[asset(key = "help_icon")]
    pub help_icon: Handle<Image>,

    #[asset(key = "land_tile")]
    pub land_tile: Handle<Mesh>,
    #[asset(key = "farm_tile")]
    pub farm_tile: Handle<Mesh>,
    #[asset(key = "city_tile")]
    pub city_tile: Handle<Mesh>,
    #[asset(key = "sea_tile")]
    pub sea_tile: Handle<Mesh>,
    #[asset(key = "tree1")]
    pub tree1: Handle<Mesh>,
    #[asset(key = "tree4")]
    pub tree4: Handle<Mesh>,
    #[asset(key = "tree3")]
    pub tree3: Handle<Mesh>,
    #[asset(key = "tree2")]
    pub tree2: Handle<Mesh>,
    #[asset(key = "house")]
    pub house: Handle<Mesh>,
    #[asset(key = "house2")]
    pub house2: Handle<Mesh>,
    #[asset(key = "house3")]
    pub house3: Handle<Mesh>,
    #[asset(key = "house4")]
    pub house4: Handle<Mesh>,
    #[asset(key = "goal")]
    pub goal: Handle<Mesh>,
    #[asset(key = "canal_line")]
    pub canal_line: Handle<Mesh>,
    #[asset(key = "canal_end")]
    pub canal_end: Handle<Mesh>,
    #[asset(key = "canal_center")]
    pub canal_center: Handle<Mesh>,
    #[asset(key = "river_line")]
    pub river_line: Handle<Mesh>,
    #[asset(key = "river_end")]
    pub river_end: Handle<Mesh>,
    #[asset(key = "river_center")]
    pub river_center: Handle<Mesh>,
    #[asset(key = "river_to_canal_line")]
    pub river_to_canal_line: Handle<Mesh>,
    #[asset(key = "river_to_canal_end")]
    pub river_to_canal_end: Handle<Mesh>,
    #[asset(key = "aquaduct_line")]
    pub aquaduct_line: Handle<Mesh>,
    #[asset(key = "aquaduct_end")]
    pub aquaduct_end: Handle<Mesh>,
    #[asset(key = "aquaduct_center")]
    pub aquaduct_center: Handle<Mesh>,
    #[asset(key = "lock")]
    pub lock: Handle<Mesh>,
    #[asset(key = "road_line")]
    pub road_line: Handle<Mesh>,
    #[asset(key = "road_end")]
    pub road_end: Handle<Mesh>,
    #[asset(key = "road_center")]
    pub road_center: Handle<Mesh>,
}
