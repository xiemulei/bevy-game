/// 敌人战斗系统
///
/// 此模块实现敌人的攻击行为，包括：
/// - 自动检测玩家是否在攻击范围内
/// - 攻击冷却管理
/// - 发射投射物攻击玩家
use crate::characters::input::Player;
use crate::combat::spawn_projectile;
use crate::enemy::components::{AIBehavior, Enemy, EnemyCombat};
use bevy::prelude::*;

/// 敌人攻击系统函数
///
/// 此函数在每一帧被调用，更新所有敌人的攻击行为。
/// 敌人会检查玩家是否在攻击范围内，并在冷却时间结束后发射投射物。
///
/// # 系统参数
/// - `commands`: 命令队列，用于生成投射物实体
/// - `time`: 游戏时间资源，用于更新冷却计时器
/// - `enemy_query`: 敌人实体查询，包含敌人的位置、战斗组件和 AI 行为
/// - `player_query`: 玩家实体查询，获取玩家当前位置
///
/// # 攻击逻辑
/// 1. 更新所有敌人的攻击冷却计时器
/// 2. 检查玩家是否在攻击范围内
/// 3. 如果冷却完成且玩家在范围内，发射投射物
/// 4. 重置冷却计时器
pub fn enemy_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<(&GlobalTransform, &mut EnemyCombat, &AIBehavior), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    // 获取玩家的变换组件，如果玩家不存在则直接返回
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    // 遍历所有敌人实体，更新它们的攻击行为
    for (enemy_transform, mut combat, ai) in enemy_query.iter_mut() {
        // 更新攻击冷却计时器
        combat.cooldown.tick(time.delta());

        // 获取敌人和玩家的世界位置
        let enemy_pos = enemy_transform.translation();
        let player_pos = player_transform.translation;

        // 计算敌人和玩家之间的距离
        let distance = enemy_pos.distance(player_pos);

        // 判断是否可以攻击：
        // 1. 玩家在攻击范围内
        // 2. 攻击冷却时间已结束
        if distance <= ai.attack_range && combat.cooldown.elapsed() >= combat.cooldown.duration() {
            // 计算从敌人指向玩家的单位向量
            let to_player = (player_pos - enemy_pos).normalize();
            // 计算投射物生成位置：在敌人前方 5 像素处
            let spawn_position = enemy_pos + to_player * 5.0;

            // 根据攻击类型生成对应的视觉效果
            let visuals = combat.power_type.visuals(to_player);

            // 生成投射物实体
            spawn_projectile(&mut commands, spawn_position, combat.power_type, &visuals);

            // 重置攻击冷却计时器
            combat.cooldown.reset();

            // 记录攻击日志
            info!("Enemy fired {:?} projectile at player", combat.power_type);
        }
    }
}
