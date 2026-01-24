use crate::characters::facing::Facing;
use crate::characters::input::Player;
use crate::combat::player_combat::PlayerCombat;
use crate::combat::power_type::{PowerType, PowerVisuals};
use crate::particles::components::ParticleEmitter;
use bevy::prelude::*;

/// 投射物效果组件
#[derive(Component)]
pub struct ProjectileEffect {
    #[allow(dead_code)]
    pub power_type: PowerType,
}

/// 处理技能输入系统 - 响应Ctrl键释放技能
pub fn handle_power_input(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&GlobalTransform, &Facing, &mut PlayerCombat), With<Player>>,
) {
    let Ok((global_transform, facing, mut combat)) = player_query.single_mut() else {
        return;
    };

    // 更新冷却时间
    combat.cooldown.tick(time.delta());

    // 检查是否按下了Ctrl键
    let ctrl_pressed =
        input.just_pressed(KeyCode::ControlLeft) || input.just_pressed(KeyCode::ControlRight);
    if !ctrl_pressed {
        return;
    }

    // 检查冷却时间是否结束
    if combat.cooldown.elapsed_secs() < combat.cooldown.duration().as_secs_f32() {
        return;
    }

    // 重置冷却时间
    combat.cooldown.reset();

    // 计算生成位置和方向
    let position: Vec3 = global_transform.translation();
    let direction = facing_to_vec3(facing);
    let spawn_position = position + direction * 5.0;

    // 获取视觉效果配置
    let visuals = combat.power_type.visual(direction);

    // 生成投射物
    spawn_projectile(&mut commands, spawn_position, combat.power_type, &visuals);

    info!("{:?} projectile fired!", combat.power_type);
}

/// 生成投射物和粒子效果
fn spawn_projectile(
    commands: &mut Commands,
    position: Vec3,
    power_type: PowerType,
    visuals: &PowerVisuals,
) {
    // 创建主粒子发射器
    let primary_emitter =
        ParticleEmitter::new(0.016, visuals.particles_per_spawn, visuals.primary.clone())
            .one_shot();

    commands.spawn((
        primary_emitter,
        Transform::from_translation(position),
        GlobalTransform::from(Transform::from_translation(position)),
        ProjectileEffect { power_type },
    ));

    // 创建核心粒子发射器（如果配置了核心粒子）
    if let Some(ref core_config) = visuals.core {
        let core_emitter =
            ParticleEmitter::new(0.016, visuals.core_particles_per_spawn, core_config.clone())
                .one_shot();

        commands.spawn((
            core_emitter,
            Transform::from_translation(position),
            GlobalTransform::from(Transform::from_translation(position)),
            ProjectileEffect { power_type },
        ));
    }
}

/// 将朝向转换为3D向量
fn facing_to_vec3(facing: &Facing) -> Vec3 {
    match facing {
        Facing::Right => Vec3::X,
        Facing::Left => Vec3::NEG_X,
        Facing::Up => Vec3::Y,
        Facing::Down => Vec3::NEG_Y,
    }
}

/// 调试系统 - 使用数字键切换能力类型
pub fn debug_switch_power(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut PlayerCombat, With<Player>>,
) {
    let Ok(mut combat) = player_query.single_mut() else {
        return;
    };

    // 根据数字键选择能力类型
    let new_power = if input.just_pressed(KeyCode::Digit1) {
        Some(PowerType::Fire)
    } else if input.just_pressed(KeyCode::Digit2) {
        Some(PowerType::Arcane)
    } else if input.just_pressed(KeyCode::Digit3) {
        Some(PowerType::Shadow)
    } else if input.just_pressed(KeyCode::Digit4) {
        Some(PowerType::Poison)
    } else {
        None
    };

    // 切换能力类型
    if let Some(power) = new_power {
        combat.power_type = power;
        info!("Switched to {:?} power!", power);
    }
}
