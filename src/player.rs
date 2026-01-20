/// 玩家模块
///
/// 这个模块实现了一个完整的2D玩家角色系统，包含以下功能：
/// - 玩家实体的生成和初始化
/// - 基于键盘输入的8方向移动控制
/// - 基于精灵表的行走动画系统
/// - 朝向管理和状态切换
///
/// 使用方向键控制玩家移动：
/// - ↑↓←→ : 对应上下左右移动
/// - 支持同时按下多个方向键进行斜向移动
///
/// 动画系统特点：
/// - 每个方向有9帧行走动画
/// - 移动时自动播放动画，停止时恢复静止状态
/// - 朝向变化时立即切换到对应方向的动画帧
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::*;

// 图集相关常量
const TILE_SIZE: u32 = 64; // 每个精灵块的大小为 64x64 像素
const WALK_FRAMES: usize = 9; // 每个行走方向有 9 帧动画
const MOVE_SPEED: f32 = 140.0; // 移动速度（像素/秒）
const ANIM_DT: f32 = 0.1; // 动画帧间隔时间（秒，约 10 FPS）
const PLAYER_Z: f32 = 20.0;

/// 玩家实体组件标记
#[derive(Component)]
struct Player;

/// 玩家朝向枚举
/// 定义玩家可以面向的四个方向
#[derive(Component, Debug, Copy, Clone, Eq, PartialEq)]
enum Facing {
    UP,    // 向上
    LEFT,  // 向左
    DOWN,  // 向下
    RIGHT, // 向右
}

/// 动画计时器组件
/// 用于控制动画帧的切换时机
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

/// 动画状态组件
/// 存储玩家当前的动画状态信息
#[derive(Component)]
struct AnimationState {
    facing: Facing,   // 当前朝向
    moving: bool,     // 是否正在移动
    was_moving: bool, // 上一帧是否正在移动
}

/// 玩家插件
/// 负责注册玩家相关的系统和组件
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player) // 启动时生成玩家
            .add_systems(Update, (move_player, animate_player)); // 每帧更新：移动和动画
    }
}

/// 生成玩家实体
/// 在游戏启动时创建玩家角色，包括精灵、变换、动画状态等组件
fn spawn_player(
    mut commands: Commands,                               // 实体命令队列
    asset_server: Res<AssetServer>,                       // 资源服务器，用于加载纹理
    mut atlas_layout: ResMut<Assets<TextureAtlasLayout>>, // 纹理图集布局资源
) {
    // 加载男性精灵表纹理
    let texture = asset_server.load("male_spritesheet.png");

    // 创建纹理图集布局：网格布局，每块64x64，9列12行
    let layout = atlas_layout.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE), // 每个精灵块的大小
        WALK_FRAMES as u32,      // 列数（9列）
        12,                      // 行数（12行）
        None,                    // 无间距
        None,                    // 无偏移
    ));

    // 设置初始朝向为向下
    let facing = Facing::DOWN;
    // 计算初始精灵在图集中的索引
    let start_index = atlas_index_for(facing, 0);

    // 生成玩家实体，包含所有必要的组件
    commands.spawn((
        // 精灵组件：使用图集纹理
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: start_index,
            },
        ),
        // 变换组件：设置初始位置为原点
        Transform::from_translation(Vec3::new(0., 0., PLAYER_Z)).with_scale(Vec3::splat(0.8)),
        // 玩家标记组件
        Player,
        // 动画状态组件
        AnimationState {
            facing,
            moving: false,
            was_moving: false,
        },
        // 动画计时器组件
        AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
    ));
}

