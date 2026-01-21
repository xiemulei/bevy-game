use crate::map::tilemap::TILEMAP;
use bevy::prelude::*;
use bevy_procedural_tilemaps::prelude::{GridDelta, ModelAsset, ModelsAssets};

/// 可生成资源结构体
///
/// 定义可以在地图中生成的资源及其属性
#[derive(Clone)]
pub struct SpawnableAsset {
    /// 精灵名称（在瓦片图中的标识）
    sprite_name: &'static str,
    /// 网格偏移量
    grid_offset: GridDelta,
    /// 世界坐标偏移量
    offset: Vec3,
    /// 组件生成器函数（用于添加额外的组件）
    components_spawner: fn(&mut EntityCommands),
}

impl SpawnableAsset {
    /// 创建新的可生成资源
    ///
    /// # 参数
    /// - `sprite_name`: 精灵名称
    ///
    /// # 返回
    /// 新的可生成资源实例
    pub fn new(sprite_name: &'static str) -> Self {
        Self {
            sprite_name,
            grid_offset: GridDelta::new(0, 0, 0),
            offset: Vec3::ZERO,
            components_spawner: |_| {},
        }
    }

    /// 设置网格偏移量
    ///
    /// # 参数
    /// - `offset`: 网格偏移量
    ///
    /// # 返回
    /// 修改后的可生成资源
    pub fn with_grid_offset(mut self, offset: GridDelta) -> Self {
        self.grid_offset = offset;
        self
    }
}

/// 瓦片图句柄
///
/// 存储瓦片图的图像和布局句柄
#[derive(Clone)]
pub struct TilemapHandles {
    /// 图像句柄
    pub image: Handle<Image>,
    /// 纹理图集布局句柄
    pub layout: Handle<TextureAtlasLayout>,
}

impl TilemapHandles {
    /// 根据图集索引创建精灵
    ///
    /// # 参数
    /// - `atlas_index`: 图集中的索引
    ///
    /// # 返回
    /// 配置好的精灵
    pub fn sprite(&self, atlas_index: usize) -> Sprite {
        Sprite::from_atlas_image(
            self.image.clone(),
            TextureAtlas::from(self.layout.clone()).with_index(atlas_index),
        )
    }
}

/// 准备瓦片图句柄
///
/// 加载瓦片图图像并创建纹理图集布局
///
/// # 参数
/// - `asset_server`: 资源服务器
/// - `atlas_layouts`: 纹理图集布局资源集合
/// - `assets_directory`: 资源目录路径
/// - `tilemap_file`: 瓦片图文件名
///
/// # 返回
/// 瓦片图句柄
pub fn prepare_tilemap_handles(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    assets_directory: &str,
    tilemap_file: &str,
) -> TilemapHandles {
    // 加载瓦片图图像
    let image = asset_server.load::<Image>(format!("{assets_directory}/{tilemap_file}"));
    // 创建空的纹理图集布局
    let mut layout = TextureAtlasLayout::new_empty(TILEMAP.atlas_size());
    // 为每个精灵添加纹理区域
    for index in 0..TILEMAP.sprites.len() {
        layout.add_texture(TILEMAP.sprite_rect(index));
    }
    // 将布局添加到资源集合中
    let layout = atlas_layouts.add(layout);

    TilemapHandles { image, layout }
}

/// 加载地图资源
///
/// 将资源定义转换为可用的模型资源
///
/// # 参数
/// - `tilemap_handles`: 瓦片图句柄
/// - `assets_definitions`: 资源定义列表
///
/// # 返回
/// 模型资源集合
pub fn load_assets(
    tilemap_handles: &TilemapHandles,
    assets_definitions: Vec<Vec<SpawnableAsset>>,
) -> ModelsAssets<Sprite> {
    let mut models_assets = ModelsAssets::<Sprite>::new();

    // 遍历每个模型的资源定义
    for (model_index, assets) in assets_definitions.into_iter().enumerate() {
        // 遍历模型中的每个资源
        for asset_def in assets {
            let SpawnableAsset {
                sprite_name,
                grid_offset,
                offset,
                components_spawner,
            } = asset_def;

            // 根据名称查找图集索引
            let Some(atlas_index) = TILEMAP.sprite_index(sprite_name) else {
                panic!("Unknown atlas sprite '{}'", sprite_name);
            };

            // 将资源添加到模型资源集合中
            models_assets.add(
                model_index,
                ModelAsset {
                    assets_bundle: tilemap_handles.sprite(atlas_index),
                    grid_offset,
                    world_offset: offset,
                    spawn_commands: components_spawner,
                },
            )
        }
    }

    models_assets
}
