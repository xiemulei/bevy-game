use crate::characters::animation::{AnimationController, AnimationTimer};
use crate::characters::config::CharacterEntry;
use crate::characters::facing::Facing;
use crate::characters::physics::Velocity;
use crate::characters::state::CharacterState;
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

/// 根据当前状态和输入确定角色的新状态
///
/// # 参数
/// * `current` - 角色当前的状态
/// * `direction` - 移动方向向量
/// * `is_running` - 是否处于跑步状态
/// * `wants_jump` - 是否想要跳跃
///
/// # 返回值
/// 返回根据当前状态和输入条件计算出的新角色状态
fn determine_new_state(
    current: CharacterState,
    direction: Vec2,
    is_running: bool,
    wants_jump: bool,
) -> CharacterState {
    match current {
        // 跳跃状态只能等跳跃动作结束
        CharacterState::Jumping => CharacterState::Jumping,
        // 在地面，想要跳跃时进入跳跃状态
        _ if wants_jump && current.is_grounded() => CharacterState::Jumping,
        // 有移动方向时根据是否跑步决定行走或跑步状态
        _ if direction != Vec2::ZERO => {
            if is_running {
                CharacterState::Running
            } else {
                CharacterState::Walking
            }
        }
        // 其他情况设为空闲状态
        _ => CharacterState::Idle,
    }
}

pub fn handle_player_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut CharacterState,
            &mut Velocity,
            &mut Facing,
            &CharacterEntry,
        ),
        With<Player>,
    >,
) {
    let Ok((mut state, mut velocity, mut facing, character)) = query.single_mut() else {
        return;
    };

    // 读取用户输入
    let direction = read_movement_input(&input);
    let is_running = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);
    let wants_jump = input.just_pressed(KeyCode::Space);

    // 更新用户朝向
    if direction != Vec2::ZERO {
        let new_facing = Facing::from_velocity(direction);
        if *facing != new_facing {
            *facing = new_facing;
        }
    }

    // 根据输入更新用户状态
    let new_state = determine_new_state(*state, direction, is_running, wants_jump);
    if *state != new_state {
        *state = new_state; // 触发 Changed<CharacterState>
    }

    // 根据用户状态计算速度向量
    *velocity = super::physics::calculate_velocity(*state, direction, character);
}

pub fn update_jump_state(
    mut query: Query<
        (
            &mut CharacterState,
            &Facing,
            &AnimationController,
            &AnimationTimer,
            &Sprite,
            &CharacterEntry,
        ),
        With<Player>,
    >,
) {
    let Ok((mut state, facing, controller, timer, sprite, config)) = query.single_mut() else {
        return;
    };

    if *state != CharacterState::Jumping {
        return;
    }

    let Some(atlas) = sprite.texture_atlas.as_ref() else {
        return;
    };
    let Some(clip) = controller.get_clip(config, *facing) else {
        return;
    };

    if clip.is_complete(atlas.index, timer.just_finished()) {
        *state = CharacterState::Idle;
    }
}