/// 处理玩家移动
/// 根据键盘输入更新玩家位置和朝向
fn move_player(
    input: Res<ButtonInput<KeyCode>>, // 键盘输入状态
    time: Res<Time>,                  // 时间资源
    mut player: Query<(&mut Transform, &mut AnimationState), With<Player>>, // 玩家实体的变换和动画状态
) {
    // 获取玩家实体的可变引用，如果失败则返回
    let Ok((mut transform, mut anim)) = player.single_mut() else {
        return;
    };

    // 初始化移动方向向量
    let mut direction = Vec2::ZERO;

    // 检测方向键输入，累加移动方向
    if input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0; // 向左移动
    }
    if input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0; // 向右移动
    }
    if input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0; // 向上移动
    }
    if input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0; // 向下移动
    }

    // 如果有移动输入
    if direction != Vec2::ZERO {
        // 计算本帧移动距离：标准化方向向量 × 速度 × 时间间隔
        let delta = direction.normalize() * MOVE_SPEED * time.delta_secs();

        // 更新玩家位置
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;

        // 设置移动状态为true
        anim.moving = true;

        // 根据移动方向更新朝向：优先选择轴方向上移动较大的方向
        if direction.x.abs() > direction.y.abs() {
            // 水平移动占主导
            anim.facing = if direction.x > 0.0 {
                Facing::RIGHT // 向右
            } else {
                Facing::LEFT // 向左
            };
        } else {
            // 垂直移动占主导
            anim.facing = if direction.y > 0.0 {
                Facing::UP // 向上
            } else {
                Facing::DOWN // 向下
            };
        }
    } else {
        // 没有移动输入，设置移动状态为false
        anim.moving = false;
    }
}

/// 处理玩家动画
/// 根据玩家移动状态和朝向更新精灵动画帧
fn animate_player(
    time: Res<Time>, // 时间资源
    mut query: Query<(&mut AnimationState, &mut AnimationTimer, &mut Sprite), With<Player>>, // 玩家动画相关组件
) {
    // 获取玩家实体的动画相关组件
    let Ok((mut anim, mut timer, mut sprite)) = query.single_mut() else {
        return;
    };

    // 获取精灵的纹理图集引用
    let atlas = match sprite.texture_atlas.as_mut() {
        Some(a) => a,
        None => return,
    };

    // 检测移动状态变化
    let just_started = anim.moving && !anim.was_moving; // 刚开始移动
    let just_stopped = !anim.moving && anim.was_moving; // 刚停止移动

    // 处理停止移动
    if just_stopped {
        timer.reset();
    }

    // 预计算当前朝向的起始索引，避免重复计算
    let row_start = row_start_index(anim.facing);
    let target_row = row_zero_based(anim.facing);
    let current_row = atlas.index / WALK_FRAMES;

    // 如果朝向发生变化，切换到新朝向的第一帧
    if current_row != target_row {
        atlas.index = row_start;
        timer.reset();
    }

    // 处理移动中的动画
    if anim.moving {
        // 计算下一帧的列索引
        let current_col = atlas.index % WALK_FRAMES;
        let next_col = (current_col + 1) % WALK_FRAMES;

        if just_started {
            // 刚开始移动：立即切换到下一帧
            atlas.index = row_start + next_col;
        } else {
            // 持续移动：根据计时器更新帧
            timer.tick(time.delta());
            if timer.just_finished() {
                atlas.index = row_start + next_col;
            }
        }
    }

    // 更新上一帧的移动状态
    anim.was_moving = anim.moving;
}

/// 获取指定朝向在图集中的起始索引
/// 返回该朝向第一帧在整个图集中的索引位置
fn row_start_index(facing: Facing) -> usize {
    row_zero_based(facing) * WALK_FRAMES
}

/// 计算指定朝向和帧数在图集中的索引
/// 返回该朝向特定帧在整个图集中的索引位置
fn atlas_index_for(facing: Facing, frame_in_row: usize) -> usize {
    row_start_index(facing) + frame_in_row.min(WALK_FRAMES - 1)
}

/// 获取朝向对应的行号（从0开始）
/// 根据精灵表的布局，每个朝向对应特定的行
fn row_zero_based(facing: Facing) -> usize {
    match facing {
        Facing::UP => 8,     // 向上：第8行
        Facing::LEFT => 9,   // 向左：第9行
        Facing::DOWN => 10,  // 向下：第10行
        Facing::RIGHT => 11, // 向右：第11行
    }
}
