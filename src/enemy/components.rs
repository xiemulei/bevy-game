/// 敌人组件定义
///
/// 此模块定义了敌人系统所需的所有组件，包括：
/// - `Enemy`: 敌人标记组件
/// - `EnemyCombat`: 敌人战斗能力组件
/// - `AIBehavior`: 敌人 AI 行为配置组件
/// - `EnemyPath`: 敌人寻路路径组件
use crate::combat::PowerType;
use bevy::prelude::*;

/// 敌人标记组件
///
/// 用于识别实体的敌人身份，在系统查询中区分敌人和玩家
#[derive(Component)]
pub struct Enemy;

/// 敌人战斗能力组件
///
/// 定义敌人的攻击能力和攻击冷却时间
#[derive(Component)]
pub struct EnemyCombat {
    /// 攻击能力类型（决定投射物的属性和效果）
    pub power_type: PowerType,
    /// 攻击冷却计时器
    pub cooldown: Timer,
}

impl Default for EnemyCombat {
    fn default() -> Self {
        Self {
            // 默认使用暗影攻击
            power_type: PowerType::Shadow,
            // 默认攻击冷却时间为 2 秒
            cooldown: Timer::from_seconds(2.0, TimerMode::Once),
        }
    }
}

impl EnemyCombat {
    /// 创建新的敌人战斗能力组件
    ///
    /// # 参数
    /// - `power_type`: 攻击能力类型
    /// - `cooldown_seconds`: 攻击冷却时间（秒）
    #[allow(unused)]
    pub fn new(power_type: PowerType, cooldown_seconds: f32) -> Self {
        Self {
            power_type,
            cooldown: Timer::from_seconds(cooldown_seconds, TimerMode::Once),
        }
    }
}

/// 敌人 AI 行为配置组件
///
/// 定义敌人的检测范围和攻击范围
#[derive(Component)]
pub struct AIBehavior {
    /// 攻击范围：敌人在这个距离内可以攻击玩家
    pub attack_range: f32,
    /// 检测范围：敌人在这个距离内会开始追踪玩家
    pub detection_range: f32,
}

impl Default for AIBehavior {
    fn default() -> Self {
        Self {
            // 默认攻击范围：150 像素
            attack_range: 150.0,
            // 默认检测范围：500 像素
            detection_range: 500.0,
        }
    }
}

impl AIBehavior {
    /// 创建新的 AI 行为配置
    ///
    /// # 参数
    /// - `attack_range`: 攻击范围
    /// - `detection_range`: 检测范围
    #[allow(unused)]
    pub fn new(attack_range: f32, detection_range: f32) -> Self {
        Self {
            attack_range,
            detection_range,
        }
    }
}

/// 敌人寻路路径组件
///
/// 存储敌人追踪玩家时的路径点和相关状态
#[derive(Component, Default)]
pub struct EnemyPath {
    /// 路径点列表（世界坐标）
    pub waypoints: Vec<Vec2>,
    /// 当前路径点的索引
    pub current_index: usize,
    /// 路径重新计算计时器
    pub recalc_timer: f32,
}

impl EnemyPath {
    /// 到达路径点的距离阈值
    ///
    /// 当敌人距离当前路径点小于此值时，认为已经到达，移动到下一个路径点
    pub const WAYPOINT_THRESHOLD: f32 = 16.0;
    /// 路径重新计算间隔（秒）
    ///
    /// 每隔这个时间重新计算一次路径，以适应玩家的移动
    pub const RECALC_INTERVAL: f32 = 0.5;

    /// 获取当前路径点
    ///
    /// # 返回
    /// - `Some(Vec2)`: 当前路径点的世界坐标
    /// - `None`: 没有更多路径点
    pub fn current_waypoint(&self) -> Option<Vec2> {
        self.waypoints.get(self.current_index).copied()
    }

    /// 前进到下一个路径点
    ///
    /// # 返回
    /// - `true`: 已经到达路径终点
    /// - `false`: 还有更多路径点
    pub fn advance(&mut self) -> bool {
        self.current_index += 1;
        self.current_index >= self.waypoints.len()
    }

    /// 设置新的路径
    ///
    /// 此函数会智能处理路径更新：
    /// - 跳过路径的第一个点（通常是敌人当前位置）
    /// - 如果新路径的第一个点距离当前目标很近，则不更新路径（避免抖动）
    ///
    /// # 参数
    /// - `waypoints`: 新的路径点列表
    pub fn set_path(&mut self, waypoints: Vec<Vec2>) {
        // 跳过第一个点（通常是敌人当前位置），从第二个点开始
        let new_waypoints = if waypoints.len() > 1 {
            waypoints[1..].to_vec()
        } else {
            waypoints
        };

        // 如果当前有目标路径点，检查新路径的第一个点是否距离太近
        // 如果太近就不更新，避免敌人在接近目标时频繁更新路径导致抖动
        if let Some(current_target) = self.current_waypoint() {
            if let Some(new_first) = new_waypoints.first() {
                if current_target.distance(*new_first) < Self::WAYPOINT_THRESHOLD * 1.5 {
                    return;
                }
            }
        }

        // 更新路径并重置索引
        self.waypoints = new_waypoints;
        self.current_index = 0;
    }

    /// 检查是否有有效路径
    ///
    /// # 返回
    /// - `true`: 有可用的路径点
    /// - `false`: 路径为空或已经到达终点
    pub fn has_path(&self) -> bool {
        !self.waypoints.is_empty() && self.current_index < self.waypoints.len()
    }
}
