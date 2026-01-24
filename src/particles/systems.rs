use crate::particles::components::{EmissionShape, Particle, ParticleConfig, ParticleEmitter};
use crate::particles::material::ParticleMaterial;
use bevy::prelude::*;
use rand::Rng;

/// 更新粒子发射器系统 - 生成新粒子
pub fn update_emitters(
    mut commands: Commands,
    time: Res<Time>,
    mut emitters: Query<(Entity, &mut ParticleEmitter, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParticleMaterial>>,
) {
    let mut rng = rand::rng();

    for (entity, mut emitter, global_transform) in emitters.iter_mut() {
        // 跳过非激活的发射器
        if !emitter.active {
            continue;
        }

        // 跳过已完成的一次性发射器
        if emitter.one_shot && emitter.has_spawned {
            emitter.active = false;
            continue;
        }

        // 更新生成计时器
        emitter.spawn_timer.tick(time.delta());

        // 当计时器完成时生成粒子
        if emitter.spawn_timer.just_finished() {
            emitter.has_spawned = true;

            // 生成指定数量的粒子
            for i in 0..emitter.particles_per_spawn {
                spawn_particle(
                    &mut commands,
                    &emitter.particle_config,
                    global_transform,
                    &mut rng,
                    &mut meshes,
                    &mut materials,
                    Some(entity),
                    i,
                );
            }

            // 如果是一次性发射，则停用发射器
            if emitter.one_shot {
                emitter.active = false;
            }
        }
    }
}

/// 生成单个粒子
pub fn spawn_particle(
    commands: &mut Commands,
    config: &ParticleConfig,
    global_transform: &GlobalTransform,
    rng: &mut impl Rng,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ParticleMaterial>>,
    _owner: Option<Entity>,
    _particle_index: u32,
) {
    // 计算随机属性值
    let lifetime =
        config.lifetime + rng.random_range(-config.lifetime_variance..config.lifetime_variance);
    let speed = config.speed + rng.random_range(-config.speed_variance..config.speed_variance);
    let scale = config.scale + rng.random_range(-config.scale_variance..config.scale_variance);
    let angular_velocity = config.angular_velocity
        + rng.random_range(-config.angular_velocity_variance..config.angular_velocity_variance);

    // 计算方向向量
    let base_direction = config.direction.normalize_or_zero();
    let direction = if config.direction_variance > 0.0 {
        apply_direction_variance(base_direction, config.direction_variance, rng)
    } else {
        base_direction
    };

    // 根据发射形状计算偏移位置
    let emission_offset = match config.emission_shape {
        EmissionShape::Point => Vec3::ZERO,
        EmissionShape::Circle { radius } => {
            let angle = rng.random_range(0.0..std::f32::consts::TAU);
            let distance = rng.random_range(0.0..radius);
            Vec3::new(angle.cos() * distance, angle.sin() * distance, 0.0)
        }
        EmissionShape::Cone { angle } => {
            let cone_angle = rng.random_range(-angle..angle);
            let rotated = rotate_vector_2d(base_direction, cone_angle);
            rotated * rng.random_range(0.0..1.0)
        }
    };

    // 计算速度
    let velocity = direction * speed;

    // 计算生成位置
    let emitter_position = global_transform.translation();
    let mut position = emitter_position + emission_offset;

    // 设置渲染层级
    position.z = 25.0;

    // 计算颜色渐变
    let start_color = config.color;
    let mid_color = {
        let linear = config.color.to_linear();
        Color::LinearRgba(LinearRgba::new(
            linear.red * 0.7,
            linear.green * 0.7,
            linear.blue * 0.7,
            linear.alpha,
        ))
    };
    let end_color = Color::srgba(
        config.color.to_linear().red * 0.3,
        config.color.to_linear().green * 0.3,
        config.color.to_linear().blue * 0.3,
        0.0,
    );

    // 创建粒子组件
    let particle = Particle::new(velocity, lifetime, scale, start_color)
        .with_angular_velocity(angular_velocity)
        .with_acceleration(config.acceleration)
        .with_color_curve(mid_color, end_color)
        .with_scale_curve(scale * 0.2);

    // 创建网格和材质
    let size = 24.0 * scale;
    let mesh = meshes.add(Rectangle::new(size, size));
    let material = materials.add(ParticleMaterial::new(start_color));

    // 生成粒子实体
    commands.spawn((
        particle,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(position),
    ));
}

/// 更新粒子系统 - 更新粒子位置、旋转、颜色和缩放
pub fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(
        Entity,
        &mut Particle,
        &mut Transform,
        &MeshMaterial2d<ParticleMaterial>,
    )>,
    mut materials: ResMut<Assets<ParticleMaterial>>,
) {
    for (entity, mut particle, mut transform, material_handle) in particles.iter_mut() {
        // 更新生命周期
        particle.lifetime -= time.delta_secs();

        // 移除生命周期结束的粒子
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        // 应用加速度
        let acceleration = particle.acceleration;
        particle.velocity += acceleration * time.delta_secs();
        
        // 更新位置
        transform.translation += particle.velocity * time.delta_secs();
        
        // 更新旋转
        transform.rotate_z(particle.angular_velocity * time.delta_secs());

        // 更新颜色和缩放
        let current_color = particle.current_color();
        let current_scale = particle.current_scale();
        transform.scale = Vec3::splat(current_scale);

        // 更新材质颜色
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.color = current_color.to_linear();
        }
    }
}

/// 应用方向方差
fn apply_direction_variance(direction: Vec3, variance: f32, rng: &mut impl Rng) -> Vec3 {
    let angle = rng.random_range(-variance..variance);
    rotate_vector_2d(direction, angle)
}

/// 2D向量旋转
fn rotate_vector_2d(vec: Vec3, angle: f32) -> Vec3 {
    let cos = angle.cos();
    let sin = angle.sin();
    Vec3::new(vec.x * cos - vec.y * sin, vec.x * sin + vec.y * cos, vec.z)
}

/// 清理完成发射器系统 - 移除已完成的一次性发射器
pub fn cleanup_finished_emitters(
    mut commands: Commands,
    emitters: Query<(Entity, &ParticleEmitter)>,
) {
    for (entity, emitter) in emitters.iter() {
        if emitter.one_shot && !emitter.active {
            commands.entity(entity).despawn();
        }
    }
}
