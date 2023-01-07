use bevy::prelude::*;
use noisy_bevy::simplex_noise_3d;
use serde::{Deserialize, Serialize};

use crate::assets::CanalManiaAssets;

#[derive(Component, Default, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    #[serde(default)]
    pub z: usize,
    #[serde(default)]
    pub tile_type: TileType,
    #[serde(default)]
    pub contents: TileContents,
    #[serde(default)]
    pub is_goal: bool,
    #[serde(default)]
    pub is_wet: bool,
}

#[derive(Component, Default, Clone, Debug)]
pub struct TileNeighbours(pub [Option<Entity>; 8]);

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileType {
    Land,
    Farm,
    City,
}

impl Default for TileType {
    fn default() -> Self {
        Self::Land
    }
}

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileContents {
    None,
    Road,
    Canal,
    Lock,
    Aquaduct(usize),
}

impl Default for TileContents {
    fn default() -> Self {
        Self::None
    }
}

impl TileContents {
    pub fn center(&self, assets: &CanalManiaAssets) -> Handle<Mesh> {
        match self {
            TileContents::None => Handle::default(),
            TileContents::Road => assets.road_center.clone(),
            TileContents::Canal => assets.canal_center.clone(),
            TileContents::Lock => Handle::default(),
            TileContents::Aquaduct(_) => assets.aquaduct_center.clone(),
        }
    }
    pub fn line(&self, assets: &CanalManiaAssets) -> Handle<Mesh> {
        match self {
            TileContents::None => Handle::default(),
            TileContents::Road => assets.road_line.clone(),
            TileContents::Canal => assets.canal_line.clone(),
            TileContents::Lock => assets.lock.clone(),
            TileContents::Aquaduct(_) => assets.aquaduct_line.clone(),
        }
    }
    pub fn end(&self, assets: &CanalManiaAssets) -> Handle<Mesh> {
        match self {
            TileContents::None => Handle::default(),
            TileContents::Road => assets.road_end.clone(),
            TileContents::Canal => assets.canal_end.clone(),
            TileContents::Lock => assets.lock.clone(),
            TileContents::Aquaduct(_) => assets.aquaduct_end.clone(),
        }
    }
}

impl Tile {
    pub fn get_dig_cost(&self) -> usize {
        let type_cost = match self.tile_type {
            TileType::Land => 3,
            TileType::Farm => 4,
            TileType::City => 6,
        };
        let road_cost = if self.contents == TileContents::Road {
            1
        } else {
            0
        };
        type_cost + road_cost
    }

    pub fn get_lock_cost(&self) -> usize {
        self.get_dig_cost() + 1
    }

    pub fn get_aquaduct_cost(&self) -> usize {
        self.get_dig_cost() + 2
    }

    pub fn get_demolish_cost(&self) -> usize {
        match self.contents {
            TileContents::None => 0,
            TileContents::Road => 1,
            TileContents::Canal => 3,
            TileContents::Lock => 4,
            TileContents::Aquaduct(h) => 5 * h,
        }
    }

    pub fn get_decorations(&self, assets: &CanalManiaAssets) -> Vec<Handle<Mesh>> {
        let count = match self.contents {
            TileContents::None => match self.tile_type {
                TileType::Land => 3.,
                TileType::Farm => 3.,
                TileType::City => 8.,
            },
            _ => match self.tile_type {
                TileType::Land => 1.,
                TileType::Farm => 1.,
                TileType::City => 4.,
            },
        };

        let mut pos = Vec3::new(self.x as f32, self.y as f32, self.z as f32);

        let amount = (simplex_noise_3d(pos).abs() * (count + 1.)).floor() as usize;

        (0..amount)
            .map(|i| {
                let i = i as f32;
                pos = Vec3::new(-1. * i, 2. * i, 0.24 * i) * pos;
                match self.tile_type {
                    TileType::Land => {
                        let index = (simplex_noise_3d(pos).abs() * 4.).floor() as usize;
                        match index {
                            1 => assets.tree2.clone(),
                            2 => assets.tree3.clone(),
                            3 => assets.tree4.clone(),
                            _ => assets.tree1.clone(),
                        }
                    }
                    TileType::Farm => {
                        let index = (simplex_noise_3d(pos).abs() * 6.).floor() as usize;
                        match index {
                            1 => assets.house2.clone(),
                            2 => assets.house3.clone(),
                            3 => assets.house4.clone(),
                            4 => assets.tree2.clone(),
                            5 => assets.tree3.clone(),
                            _ => assets.house.clone(),
                        }
                    }
                    TileType::City => {
                        let index = (simplex_noise_3d(pos).abs() * 4.).floor() as usize;
                        match index {
                            1 => assets.house2.clone(),
                            2 => assets.house3.clone(),
                            3 => assets.house4.clone(),
                            _ => assets.house.clone(),
                        }
                    }
                }
            })
            .collect()
    }
}

pub fn tile_position(x: Option<usize>, y: Option<usize>) -> Option<(usize, usize)> {
    if let (Some(x), Some(y)) = (x, y) {
        Some((x, y))
    } else {
        None
    }
}
