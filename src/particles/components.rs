use bevy::prelude::*;

/// 粒子组件 - 定义单个粒子的属性和行为
#[derive(Component, Clone)]
pub struct Particle {
    /// 速度向量
    pub velocity: Vec3,
    /// 剩余生命周期（秒）
    pub lifetime: f32,
    /// 最大生命周期（秒）
    pub max_lifetime: f32,
    #[allow(dead_code)]
    pub scale: f32,
    /// 角速度（弧度/秒）
    pub angular_velocity: f32,
    /// 加速度向量
    pub acceleration: Vec3,
    /// 初始颜色
    pub start_color: Color,
    /// 中期颜色
    pub mid_color: Color,
    /// 结束颜色
    pub end_color: Color,
    /// 初始缩放
    pub start_scale: f32,
    /// 结束缩放
    pub end_scale: f32,
}

impl Particle {
    /// 创建新粒子
    pub fn new(velocity: Vec3, lifetime: f32, scale: f32, start_color: Color) -> Self {
        Self {
            velocity,
            lifetime,
            max_lifetime: lifetime,
            scale,
            angular_velocity: 0.0,
            acceleration: Vec3::ZERO,
            start_color,
            mid_color: start_color,
            end_color: start_color,
            start_scale: scale,
            end_scale: scale * 0.5,
        }
    }

    /// 设置角速度
    pub fn with_angular_velocity(mut self, angular_velocity: f32) -> Self {
        self.angular_velocity = angular_velocity;
        self
    }

    /// 设置加速度
    pub fn with_acceleration(mut self, acceleration: Vec3) -> Self {
        self.acceleration = acceleration;
        self
    }

    /// 设置颜色渐变曲线
    pub fn with_color_curve(mut self, mid_color: Color, end_color: Color) -> Self {
        self.mid_color = mid_color;
        self.end_color = end_color;
        self
    }

    /// 设置缩放渐变曲线
    pub fn with_scale_curve(mut self, end_scale: f32) -> Self {
        self.end_scale = end_scale;
        self
    }

    /// 获取粒子进度（0.0-1.0）
    pub fn progress(&self) -> f32 {
        1.0 - (self.lifetime / self.max_lifetime)
    }

    /// 获取当前颜色
    pub fn current_color(&self) -> Color {
        let progress = self.progress();

        if progress < 0.5 {
            let t = progress * 2.0;
            self.start_color.mix(&self.mid_color, t)
        } else {
            let t = (progress - 0.5) * 2.0;
            self.mid_color.mix(&self.end_color, t)
        }
    }

    /// 获取当前缩放
    pub fn current_scale(&self) -> f32 {
        let progress = self.progress();
        self.start_scale.lerp(self.end_scale, progress)
    }
}

/// 粒子发射器组件 - 控制粒子生成
#[derive(Component, Clone)]
pub struct ParticleEmitter {
    /// 生成计时器
    pub spawn_timer: Timer,
    /// 每次生成的粒子数量
    pub particles_per_spawn: u32,
    /// 粒子配置
    pub particle_config: ParticleConfig,
    /// 是否激活
    pub active: bool,
    /// 是否为一次性发射
    pub one_shot: bool,
    /// 是否已生成过
    pub has_spawned: bool,
}

impl ParticleEmitter {
    /// 创建新的粒子发射器
    pub fn new(spawn_rate: f32, particles_per_spawn: u32, particle_config: ParticleConfig) -> Self {
        Self {
            spawn_timer: Timer::from_seconds(spawn_rate, TimerMode::Repeating),
            particles_per_spawn,
            particle_config,
            active: true,
            one_shot: false,
            has_spawned: false,
        }
    }

    /// 设置为一次性发射模式
    pub fn one_shot(mut self) -> Self {
        self.one_shot = true;
        self
    }
}

/// 粒子配置 - 定义粒子的生成参数
#[derive(Clone)]
pub struct ParticleConfig {
    /// 生命周期（秒）
    pub lifetime: f32,
    /// 生命周期方差
    pub lifetime_variance: f32,
    /// 速度
    pub speed: f32,
    /// 速度方差
    pub speed_variance: f32,
    /// 方向向量
    pub direction: Vec3,
    /// 方向方差（弧度）
    pub direction_variance: f32,
    /// 缩放
    pub scale: f32,
    /// 缩放方差
    pub scale_variance: f32,
    /// 颜色
    pub color: Color,
    /// 角速度（弧度/秒）
    pub angular_velocity: f32,
    /// 角速度方差
    pub angular_velocity_variance: f32,
    /// 加速度向量
    pub acceleration: Vec3,
    /// 发射形状
    pub emission_shape: EmissionShape,
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            lifetime: 1.0,
            lifetime_variance: 0.1,
            speed: 100.0,
            speed_variance: 10.0,
            direction: Vec3::X,
            direction_variance: 0.1,
            scale: 1.0,
            scale_variance: 0.1,
            color: Color::WHITE,
            angular_velocity: 0.0,
            angular_velocity_variance: 0.0,
            acceleration: Vec3::ZERO,
            emission_shape: EmissionShape::Point,
        }
    }
}

/// 粒子发射形状
#[derive(Clone)]
pub enum EmissionShape {
    /// 点发射
    Point,
    /// 圆形发射
    Circle {
        radius: f32,
    },
    /// 锥形发射
    #[allow(dead_code)]
    Cone {
        angle: f32,
    },
}
