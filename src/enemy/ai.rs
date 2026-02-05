/// 敌人 AI 行为系统
///
/// 此模块实现敌人的智能行为逻辑，主要包括：
/// - 追踪玩家：检测玩家位置并自动追踪
/// - 寻路移动：使用 A* 算法避开障碍物
/// - 攻击范围判断：在适当距离停止并攻击玩家
/// - 状态管理：根据距离和位置切换待机、行走等状态
use crate::characters::config::CharacterEntry;
use crate::characters::facing::Facing;
use crate::characters::input::Player;
use crate::characters::physics::{Velocity, calculate_velocity};
use crate::characters::state::CharacterState;
use crate::collision::CollisionMap;
use crate::enemy::components::{AIBehavior, Enemy, EnemyPath};
use bevy::prelude::*;

/// 敌人追踪玩家系统的主函数
///
/// 此函数在每一帧被调用，更新所有敌人的 AI 行为。
/// 敌人会根据与玩家的距离采取不同行动：
/// 1. **超出检测范围**：进入待机状态，停止移动
/// 2. **在攻击范围内**：停止移动，朝向玩家准备攻击
/// 3. **在检测范围内但超出攻击范围**：使用寻路系统追踪玩家
///
/// # 系统参数
/// - `time`: 游戏时间资源，用于计算时间增量
/// - `collision_map`: 碰撞地图，用于寻路计算
/// - `enemy_query`: 敌人实体查询，包含所有需要更新 AI 的敌人
/// - `player_query`: 玩家实体查询，获取玩家当前位置
///
/// # 行为逻辑
/// - 敌人检测到玩家后，会定期重新计算路径以适应玩家移动
/// - 使用 A* 算法寻找避开障碍物的最优路径
/// - 到达路径点后自动移动到下一个路径点
/// - 路径不可达时直接朝向玩家移动
pub fn enemy_follow_player(
    time: Res<Time>,
    collision_map: Option<Res<CollisionMap>>,
    mut enemy_query: Query<
        (
            &Transform,
            &mut CharacterState,
            &mut Velocity,
            &mut Facing,
            &CharacterEntry,
            &AIBehavior,
            &mut EnemyPath,
        ),
        With<Enemy>,
    >,
    player_query: Query<&Transform, With<Player>>,
) {
    // 获取玩家的变换组件，如果玩家不存在则直接返回
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    // 获取碰撞地图资源，如果地图不存在则直接返回
    let Some(collision_map) = collision_map else {
        return;
    };

    // 获取玩家的世界坐标（只取 X 和 Y 分量，忽略 Z 轴）
    let player_pos = player_transform.translation.truncate();
    // 获取当前帧的时间增量（秒）
    let delta = time.delta_secs();

    // 遍历所有敌人实体，更新它们的 AI 行为
    for (enemy_transform, mut state, mut velocity, mut facing, character, ai, mut path) in
        enemy_query.iter_mut()
    {
        // 获取敌人的当前位置
        let enemy_pos = enemy_transform.translation.truncate();
        // 计算从敌人指向玩家的向量
        let to_player = player_pos - enemy_pos;
        // 计算敌人和玩家之间的距离
        let distance = to_player.length();

        // 判断玩家是否超出敌人的检测范围
        if distance > ai.detection_range {
            // 如果玩家在检测范围外，敌人进入待机状态
            if *state != CharacterState::Idle {
                *state = CharacterState::Idle;
            }
            // 停止移动
            *velocity = Velocity::ZERO;
            continue;
        }

        // 计算攻击阈值：如果当前是待机状态，增加一点缓冲距离
        // 这样可以避免敌人在边界附近反复切换状态
        let attack_threshold = if *state == CharacterState::Idle {
            ai.attack_range + 20.0 // 待机时需要更近一点才开始追踪
        } else {
            ai.attack_range
        };

        // 判断敌人是否在攻击范围内
        if distance <= attack_threshold {
            // 在攻击范围内，敌人进入待机状态准备攻击
            if *state != CharacterState::Idle {
                *state = CharacterState::Idle;
            }
            // 停止移动
            *velocity = Velocity::ZERO;

            // 更新朝向，让敌人面向玩家
            let direction = to_player.normalize_or_zero();
            if direction != Vec2::ZERO {
                let new_facing = Facing::from_velocity(direction);
                if new_facing != *facing {
                    *facing = new_facing;
                }
            }
            continue;
        }

        // 更新路径重新计算计时器
        path.recalc_timer -= delta;

        // 检查是否需要计算或更新路径
        if !path.has_path() {
            // 如果当前没有路径，尝试计算从敌人位置到玩家位置的路径
            if let Some(waypoints) = collision_map.find_path(enemy_pos, player_pos) {
                path.set_path(waypoints);
                // 重置路径重新计算计时器
                path.recalc_timer = EnemyPath::RECALC_INTERVAL;
            }
        } else if path.recalc_timer <= 0.0 {
            // 如果计时器到期，重新计算路径以适应玩家的移动
            path.recalc_timer = EnemyPath::RECALC_INTERVAL;
            if let Some(waypoints) = collision_map.find_path(enemy_pos, player_pos) {
                path.set_path(waypoints);
            }
        }

        // 检查是否有当前路径点
        if let Some(waypoint) = path.current_waypoint() {
            // 计算从敌人位置到当前路径点的向量和距离
            let to_waypoint = waypoint - enemy_pos;
            let waypoint_distance = to_waypoint.length();

            // 如果已经到达当前路径点（距离小于阈值），移动到下一个路径点
            if waypoint_distance <= EnemyPath::WAYPOINT_THRESHOLD {
                path.advance();
            }

            // 获取新的当前路径点（可能已经在上一步前进）
            if let Some(current_wp) = path.current_waypoint() {
                // 计算从敌人位置到当前路径点的方向向量
                let to_waypoint = current_wp - enemy_pos;
                let direction = to_waypoint.normalize_or_zero();

                // 更新敌人状态为行走
                if *state != CharacterState::Walking {
                    *state = CharacterState::Walking;
                }

                // 更新朝向，让敌人朝向当前路径点
                if direction != Vec2::ZERO {
                    let new_facing = Facing::from_velocity(direction);
                    if new_facing != *facing {
                        *facing = new_facing;
                    }
                }

                // 根据状态、方向和角色配置计算速度
                *velocity = calculate_velocity(*state, direction, character);
            }
        } else {
            // 如果没有路径（寻路失败），直接朝向玩家移动
            let direction = to_player.normalize_or_zero();

            // 更新敌人状态为行走
            if *state != CharacterState::Walking {
                *state = CharacterState::Walking;
            }

            // 更新朝向，让敌人朝向玩家
            if direction != Vec2::ZERO {
                let new_facing = Facing::from_velocity(direction);
                if new_facing != *facing {
                    *facing = new_facing;
                }
            }

            // 根据状态、方向和角色配置计算速度
            *velocity = calculate_velocity(*state, direction, character);
        }
    }
}
