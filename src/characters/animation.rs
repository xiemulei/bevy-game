use crate::characters::config::{AnimationType, CharacterEntry};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// 默认动画帧时间（秒）
pub const DEFAULT_ANIMATION_FRAME_TIME: f32 = 0.1;

/// 角色朝向枚举
///
/// 定义角色在 2D 平面上的四个朝向
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Facing {
    /// 上
    Up,
    /// 左
    Left,
    /// 下
    Down,
    /// 右
    Right,
}

impl Facing {
    /// 根据移动方向向量确定角色朝向
    ///
    /// 比较水平和垂直分量的大小，选择较大的一侧作为朝向
    pub fn from_direction(direction: Vec2) -> Self {
        if direction.x.abs() > direction.y.abs() {
            if direction.x > 0.0 {
                Facing::Right
            } else {
                Facing::Left
            }
        } else {
            if direction.y > 0.0 {
                Facing::Up
            } else {
                Facing::Down
            }
        }
    }

    /// 获取朝向对应的索引
    ///
    /// 上: 0, 左: 1, 下: 2, 右: 3
    pub fn direction_index(&self) -> usize {
        match self {
            Facing::Up => 0,
            Facing::Left => 1,
            Facing::Down => 2,
            Facing::Right => 3,
        }
    }
}

/// 动画控制器组件
///
/// 控制角色当前播放的动画类型和朝向
#[derive(Component)]
pub struct AnimationController {
    /// 当前播放的动画类型
    pub current_animation: AnimationType,
    /// 角色当前朝向
    pub facing: Facing,
}

impl Default for AnimationController {
    fn default() -> Self {
        Self {
            current_animation: AnimationType::Walk,
            facing: Facing::Down,
        }
    }
}

impl AnimationController {
    /// 根据当前动画和朝向获取动画剪辑
    ///
    /// 动画剪辑定义了精灵表中应该播放哪些帧
    pub fn get_clip(&self, config: &CharacterEntry) -> Option<AnimationClip> {
        let def = config.animations.get(&self.current_animation)?;

        // 计算动画在精灵表中的行号
        // 如果动画有方向变化，则根据朝向调整行号
        let row = if def.directional {
            def.start_row + self.facing.direction_index()
        } else {
            def.start_row
        };

        Some(AnimationClip::new(
            row,
            def.frame_count,
            config.atlas_columns,
        ))
    }
}

/// 动画状态组件
///
/// 追踪角色的运动和跳跃状态，用于触发适当的动画
#[derive(Component, Default)]
pub struct AnimationState {
    /// 角色是否正在移动
    pub is_moving: bool,
    /// 上一帧角色是否正在移动
    pub was_moving: bool,
    /// 角色是否正在跳跃
    pub is_jumping: bool,
    /// 上一帧角色是否正在跳跃
    pub was_jumping: bool,
}

/// 动画计时器组件
///
/// 控制动画帧切换的时间间隔
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/// 动画剪辑结构体
///
/// 定义动画的帧范围（从起始帧到结束帧）
#[derive(Copy, Clone)]
pub struct AnimationClip {
    /// 起始帧索引
    first: usize,
    /// 结束帧索引
    last: usize,
}

impl AnimationClip {
    /// 创建新的动画剪辑
    ///
    /// # 参数
    /// - `row`: 动画在精灵表中的起始行
    /// - `frame_count`: 动画帧的总数
    /// - `atlas_columns`: 精灵表的列数
    pub fn new(row: usize, frame_count: usize, atlas_columns: usize) -> Self {
        let first = row * atlas_columns;
        Self {
            first,
            last: first + frame_count - 1,
        }
    }

    /// 获取动画的起始帧索引
    pub fn start(&self) -> usize {
        self.first
    }

    /// 检查给定帧索引是否在动画范围内
    pub fn contains(&self, index: usize) -> bool {
        (self.first..=self.last).contains(&index)
    }

    /// 获取下一帧的索引
    ///
    /// 如果是最后一帧，则循环回到起始帧
    pub fn next(&self, index: usize) -> usize {
        if index == self.last {
            self.first
        } else {
            index + 1
        }
    }

    /// 检查动画是否已完成
    ///
    /// 当前帧为最后一帧且计时器已完成时返回 true
    pub fn is_complete(&self, current_index: usize, timer_finished: bool) -> bool {
        current_index >= self.last && timer_finished
    }
}

/// 角色动画系统
///
/// 根据角色的运动状态和动画控制器更新精灵显示的帧
pub fn animate_characters(
    time: Res<Time>,
    mut query: Query<(
        &AnimationController,
        &AnimationState,
        &mut AnimationTimer,
        &mut Sprite,
        &CharacterEntry,
    )>,
) {
    for (animated, state, mut timer, mut sprite, config) in query.iter_mut() {
        // 获取精灵的纹理图集
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        // 获取当前动画的剪辑
        let Some(clip) = animated.get_clip(config) else {
            continue;
        };
        // 获取当前动画的定义
        let Some(anim_def) = config.animations.get(&animated.current_animation) else {
            continue;
        };

        // 如果当前帧不在动画范围内，重置到起始帧
        if !clip.contains(atlas.index) {
            atlas.index = clip.start();
            timer.reset();
        }

        // 检测状态变化
        let just_started_moving = state.is_moving && !state.was_moving;
        let just_stopped_moving = !state.is_moving && state.was_moving;
        let just_started_jumping = state.is_jumping && !state.was_jumping;
        let just_stopped_jumping = !state.is_jumping && state.was_jumping;

        // 确定是否需要播放动画
        let should_animate = state.is_jumping || state.is_moving;
        // 检测动画是否改变
        let animation_changed = just_started_moving
            || just_started_jumping
            || just_stopped_moving
            || just_stopped_jumping;

        if animation_changed {
            // 动画改变时，重置到起始帧
            atlas.index = clip.start();
            timer.set_duration(Duration::from_secs_f32(anim_def.frame_time));
            timer.reset();
        } else if should_animate {
            // 播放动画时，更新计时器和帧索引
            timer.tick(time.delta());
            if timer.just_finished() {
                atlas.index = clip.next(atlas.index);
            }
        } else {
            // 不播放动画时，显示起始帧
            if atlas.index != clip.start() {
                atlas.index = clip.start();
            }
        }
    }
}

/// 更新动画状态标志系统
///
/// 在每帧结束时，将当前状态保存为"上一帧"状态
pub fn update_animation_flags(mut query: Query<&mut AnimationState>) {
    for mut state in query.iter_mut() {
        state.was_moving = state.is_moving;
        state.was_jumping = state.is_jumping;
    }
}
