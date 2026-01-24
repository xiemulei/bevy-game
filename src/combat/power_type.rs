use crate::particles::components::{EmissionShape, ParticleConfig};
use bevy::prelude::*;

/// 能力类型枚举 - 定义不同的攻击技能
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum PowerType {
    #[default]
    Fire,    // 火焰
    Arcane,  // 奥术
    Shadow,  // 暗影
    Poison,  // 剧毒
}

/// 能力视觉效果配置 - 定义粒子的外观和行为
#[derive(Clone)]
pub struct PowerVisuals {
    /// 主粒子配置
    pub primary: ParticleConfig,
    /// 核心粒子配置（可选）
    pub core: Option<ParticleConfig>,
    /// 每次生成的主粒子数量
    pub particles_per_spawn: u32,
    /// 每次生成的核心粒子数量
    pub core_particles_per_spawn: u32,
}

impl PowerType {
    /// 获取能力的视觉效果配置
    pub fn visual(&self, direction: Vec3) -> PowerVisuals {
        match self {
            PowerType::Fire => Self::fire_visuals(direction),
            PowerType::Arcane => Self::arcane_visuals(direction),
            PowerType::Shadow => Self::shadow_visuals(direction),
            PowerType::Poison => Self::poison_visuals(direction),
        }
    }

    /// 火焰能力视觉效果
    fn fire_visuals(direction: Vec3) -> PowerVisuals {
        PowerVisuals {
            primary: ParticleConfig {
                lifetime: 1.0,
                lifetime_variance: 0.2,
                speed: 350.0,
                speed_variance: 40.0,
                direction,
                direction_variance: 0.12,
                scale: 1.5,
                scale_variance: 0.5,
                color: Color::srgb(3.0, 0.5, 0.1), // 明亮的橙红色
                angular_velocity: 3.0,
                angular_velocity_variance: 2.0,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Circle { radius: 10.0 },
            },
            core: Some(ParticleConfig {
                lifetime: 0.8,
                lifetime_variance: 0.2,
                speed: 350.0,
                speed_variance: 30.0,
                direction,
                direction_variance: 0.08,
                scale: 1.0,
                scale_variance: 0.3,
                color: Color::srgb(4.0, 1.0, 0.2), // 非常明亮的黄白色
                angular_velocity: 5.0,
                angular_velocity_variance: 2.0,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Circle { radius: 5.0 },
            }),
            particles_per_spawn: 5,
            core_particles_per_spawn: 3,
        }
    }

    /// 奥术能力视觉效果
    fn arcane_visuals(direction: Vec3) -> PowerVisuals {
        PowerVisuals {
            primary: ParticleConfig {
                lifetime: 1.2,
                lifetime_variance: 0.2,
                speed: 300.0,
                speed_variance: 30.0,
                direction,
                direction_variance: 0.05, // 非常精确
                scale: 1.2,
                scale_variance: 0.3,
                color: Color::srgb(0.5, 0.8, 2.5), // 蓝色奥术能量
                angular_velocity: 2.0,
                angular_velocity_variance: 1.0,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Circle { radius: 6.0 },
            },
            core: Some(ParticleConfig {
                lifetime: 1.0,
                lifetime_variance: 0.1,
                speed: 300.0,
                speed_variance: 20.0,
                direction,
                direction_variance: 0.03, // 更加精确
                scale: 0.8,
                scale_variance: 0.2,
                color: Color::srgb(0.9, 0.95, 3.0), // 明亮的白色蓝色
                angular_velocity: 0.5,
                angular_velocity_variance: 0.5,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Point,
            }),
            particles_per_spawn: 4,
            core_particles_per_spawn: 2,
        }
    }

    /// 暗影能力视觉效果
    fn shadow_visuals(direction: Vec3) -> PowerVisuals {
        PowerVisuals {
            primary: ParticleConfig {
                lifetime: 0.6, // 短生命周期
                lifetime_variance: 0.15,
                speed: 600.0, // 非常快
                speed_variance: 100.0,
                direction,
                direction_variance: 0.04,
                scale: 1.0,
                scale_variance: 0.4,
                color: Color::srgb(0.6, 0.2, 1.2), // 深紫色
                angular_velocity: 8.0,             // 快速旋转
                angular_velocity_variance: 4.0,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Point,
            },
            core: Some(ParticleConfig {
                lifetime: 0.5,
                lifetime_variance: 0.1,
                speed: 650.0,
                speed_variance: 80.0,
                direction,
                direction_variance: 0.02,
                scale: 1.3,
                scale_variance: 0.3,
                color: Color::srgb(1.0, 0.5, 1.8), // 更明亮的紫色核心
                angular_velocity: 12.0,
                angular_velocity_variance: 5.0,
                acceleration: Vec3::ZERO,
                emission_shape: EmissionShape::Point,
            }),
            particles_per_spawn: 7, // 大量粒子
            core_particles_per_spawn: 3,
        }
    }

    /// 剧毒能力视觉效果
    fn poison_visuals(direction: Vec3) -> PowerVisuals {
        PowerVisuals {
            primary: ParticleConfig {
                lifetime: 1.5, // 长生命周期
                lifetime_variance: 0.4,
                speed: 200.0, // 慢速
                speed_variance: 50.0,
                direction,
                direction_variance: 0.25, // 大范围扩散
                scale: 1.8,               // 大粒子
                scale_variance: 0.6,
                color: Color::srgb(0.3, 2.0, 0.3), // 有毒绿色
                angular_velocity: 1.0,
                angular_velocity_variance: 2.0,
                acceleration: Vec3::new(0.0, 20.0, 0.0), // 稍微向上漂浮
                emission_shape: EmissionShape::Circle { radius: 15.0 },
            },
            core: None, // 无核心 - 只有云雾
            particles_per_spawn: 6,
            core_particles_per_spawn: 0,
        }
    }
}
