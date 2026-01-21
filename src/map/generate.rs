use crate::config::map::{GRID_X, GRID_Y, TILE_SIZE};
use crate::map::assets::{load_assets, prepare_tilemap_handles};
use crate::map::rules::build_world;
use bevy::prelude::*;
use bevy_procedural_tilemaps::prelude::{
    CartesianGrid, Direction, GeneratorBuilder, ModelSelectionHeuristic, NodeSelectionHeuristic,
    NodesSpawner, RngMode, RulesBuilder,
};

/// 资源路径
const ASSETS_PATH: &str = "tile_layers";
/// 瓦片图文件名
const TILEMAP_FILE: &str = "tilemap.png";
/// 单个网格节点在世界单位中的大小
const NODE_SIZE: Vec3 = Vec3::new(TILE_SIZE, TILE_SIZE, 1.);
/// 资源缩放比例
const ASSETS_SCALE: Vec3 = Vec3::ONE;
/// 地图中的 Z 层数量，从默认地形层派生
const GRID_Z: u32 = 5;

/// 计算地图的像素尺寸
///
/// # 返回
/// 地图宽度和高度的 2D 向量
pub fn map_pixel_dimensions() -> Vec2 {
    Vec2::new(TILE_SIZE * GRID_X as f32, TILE_SIZE * GRID_Y as f32)
}

/// 设置地图生成器
///
/// 初始化地图生成系统，包括规则、网格和资源
pub fn setup_generator(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // 构建世界模型、资源和连接器集合
    let (assets_definitions, models, socket_collection) = build_world();

    // 创建 3D 笛卡尔坐标系的规则
    // 使用 Z 轴向前作为旋转轴
    let rules = RulesBuilder::new_cartesian_3d(models, socket_collection)
        .with_rotation_axis(Direction::ZForward)
        .build()
        .unwrap();

    // 创建 3D 笛卡尔网格
    let grid = CartesianGrid::new_cartesian_3d(GRID_X, GRID_Y, GRID_Z, false, false, false);

    // 构建地图生成器
    let gen_builder = GeneratorBuilder::new()
        // 添加规则
        .with_rules(rules)
        // 添加网格
        .with_grid(grid.clone())
        // 使用随机种子
        .with_rng(RngMode::RandomSeed)
        // 使用最小剩余值启发式算法（提高生成效率）
        .with_node_heuristic(NodeSelectionHeuristic::MinimumRemainingValue)
        // 使用加权概率启发式算法（控制模型选择）
        .with_model_heuristic(ModelSelectionHeuristic::WeightedProbability);

    // 创建生成器实例
    let generator = gen_builder.build().unwrap();

    // 准备瓦片图句柄
    let tilemap_handles =
        prepare_tilemap_handles(&asset_server, &mut atlas_layouts, ASSETS_PATH, TILEMAP_FILE);

    // 加载地图资源
    let models_assets = load_assets(&tilemap_handles, assets_definitions);

    // 生成地图实体
    commands.spawn((
        // 设置地图位置（居中）
        Transform::from_translation(Vec3 {
            x: -TILE_SIZE * grid.size_x() as f32 / 2.,
            y: -TILE_SIZE * grid.size_y() as f32 / 2.,
            z: 0.,
        }),
        // 添加网格组件
        grid,
        // 添加生成器组件
        generator,
        // 添加节点生成器（用于生成地图实体）
        NodesSpawner::new(models_assets, NODE_SIZE, ASSETS_SCALE).with_z_offset_from_y(true),
    ));
}
