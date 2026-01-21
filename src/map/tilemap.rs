use bevy::math::{URect, UVec2};

/// 瓦片图定义常量
///
/// 定义了地图中所有可用的瓦片类型及其在纹理图集中的位置
pub const TILEMAP: TilemapDefinition = TilemapDefinition {
    // 单个瓦片的宽度（像素）
    tile_width: 32,
    // 单个瓦片的高度（像素）
    tile_height: 32,
    // 纹理图集的总宽度（像素）
    atlas_width: 256,
    // 纹理图集的总高度（像素）
    atlas_height: 320,
    // 瓦片精灵列表
    sprites: &[
        // 泥土瓦片
        TilemapSprite {
            name: "dirt",
            pixel_x: 128,
            pixel_y: 0,
        },
        // 绿色草地瓦片
        TilemapSprite {
            name: "green_grass",
            pixel_x: 160,
            pixel_y: 0,
        },
        // 绿色草地内角（左上）
        TilemapSprite {
            name: "green_grass_corner_in_tl",
            pixel_x: 192,
            pixel_y: 0,
        },
        // 绿色草地内角（右上）
        TilemapSprite {
            name: "green_grass_corner_in_tr",
            pixel_x: 224,
            pixel_y: 0,
        },
        // 绿色草地内角（左下）
        TilemapSprite {
            name: "green_grass_corner_in_bl",
            pixel_x: 192,
            pixel_y: 32,
        },
        // 绿色草地内角（右下）
        TilemapSprite {
            name: "green_grass_corner_in_br",
            pixel_x: 224,
            pixel_y: 32,
        },
        // 绿色草地外角（左上）
        TilemapSprite {
            name: "green_grass_corner_out_tl",
            pixel_x: 0,
            pixel_y: 64,
        },
        // 绿色草地外角（右上）
        TilemapSprite {
            name: "green_grass_corner_out_tr",
            pixel_x: 32,
            pixel_y: 64,
        },
        // 绿色草地外角（左下）
        TilemapSprite {
            name: "green_grass_corner_out_bl",
            pixel_x: 0,
            pixel_y: 96,
        },
        // 绿色草地外角（右下）
        TilemapSprite {
            name: "green_grass_corner_out_br",
            pixel_x: 32,
            pixel_y: 96,
        },
        // 绿色草地边缘（上）
        TilemapSprite {
            name: "green_grass_side_t",
            pixel_x: 64,
            pixel_y: 64,
        },
        // 绿色草地边缘（右）
        TilemapSprite {
            name: "green_grass_side_r",
            pixel_x: 96,
            pixel_y: 64,
        },
        // 绿色草地边缘（左）
        TilemapSprite {
            name: "green_grass_side_l",
            pixel_x: 64,
            pixel_y: 96,
        },
        // 绿色草地边缘（下）
        TilemapSprite {
            name: "green_grass_side_b",
            pixel_x: 96,
            pixel_y: 96,
        },
        // 黄色草地瓦片
        TilemapSprite {
            name: "yellow_grass",
            pixel_x: 0,
            pixel_y: 256,
        },
        // 黄色草地内角（左上）
        TilemapSprite {
            name: "yellow_grass_corner_in_tl",
            pixel_x: 32,
            pixel_y: 256,
        },
        // 黄色草地内角（右上）
        TilemapSprite {
            name: "yellow_grass_corner_in_tr",
            pixel_x: 64,
            pixel_y: 256,
        },
        // 黄色草地内角（左下）
        TilemapSprite {
            name: "yellow_grass_corner_in_bl",
            pixel_x: 32,
            pixel_y: 288,
        },
        // 黄色草地内角（右下）
        TilemapSprite {
            name: "yellow_grass_corner_in_br",
            pixel_x: 64,
            pixel_y: 288,
        },
        // 黄色草地外角（左上）
        TilemapSprite {
            name: "yellow_grass_corner_out_tl",
            pixel_x: 96,
            pixel_y: 256,
        },
        // 黄色草地外角（右上）
        TilemapSprite {
            name: "yellow_grass_corner_out_tr",
            pixel_x: 128,
            pixel_y: 256,
        },
        // 黄色草地外角（左下）
        TilemapSprite {
            name: "yellow_grass_corner_out_bl",
            pixel_x: 96,
            pixel_y: 288,
        },
        // 黄色草地外角（右下）
        TilemapSprite {
            name: "yellow_grass_corner_out_br",
            pixel_x: 128,
            pixel_y: 288,
        },
        // 黄色草地边缘（上）
        TilemapSprite {
            name: "yellow_grass_side_t",
            pixel_x: 160,
            pixel_y: 256,
        },
        // 黄色草地边缘（右）
        TilemapSprite {
            name: "yellow_grass_side_r",
            pixel_x: 192,
            pixel_y: 256,
        },
        // 黄色草地边缘（左）
        TilemapSprite {
            name: "yellow_grass_side_l",
            pixel_x: 160,
            pixel_y: 288,
        },
        // 黄色草地边缘（下）
        TilemapSprite {
            name: "yellow_grass_side_b",
            pixel_x: 192,
            pixel_y: 288,
        },
        // 水瓦片
        TilemapSprite {
            name: "water",
            pixel_x: 32,
            pixel_y: 192,
        },
        // 水内角（左上）
        TilemapSprite {
            name: "water_corner_in_tl",
            pixel_x: 64,
            pixel_y: 192,
        },
        // 水内角（右上）
        TilemapSprite {
            name: "water_corner_in_tr",
            pixel_x: 96,
            pixel_y: 192,
        },
        // 水内角（左下）
        TilemapSprite {
            name: "water_corner_in_bl",
            pixel_x: 64,
            pixel_y: 224,
        },
        // 水内角（右下）
        TilemapSprite {
            name: "water_corner_in_br",
            pixel_x: 96,
            pixel_y: 224,
        },
        // 水外角（左上）
        TilemapSprite {
            name: "water_corner_out_tl",
            pixel_x: 128,
            pixel_y: 192,
        },
        // 水外角（右上）
        TilemapSprite {
            name: "water_corner_out_tr",
            pixel_x: 160,
            pixel_y: 192,
        },
        // 水外角（左下）
        TilemapSprite {
            name: "water_corner_out_bl",
            pixel_x: 128,
            pixel_y: 224,
        },
        // 水外角（右下）
        TilemapSprite {
            name: "water_corner_out_br",
            pixel_x: 160,
            pixel_y: 224,
        },
        // 水边缘（上）
        TilemapSprite {
            name: "water_side_t",
            pixel_x: 192,
            pixel_y: 192,
        },
        // 水边缘（右）
        TilemapSprite {
            name: "water_side_r",
            pixel_x: 224,
            pixel_y: 192,
        },
        // 水边缘（左）
        TilemapSprite {
            name: "water_side_l",
            pixel_x: 192,
            pixel_y: 224,
        },
        // 水边缘（下）
        TilemapSprite {
            name: "water_side_b",
            pixel_x: 224,
            pixel_y: 224,
        },
        // 大树-1（左上）
        TilemapSprite {
            name: "big_tree_1_tl",
            pixel_x: 0,
            pixel_y: 0,
        },
        // 大树-1（右上）
        TilemapSprite {
            name: "big_tree_1_tr",
            pixel_x: 32,
            pixel_y: 0,
        },
        // 大树-1（左下）
        TilemapSprite {
            name: "big_tree_1_bl",
            pixel_x: 0,
            pixel_y: 32,
        },
        // 大树-1（右下）
        TilemapSprite {
            name: "big_tree_1_br",
            pixel_x: 32,
            pixel_y: 32,
        },
        // 大树-2（左上）
        TilemapSprite {
            name: "big_tree_2_tl",
            pixel_x: 64,
            pixel_y: 0,
        },
        // 大树-2（右上）
        TilemapSprite {
            name: "big_tree_2_tr",
            pixel_x: 96,
            pixel_y: 0,
        },
        // 大树-2（左下）
        TilemapSprite {
            name: "big_tree_2_bl",
            pixel_x: 64,
            pixel_y: 32,
        },
        // 大树-2（右下）
        TilemapSprite {
            name: "big_tree_2_br",
            pixel_x: 96,
            pixel_y: 32,
        },
        // 植物-1
        TilemapSprite {
            name: "plant_1",
            pixel_x: 128,
            pixel_y: 64,
        },
        // 植物-2
        TilemapSprite {
            name: "plant_2",
            pixel_x: 160,
            pixel_y: 64,
        },
        // 植物-3
        TilemapSprite {
            name: "plant_3",
            pixel_x: 192,
            pixel_y: 64,
        },
        // 植物-4
        TilemapSprite {
            name: "plant_4",
            pixel_x: 224,
            pixel_y: 64,
        },
        // 岩石-1
        TilemapSprite {
            name: "rock_1",
            pixel_x: 0,
            pixel_y: 128,
        },
        // 岩石-2
        TilemapSprite {
            name: "rock_2",
            pixel_x: 32,
            pixel_y: 128,
        },
        // 岩石-3
        TilemapSprite {
            name: "rock_3",
            pixel_x: 64,
            pixel_y: 128,
        },
        // 岩石-4
        TilemapSprite {
            name: "rock_4",
            pixel_x: 96,
            pixel_y: 128,
        },
        // 小树（顶部）
        TilemapSprite {
            name: "small_tree_top",
            pixel_x: 128,
            pixel_y: 128,
        },
        // 小树（底部）
        TilemapSprite {
            name: "small_tree_bottom",
            pixel_x: 128,
            pixel_y: 160,
        },
        // 树桩-1
        TilemapSprite {
            name: "tree_stump_1",
            pixel_x: 192,
            pixel_y: 128,
        },
        // 树桩-2
        TilemapSprite {
            name: "tree_stump_2",
            pixel_x: 224,
            pixel_y: 128,
        },
        // 树桩-3
        TilemapSprite {
            name: "tree_stump_3",
            pixel_x: 0,
            pixel_y: 192,
        },
    ],
};

