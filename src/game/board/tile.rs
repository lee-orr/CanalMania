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
    pub wetness: Wetness,
    #[serde(default)]
    pub cost_modifier: TileCostModifier,
}

#[derive(Clone, Copy, Debug, Reflect, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum TileCostModifier {
    None,
    Multiplier,
    Blocked,
}

impl Default for TileCostModifier {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, Reflect, Serialize, Deserialize, PartialEq, Eq)]
pub enum Wetness {
    Dry,
    WaterSource,
    Wet(usize),
}

impl Default for Wetness {
    fn default() -> Self {
        Self::Dry
    }
}

#[derive(Component, Clone, Copy, Debug, Reflect, Serialize, Deserialize, PartialEq, Eq)]
pub enum WetnessSource {
    None,
    Source(usize, usize),
}

impl PartialOrd for WetnessSource {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WetnessSource {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (WetnessSource::None, WetnessSource::None) => std::cmp::Ordering::Equal,
            (WetnessSource::None, WetnessSource::Source(_, _)) => std::cmp::Ordering::Greater,
            (WetnessSource::Source(_, _), WetnessSource::None) => std::cmp::Ordering::Less,
            (WetnessSource::Source(x1, y1), WetnessSource::Source(x2, y2)) => {
                let a = x1 * 10000 + y1;
                let b = x2 * 10000 + y2;
                a.cmp(&b)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::WetnessSource;

    #[test]
    fn wetness_source_order() {
        let a = WetnessSource::None;
        let b = WetnessSource::Source(3, 2);
        let c = WetnessSource::Source(1, 5);

        assert!(a > b);
        assert!(b > c);
    }
}

#[derive(Component, Default, Clone, Debug, Reflect)]
pub struct TileNeighbours(pub [Option<Entity>; 4]);

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileType {
    Land,
    Farm,
    City,
    Sea,
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
    River,
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
            TileContents::River => assets.river_center.clone(),
        }
    }
    pub fn line(&self, assets: &CanalManiaAssets) -> Handle<Mesh> {
        match self {
            TileContents::None => Handle::default(),
            TileContents::Road => assets.road_line.clone(),
            TileContents::Canal => assets.canal_line.clone(),
            TileContents::Lock => assets.lock.clone(),
            TileContents::Aquaduct(_) => assets.aquaduct_line.clone(),
            TileContents::River => assets.river_line.clone(),
        }
    }
    pub fn end(&self, assets: &CanalManiaAssets) -> Handle<Mesh> {
        match self {
            TileContents::None => Handle::default(),
            TileContents::Road => assets.road_end.clone(),
            TileContents::Canal => assets.canal_end.clone(),
            TileContents::Lock => assets.lock.clone(),
            TileContents::Aquaduct(_) => assets.aquaduct_end.clone(),
            TileContents::River => assets.river_end.clone(),
        }
    }
}

impl Tile {
    fn get_modified_cost(&self, cost: usize) -> Option<usize> {
        match self.cost_modifier {
            TileCostModifier::None => Some(cost),
            TileCostModifier::Multiplier => Some(cost * 2),
            TileCostModifier::Blocked => None,
        }
    }

    pub fn get_dig_cost(&self) -> Option<usize> {
        if self.contents == TileContents::River {
            return None;
        }
        let type_cost = match self.tile_type {
            TileType::Land => 3,
            TileType::Farm => 4,
            TileType::City => 6,
            TileType::Sea => 1,
        };
        let road_cost = usize::from(self.contents == TileContents::Road);
        self.get_modified_cost(type_cost + road_cost)
    }

    pub fn get_lock_cost(&self) -> Option<usize> {
        self.get_dig_cost().map(|a| a + 1)
    }

    pub fn get_aquaduct_cost(&self) -> Option<usize> {
        self.get_dig_cost().map(|a| a + 2)
    }

    pub fn get_demolish_cost(&self) -> Option<usize> {
        match self.contents {
            TileContents::None => None,
            TileContents::Road => Some(1),
            TileContents::Canal => Some(3),
            TileContents::Lock => Some(4),
            TileContents::Aquaduct(h) => Some(5 * h),
            TileContents::River => None,
        }
    }

    pub fn get_decorations(&self, assets: &CanalManiaAssets) -> Vec<Handle<Mesh>> {
        let count = match self.contents {
            TileContents::None => match self.tile_type {
                TileType::Land => 3.,
                TileType::Farm => 3.,
                TileType::City => 8.,
                TileType::Sea => 0.,
            },
            _ => match self.tile_type {
                TileType::Land => 1.,
                TileType::Farm => 1.,
                TileType::City => 4.,
                TileType::Sea => 0.,
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
                    TileType::Sea => assets.house.clone(),
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
