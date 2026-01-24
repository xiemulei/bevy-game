use crate::combat::power_type::PowerType;
use bevy::prelude::*;

/// 玩家战斗组件 - 管理玩家的能力类型和冷却时间
#[derive(Component)]
pub struct PlayerCombat {
    /// 当前能力类型
    pub power_type: PowerType,
    /// 技能冷却计时器
    pub cooldown: Timer,
}

impl Default for PlayerCombat {
    fn default() -> Self {
        Self {
            power_type: PowerType::Fire,
            cooldown: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }
}

impl PlayerCombat {
    /// 创建新的玩家战斗组件
    #[allow(unused)]
    pub fn new(power_type: PowerType) -> Self {
        Self {
            power_type,
            cooldown: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }

    /// 设置冷却时间
    #[allow(unused)]
    pub fn with_cooldown(mut self, seconds: f32) -> Self {
        self.cooldown = Timer::from_seconds(seconds, TimerMode::Once);
        self
    }
}
