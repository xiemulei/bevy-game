/// 敌人系统模块
///
/// 此模块实现完整的敌人系统，包括：
/// - AI 行为（追踪玩家、寻路移动）
/// - 战斗能力（攻击、发射投射物）
/// - 组件定义（AI 配置、战斗能力、寻路路径）
/// - 敌人生成（智能位置选择、批量生成）
///
/// 模块结构：
/// - `ai`: 敌人 AI 行为系统
/// - `combat`: 敌人战斗系统
/// - `components`: 敌人组件定义
/// - `spawn`: 敌人生成系统
pub mod ai;
pub mod combat;
pub mod components;
pub mod spawn;

use crate::collision::CollisionMapBuilt;
use crate::enemy::spawn::EnemiesSpawned;
use crate::state::GameState;
use bevy::prelude::*;

/// 敌人系统插件
///
/// 注册所有敌人相关的系统、资源和组件到 Bevy 应用中。
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // 初始化敌人生成状态资源
            .init_resource::<EnemiesSpawned>()
            // 注册敌人生成系统
            // 运行条件：
            // 1. 碰撞地图已构建完成
            // 2. 敌人尚未生成
            // 3. 游戏处于 Playing 状态
            .add_systems(
                Update,
                spawn::spawn_test_enemies
                    .run_if(resource_equals(CollisionMapBuilt(true)))
                    .run_if(resource_equals(EnemiesSpawned(false)))
                    .run_if(in_state(GameState::Playing)),
            )
            // 注册敌人 AI 和战斗系统
            // 使用 .chain() 确保执行顺序：先 AI 更新，后战斗攻击
            // 运行条件：游戏处于 Playing 状态
            .add_systems(
                Update,
                (ai::enemy_follow_player, combat::enemy_attack)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
