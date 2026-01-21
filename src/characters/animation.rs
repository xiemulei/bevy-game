use crate::characters::config::{AnimationType, CharacterEntry};
use crate::characters::facing::Facing;
use crate::characters::state::CharacterState;
use bevy::prelude::*;

/// 默认动画帧时间（秒）
pub const DEFAULT_ANIMATION_FRAME_TIME: f32 = 0.1;

/// 动画控制器组件
///
/// 控制角色当前播放的动画类型和朝向
#[derive(Component, Default)]
pub struct AnimationController {
    /// 当前播放的动画类型
    pub current_animation: AnimationType,
}

impl AnimationController {
    /// 根据当前动画和朝向获取动画剪辑
    ///
    /// 动画剪辑定义了精灵表中应该播放哪些帧
    pub fn get_clip(&self, config: &CharacterEntry, facing: Facing) -> Option<AnimationClip> {
        let def = config.animations.get(&self.current_animation)?;

        // 计算动画在精灵表中的行号
        // 如果动画有方向变化，则根据朝向调整行号
        let row = if def.directional {
            def.start_row + facing.direction_index()
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

/// 根据角色状态变化更新动画
///
/// 当角色状态发生改变时，此系统会根据新的状态设置相应的动画类型，
/// 并重置动画计时器以确保动画从头开始播放。
///
/// # 参数
/// * `query` - 查询包含角色状态、动画控制器和动画计时器的实体，仅处理状态已改变的实体
///
/// # 组件说明
/// * `CharacterState` - 角色当前状态（空闲、行走、奔跑、跳跃）
/// * `AnimationController` - 动画控制器，管理当前播放的动画类型
/// * `AnimationTimer` - 动画计时器，控制动画播放进度
pub fn on_state_change_update_animation(
    mut query: Query<
        (
            &CharacterState,
            &mut AnimationController,
            &mut AnimationTimer,
        ),
        Changed<CharacterState>,
    >,
) {
    for (state, mut controller, mut timer) in query.iter_mut() {
        // 根据角色状态匹配对应的动画类型
        let new_animation = match state {
            CharacterState::Idle | CharacterState::Walking => AnimationType::Walk,
            CharacterState::Running => AnimationType::Run,
            CharacterState::Jumping => AnimationType::Jump,
        };

        // 如果动画类型发生变化，则更新动画并重置计时器
        if controller.current_animation != new_animation {
            controller.current_animation = new_animation;
            timer.0.reset();
        }
    }
}

/// 更新角色动画播放状态
///
/// 该系统负责根据角色当前状态、朝向和动画控制器来更新精灵图集索引，
/// 实现动画帧的切换和播放控制
///
/// # 参数
/// * `time` - 时间资源，用于获取帧间隔时间
/// * `query` - 查询包含角色动画相关组件的实体
///   - `CharacterState` - 角色状态组件
///   - `Facing` - 角色朝向组件  
///   - `AnimationController` - 动画控制器组件
///   - `AnimationTimer` - 动画计时器组件
///   - `Sprite` - 精灵组件（包含纹理图集）
///   - `CharacterEntry` - 角色配置条目组件
///
/// # 处理逻辑
/// - 对于空闲状态的角色，将其动画设置为起始帧并保持不动
/// - 对于非空闲状态的角色，根据计时器更新动画帧
/// - 自动处理动画循环和帧率调整
pub fn tick_animations(
    time: Res<Time>,
    mut query: Query<(
        &CharacterState,
        &Facing,
        &AnimationController,
        &mut AnimationTimer,
        &mut Sprite,
        &CharacterEntry,
    )>,
) {
    for (state, facing, controller, mut timer, mut sprite, config) in query.iter_mut() {
        // 处理空闲状态：将动画设置为起始帧并停止播放
        if *state == CharacterState::Idle {
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                if let Some(clip) = controller.get_clip(config, *facing) {
                    if atlas.index != clip.start() {
                        atlas.index = clip.start();
                    }
                }
            }
            continue;
        }

        // 获取必要的动画数据
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        let Some(clip) = controller.get_clip(config, *facing) else {
            continue;
        };
        let Some(anim_def) = config.animations.get(&controller.current_animation) else {
            continue;
        };

        // 检查当前帧索引是否在动画片段范围内，不在则重置到起始帧
        if !clip.contains(atlas.index) {
            atlas.index = clip.start();
            timer.0.reset();
        }

        // 同步动画帧持续时间与定义的时间设置
        let expected_duration = std::time::Duration::from_secs_f32(anim_def.frame_time);
        if timer.duration() != expected_duration {
            timer.set_duration(expected_duration);
        }

        // 更新计时器并处理帧切换
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = clip.next(atlas.index);
        }
    }
}