/// 瓦片精灵结构体
///
/// 定义单个瓦片精灵在纹理图集中的位置
pub struct TilemapSprite {
    /// 瓦片名称
    pub name: &'static str,
    /// 纹理图集中的 X 坐标（像素）
    pub pixel_x: u32,
    /// 纹理图集中的 Y 坐标（像素）
    pub pixel_y: u32,
}

/// 瓦片图定义结构体
///
/// 定义整个瓦片图的配置参数
pub struct TilemapDefinition {
    /// 单个瓦片的宽度（像素）
    pub tile_width: u32,
    /// 单个瓦片的高度（像素）
    pub tile_height: u32,
    /// 纹理图集的总宽度（像素）
    pub atlas_width: u32,
    /// 纹理图集的总高度（像素）
    pub atlas_height: u32,
    /// 瓦片精灵列表
    pub sprites: &'static [TilemapSprite],
}

impl TilemapDefinition {
    /// 获取单个瓦片的大小
    pub const fn tile_size(&self) -> UVec2 {
        UVec2::new(self.tile_width, self.tile_height)
    }

    /// 获取纹理图集的总大小
    pub const fn atlas_size(&self) -> UVec2 {
        UVec2::new(self.atlas_width, self.atlas_height)
    }

    /// 根据名称查找瓦片索引
    ///
    /// # 参数
    /// - `name`: 瓦片名称
    ///
    /// # 返回
    /// 瓦片的索引，如果未找到则返回 None
    pub fn sprite_index(&self, name: &str) -> Option<usize> {
        self.sprites.iter().position(|sprite| sprite.name == name)
    }

    /// 获取指定索引瓦片的矩形区域
    ///
    /// # 参数
    /// - `index`: 瓦片索引
    ///
    /// # 返回
    /// 瓦片在纹理图集中的矩形区域
    pub fn sprite_rect(&self, index: usize) -> URect {
        let sprite = &self.sprites[index];
        let min = UVec2::new(sprite.pixel_x, sprite.pixel_y);
        URect::from_corners(min, min + self.tile_size())
    }
}
