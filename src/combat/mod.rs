// 战斗模块 - 处理玩家攻击和技能释放
mod player_combat;
mod power_type;
mod systems;

pub use player_combat::PlayerCombat;
pub use systems::{debug_switch_power, handle_power_input};

use bevy::prelude::*;

/// 战斗插件 - 注册战斗相关系统
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        // 注册处理技能输入和调试切换能力的系统
        app.add_systems(Update, (handle_power_input, debug_switch_power));
    }
}
