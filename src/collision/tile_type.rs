use bevy::prelude::Component;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum TileType {
    // Walkable terrain
    #[default]
    Empty,
    Dirt,
    Grass,
    YellowGrass,
    Shore,
    // Non-walkable obstacles
    Water,
    Tree,
    Rock,
}

impl TileType {
    /// 检查瓦片是否可行走
    ///
    /// 返回 false 的瓦片类型：
    /// - Water: 深水区域（不可行走）
    /// - Tree: 树木（不可行走）
    /// - Rock: 岩石（不可行走）
    ///
    /// 返回 true 的瓦片类型：
    /// - Shore: 海岸/浅水区域（可行走）
    /// - Empty, Dirt, Grass, YellowGrass: 陆地区域（可行走）
    pub fn is_walkable(&self) -> bool {
        !matches!(self, TileType::Water | TileType::Tree | TileType::Rock)
    }

    /// Get the collision adjustment for this tile type.
    /// Positive = push player away, negative = allow corner cutting.
    pub fn collision_adjustment(&self) -> f32 {
        match self {
            TileType::Tree | TileType::Rock => -0.2,
            _ => 0.0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct TileMarker {
    pub tile_type: TileType,
}

impl TileMarker {
    pub fn new(tile_type: TileType) -> Self {
        Self { tile_type }
    }
}
