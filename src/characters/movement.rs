use crate::characters::animation::{AnimationController, AnimationState, AnimationTimer, Facing};
use crate::characters::config::{AnimationType, CharacterEntry};
use bevy::input::ButtonInput;
use bevy::prelude::*;

/// 玩家标记组件
///
/// 用于标识玩家控制的实体
#[derive(Component)]
pub struct Player;

/// 读取移动输入
///
/// 检查方向键是否被按下，并返回移动方向向量
fn read_movement_input(input: &ButtonInput<KeyCode>) -> Vec2 {
    // 方向键到移动方向的映射
    const MOVEMENT_KEYS: [(KeyCode, Vec2); 4] = [
        (KeyCode::ArrowLeft, Vec2::NEG_X),
        (KeyCode::ArrowRight, Vec2::X),
        (KeyCode::ArrowUp, Vec2::Y),
        (KeyCode::ArrowDown, Vec2::NEG_Y),
    ];

    // 将所有按下的方向键对应的向量相加
    MOVEMENT_KEYS
        .iter()
        .filter(|(key, _)| input.pressed(*key))
        .map(|(_, direction)| *direction)
        .sum()
}

/// 计算移动速度
///
/// 根据角色配置和是否奔跑计算实际移动速度
fn calculate_movement_speed(character: &CharacterEntry, is_running: bool) -> f32 {
    if is_running {
        character.base_move_speed * character.run_speed_multiplier
    } else {
        character.base_move_speed
    }
}

/// 玩家移动系统
///
/// 处理玩家的移动、奔跑和跳跃输入，更新角色位置和动画状态
pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &mut AnimationController,
            &mut AnimationState,
            &CharacterEntry,
        ),
        With<Player>,
    >,
) {
    // 获取玩家实体组件
    let Ok((mut transform, mut animated, mut state, character)) = query.single_mut() else {
        return;
    };
    // 读取移动输入
    let direction = read_movement_input(&input);

    // 按空格键跳跃
    if input.just_pressed(KeyCode::Space) {
        state.is_jumping = true;
        animated.current_animation = AnimationType::Jump;
    }

    // 按左或右 Shift 键奔跑
    let is_running = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);

    if direction != Vec2::ZERO {
        // 计算移动速度
        let move_speed = calculate_movement_speed(&character, is_running);
        // 计算位移量（方向归一化 × 速度 × 时间增量）
        let delta = direction.normalize() * move_speed * time.delta_secs();
        // 更新角色位置
        transform.translation += delta.extend(0.0);

        // 更新角色朝向
        animated.facing = Facing::from_direction(direction);

        // 如果没有在跳跃，更新移动状态
        if !state.is_jumping {
            state.is_moving = true;
            // 根据是否奔跑设置动画类型
            animated.current_animation = if is_running {
                AnimationType::Run
            } else {
                AnimationType::Walk
            };
        }
    } else if !state.is_jumping {
        // 停止移动时
        state.is_moving = false;
        animated.current_animation = AnimationType::Walk;
    }
}

/// 更新跳跃状态系统
///
/// 检测跳跃动画是否完成，完成后切换回行走动画
pub fn update_jump_state(
    mut query: Query<
        (
            &mut AnimationController,
            &mut AnimationState,
            &AnimationTimer,
            &Sprite,
            &CharacterEntry,
        ),
        With<Player>,
    >,
) {
    for (mut animated, mut state, timer, sprite, config) in query.iter_mut() {
        // 只处理正在跳跃的角色
        if !state.is_jumping {
            continue;
        }

        // 获取精灵的纹理图集
        let Some(atlas) = sprite.texture_atlas.as_ref() else {
            continue;
        };

        // 获取当前动画的剪辑
        let Some(clip) = animated.get_clip(config) else {
            continue;
        };

        // 检查跳跃动画是否已完成
        if clip.is_complete(atlas.index, timer.just_finished()) {
            state.is_jumping = false;
            animated.current_animation = AnimationType::Walk;
        }
    }
}
