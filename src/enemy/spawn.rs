/// 敌人生成系统
///
/// 此模块负责敌人生成的相关功能，包括：
/// - 根据角色配置创建敌人实体
/// - 智能生成位置选择（避开障碍物）
/// - 批量生成测试敌人
use crate::characters::animation::{
    AnimationController, AnimationTimer, DEFAULT_ANIMATION_FRAME_TIME,
};
use crate::characters::collider::Collider;
use crate::characters::config::CharactersList;
use crate::characters::facing::Facing;
use crate::characters::physics::Velocity;
use crate::characters::spawn::CharactersListResource;
use crate::characters::state::CharacterState;
use crate::collision::CollisionMap;
use crate::config::enemy::{ENEMY_SCALE, ENEMY_Z_POSITION};
use crate::enemy::components::{AIBehavior, Enemy, EnemyCombat, EnemyPath};
use bevy::prelude::*;

/// 生成单个敌人实体
///
/// 根据角色名称从角色列表中查找配置，并创建完整的敌人实体。
/// 敌人实体包含所有必要的组件：精灵、动画、物理、AI、战斗等。
///
/// # 参数
/// - `commands`: 命令队列，用于生成实体
/// - `asset_server`: 资源服务器，用于加载纹理
/// - `atlas_layouts`: 图集布局资源集合
/// - `characters_list`: 角色列表配置
/// - `position`: 生成位置（世界坐标）
/// - `character_name`: 角色名称，用于从配置中查找
///
/// # 返回
/// - `Some(Entity)`: 生成的敌人实体 ID
/// - `None`: 角色名称不存在
pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &AssetServer,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    characters_list: &CharactersList,
    position: Vec3,
    character_name: &str,
) -> Option<Entity> {
    // 根据名字从角色列表中查找对应的角色配置
    let character_entry = characters_list
        .characters
        .iter()
        .find(|c| c.name == character_name)?;

    // 计算动画所需的最大行号
    let max_row = character_entry.calculate_max_animation_row();
    // 创建纹理图集布局
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(character_entry.tile_size),
        character_entry.atlas_columns as u32,
        (max_row + 1) as u32,
        None,
        None,
    ));

    // 加载角色纹理
    let texture = asset_server.load(&character_entry.texture_path);
    // 从图集中创建精灵
    let sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 });

    // 生成敌人实体，包含所有必需的组件
    let entity = commands
        .spawn((
            Enemy,                                                                      // 敌人标记组件
            sprite,                                                                     // 精灵组件
            Transform::from_translation(position).with_scale(Vec3::splat(ENEMY_SCALE)), // 变换组件
            GlobalTransform::default(),     // 全局变换组件
            AnimationController::default(), // 动画控制器
            CharacterState::default(),      // 角色状态
            Velocity::default(),            // 速度
            Facing::default(),              // 朝向
            Collider::default(),            // 碰撞体
            EnemyCombat::default(),         // 敌人战斗能力
            AIBehavior::default(),          // AI 行为配置
            EnemyPath::default(),           // 敌人寻路路径
            AnimationTimer(Timer::from_seconds(
                // 动画帧计时器
                DEFAULT_ANIMATION_FRAME_TIME,
                TimerMode::Repeating,
            )),
            character_entry.clone(), // 角色配置副本
        ))
        .id();

    // 记录敌人生成日志
    info!("Spawned enemy '{}' at {:?}", character_name, position);
    Some(entity)
}

/// 敌人生成状态资源
///
/// 用于追踪是否已经生成过敌人，避免重复生成
#[derive(Resource, Default, Eq, PartialEq)]
pub struct EnemiesSpawned(pub bool);

/// 获取有效的生成位置
///
/// 检查期望的生成位置是否可行走，如果不行则寻找最近的可行走位置。
///
/// # 参数
/// - `collision_map`: 碰撞地图
/// - `desired_pos`: 期望的生成位置
///
/// # 返回
/// 有效的生成位置（可能是期望位置，也可能是调整后的位置）
fn get_valid_spawn_position(collision_map: &CollisionMap, desired_pos: Vec2) -> Vec2 {
    // 敌人的碰撞半径
    let enemy_radius = 12.0;

    // 检查期望位置是否可行走（不会与障碍物碰撞）
    if collision_map.is_circle_clear(desired_pos, enemy_radius) {
        return desired_pos;
    }

    // 如果期望位置不可行走，查找最近的可行走位置
    let grid_pos = collision_map.world_to_grid(desired_pos);
    if let Some(walkable) = collision_map.find_nearest_walkable(grid_pos) {
        let world_pos = collision_map.grid_to_world(walkable.x, walkable.y);
        info!(
            "Adjusted enemy spawn from {:?} to {:?} (was on obstacle)",
            desired_pos, world_pos
        );
        return world_pos;
    }

    // 如果找不到可行走位置，使用期望位置（会卡在障碍物中）
    warn!(
        "Could not find walkable spawn position near {:?}",
        desired_pos
    );
    desired_pos
}

/// 生成测试敌人
///
/// 此函数用于开发和测试阶段，在预定义的位置生成敌人。
/// 每个位置都会检查是否可行走，如果不行则自动调整。
///
/// # 系统参数
/// - `commands`: 命令队列
/// - `asset_server`: 资源服务器
/// - `atlas_layouts`: 图集布局资源集合
/// - `characters_list`: 角色列表配置资源
/// - `characters_list_res`: 角色列表资源句柄
/// - `collision_map`: 碰撞地图
/// - `enemies_spawned`: 敌人生成状态资源
///
/// # 生成逻辑
/// - 只在第一次调用时生成敌人
/// - 在两个预设位置生成敌人：(200, 0) 和 (-200, 100)
/// - 使用 "graveyard_reaper" 角色配置
pub fn spawn_test_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    characters_list: Res<Assets<CharactersList>>,
    characters_list_res: Option<Res<CharactersListResource>>,
    collision_map: Option<Res<CollisionMap>>,
    mut enemies_spawned: ResMut<EnemiesSpawned>,
) {
    // 等待碰撞地图资源可用
    let Some(collision_map) = collision_map else {
        return;
    };

    // 等待角色列表资源句柄可用
    let Some(characters_list_res) = characters_list_res else {
        return;
    };

    // 等待角色列表配置加载完成
    let Some(characters_list) = characters_list.get(&characters_list_res.handle) else {
        return;
    };

    // 预定义的生成位置
    let spawn_positions = [Vec2::new(200.0, 0.0), Vec2::new(-200.0, 100.0)];

    // 在每个位置生成敌人
    for desired_pos in spawn_positions {
        // 获取有效的生成位置（可能因为障碍物而调整）
        let valid_pos = get_valid_spawn_position(&collision_map, desired_pos);

        // 生成敌人实体
        spawn_enemy(
            &mut commands,
            &asset_server,
            &mut atlas_layouts,
            characters_list,
            Vec3::new(valid_pos.x, valid_pos.y, ENEMY_Z_POSITION),
            "graveyard_reaper", // 使用墓地死神角色
        );
    }

    // 标记敌人已经生成
    enemies_spawned.0 = true;
    info!("Enemies spawned with validated positions");
}
