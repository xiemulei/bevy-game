use crate::characters::config::CharactersList;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

mod animation;
mod config;
mod movement;
mod spawn;

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
                    // 初始化玩家角色的精灵和动画控制器
                    spawn::initialize_player_character,
                    // 切换角色（按数字键 1-9）
                    spawn::switch_character,
                    // 处理玩家移动
                    movement::move_player,
                    // 更新跳跃状态
                    movement::update_jump_state,
                    // 播放角色动画
                    animation::animate_characters,
                    // 更新动画状态标志
                    animation::update_animation_flags,
                ),
            );
    }
}
