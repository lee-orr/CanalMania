use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::game::level::LevelList;

#[derive(AssetCollection, Resource)]
pub struct CanalManiaAssets {
    #[asset(key = "font")]
    pub font: Handle<Font>,
    #[asset(key = "font_italic")]
    pub font_italic: Handle<Font>,
    #[asset(key = "tile_center")]
    pub tile_center: Handle<Mesh>,
    #[asset(key = "tile_corner")]
    pub tile_corner: Handle<Mesh>,
    #[asset(key = "tile_edge")]
    pub tile_edge: Handle<Mesh>,
    #[asset(key = "farm_center")]
    pub farm_center: Handle<Mesh>,
    #[asset(key = "farm_corner")]
    pub farm_corner: Handle<Mesh>,
    #[asset(key = "farm_edge")]
    pub farm_edge: Handle<Mesh>,
    #[asset(key = "road_center")]
    pub road_center: Handle<Mesh>,
    #[asset(key = "road_corner")]
    pub road_corner: Handle<Mesh>,
    #[asset(key = "road_edge")]
    pub road_edge: Handle<Mesh>,
    #[asset(key = "city_center")]
    pub city_center: Handle<Mesh>,
    #[asset(key = "city_corner")]
    pub city_corner: Handle<Mesh>,
    #[asset(key = "city_edge")]
    pub city_edge: Handle<Mesh>,
    #[asset(key = "canal_center")]
    pub canal_center: Handle<Mesh>,
    #[asset(key = "canal_corner")]
    pub canal_corner: Handle<Mesh>,
    #[asset(key = "canal_edge")]
    pub canal_edge: Handle<Mesh>,
    #[asset(key = "canal_wet_center")]
    pub canal_wet_center: Handle<Mesh>,
    #[asset(key = "canal_wet_corner")]
    pub canal_wet_corner: Handle<Mesh>,
    #[asset(key = "canal_wet_edge")]
    pub canal_wet_edge: Handle<Mesh>,
    #[asset(key = "lock_center")]
    pub lock_center: Handle<Mesh>,
    #[asset(key = "lock_corner")]
    pub lock_corner: Handle<Mesh>,
    #[asset(key = "lock_edge")]
    pub lock_edge: Handle<Mesh>,
    #[asset(key = "lock_wet_center")]
    pub lock_wet_center: Handle<Mesh>,
    #[asset(key = "lock_wet_corner")]
    pub lock_wet_corner: Handle<Mesh>,
    #[asset(key = "lock_wet_edge")]
    pub lock_wet_edge: Handle<Mesh>,
    #[asset(key = "tile_texture")]
    pub tile_texture: Handle<Image>,
    #[asset(key = "paper")]
    pub paper: Handle<Image>,
    #[asset(key = "level_list")]
    pub level_list: Handle<LevelList>,
}
