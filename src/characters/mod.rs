use crate::characters::config::CharactersList;
use crate::state::GameState;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

pub mod animation;
pub mod collider;
pub mod config;
pub mod facing;
pub mod input;
pub mod physics;
mod rendering;
pub mod spawn;
pub mod state;

/// 角色系统插件
///
/// 负责加载角色资源、处理角色生成、移动和动画
pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        // 添加 RON 资源加载插件，用于加载角色配置文件
        app.add_plugins(RonAssetPlugin::<CharactersList>::new(&["characters.ron"]))
            // 初始化当前角色索引资源
            .init_resource::<spawn::CurrentCharacterIndex>()
            // 在启动时生成玩家角色
            .add_systems(Startup, spawn::spawn_player)
            // 每帧更新系统
            .add_systems(
                Update,
                (
                    input::handle_player_input,
                    // 切换角色（按数字键 1-9）
                    spawn::switch_character,
                    input::update_jump_state,
                    animation::on_state_change_update_animation,
                    collider::validate_movement,
                    physics::apply_velocity,
                    rendering::update_player_depth,
                    animation::tick_animations,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
