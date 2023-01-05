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
    #[asset(key = "land_tile")]
    #[asset(key = "land_tile")]
    pub land_tile: Handle<Mesh>,

    #[asset(key = "farm_tile")]
    pub farm_tile: Handle<Mesh>,

    #[asset(key = "city_tile")]
    pub city_tile: Handle<Mesh>,

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

    #[asset(key = "canal_crossing")]
    pub canal_crossing: Handle<Mesh>,

    #[asset(key = "canal_t")]
    pub canal_t: Handle<Mesh>,

    #[asset(key = "canal_corner")]
    pub canal_corner: Handle<Mesh>,

    #[asset(key = "canal_line")]
    pub canal_line: Handle<Mesh>,

    #[asset(key = "canal_end")]
    pub canal_end: Handle<Mesh>,

    #[asset(key = "canal_center")]
    pub canal_center: Handle<Mesh>,

    #[asset(key = "lock_crossing")]
    pub lock_crossing: Handle<Mesh>,

    #[asset(key = "lock_corner")]
    pub lock_corner: Handle<Mesh>,

    #[asset(key = "lock_end")]
    pub lock_end: Handle<Mesh>,

    #[asset(key = "lock_line")]
    pub lock_line: Handle<Mesh>,

    #[asset(key = "lock_t")]
    pub lock_t: Handle<Mesh>,

    #[asset(key = "canal_dry_crossing")]
    pub canal_dry_crossing: Handle<Mesh>,

    #[asset(key = "canal_dry_t")]
    pub canal_dry_t: Handle<Mesh>,

    #[asset(key = "canal_dry_corner")]
    pub canal_dry_corner: Handle<Mesh>,

    #[asset(key = "canal_dry_line")]
    pub canal_dry_line: Handle<Mesh>,

    #[asset(key = "canal_dry_end")]
    pub canal_dry_end: Handle<Mesh>,

    #[asset(key = "canal_dry_center")]
    pub canal_dry_center: Handle<Mesh>,

    #[asset(key = "road_corner")]
    pub road_corner: Handle<Mesh>,

    #[asset(key = "road_crossing")]
    pub road_crossing: Handle<Mesh>,

    #[asset(key = "road_t")]
    pub road_t: Handle<Mesh>,

    #[asset(key = "road_line")]
    pub road_line: Handle<Mesh>,

    #[asset(key = "road_end")]
    pub road_end: Handle<Mesh>,

    #[asset(key = "road_center")]
    pub road_center: Handle<Mesh>,
}
