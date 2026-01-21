use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 动画类型枚举
///
/// 定义角色可以执行的不同动画类型
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Default)]
pub enum AnimationType {
    /// 行走动画
    #[default]
    Walk,
    /// 奔跑动画
    Run,
    /// 跳跃动画
    Jump,
}

/// 动画定义结构体
///
/// 描述单个动画的配置参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationDefinition {
    /// 动画在精灵表中的起始行号
    pub start_row: usize,
    /// 动画帧的总数
    pub frame_count: usize,
    /// 每帧的显示时间（秒）
    pub frame_time: f32,
    /// 是否有方向变化（上下左右）
    pub directional: bool,
}

/// 角色条目结构体
///
/// 定义单个角色的所有属性和动画配置
///
/// # 字段说明
/// - `name`: 角色名称
/// - `max_health`: 最大生命值
/// - `base_move_speed`: 基础移动速度
/// - `run_speed_multiplier`: 奔跑速度倍率
/// - `texture_path`: 精灵表纹理路径
/// - `tile_size`: 单个图块的大小（像素）
/// - `atlas_columns`: 精灵表的列数
/// - `animations`: 动画类型到动画定义的映射
#[derive(Component, Asset, TypePath, Debug, Clone, Serialize, Deserialize)]
pub struct CharacterEntry {
    pub name: String,
    pub max_health: f32,
    pub base_move_speed: f32,
    pub run_speed_multiplier: f32,
    pub texture_path: String,
    pub tile_size: u32,
    pub atlas_columns: usize,
    pub animations: HashMap<AnimationType, AnimationDefinition>,
}

impl CharacterEntry {
    /// 计算动画所需的最大行号
    ///
    /// 用于确定精灵表的高度需求
    /// 如果动画有方向变化，则额外增加 3 行（因为每个方向占一行）
    pub fn calculate_max_animation_row(&self) -> usize {
        self.animations
            .values()
            .map(|def| {
                if def.directional {
                    def.start_row + 3
                } else {
                    def.start_row
                }
            })
            .max()
            .unwrap_or(0)
    }
}

/// 角色列表资源
///
/// 包含游戏中所有可用的角色配置
#[derive(Asset, TypePath, Debug, Clone, Serialize, Deserialize)]
pub struct CharactersList {
    /// 角色配置列表
    pub characters: Vec<CharacterEntry>,
}
