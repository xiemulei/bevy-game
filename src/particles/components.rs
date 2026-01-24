use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Particle {
    pub velocity: Vec3,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub scale: f32,
    pub angular_velocity: f32,
    pub acceleration: Vec3,
    pub start_color: Color,
    pub mid_color: Color,
    pub end_color: Color,
    pub start_scale: f32,
    pub end_scale: f32,
}

impl Particle {
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

    pub fn with_angular_velocity(mut self, angular_velocity: f32) -> Self {
        self.angular_velocity = angular_velocity;
        self
    }

    pub fn with_acceleration(mut self, acceleration: Vec3) -> Self {
        self.acceleration = acceleration;
        self
    }

    pub fn with_color_curve(mut self, mid_color: Color, end_color: Color) -> Self {
        self.mid_color = mid_color;
        self.end_color = end_color;
        self
    }

    pub fn with_scale_curve(mut self, end_scale: f32) -> Self {
        self.end_scale = end_scale;
        self
    }

    pub fn progress(&self) -> f32 {
        1.0 - (self.lifetime / self.max_lifetime)
    }

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

    pub fn current_scale(&self) -> f32 {
        let progress = self.progress();
        self.start_scale.lerp(self.end_scale, progress)
    }
}

#[derive(Component, Clone)]
pub struct ParticleEmitter {
    pub spawn_timer: Timer,
    pub particles_per_spawn: u32,
    pub particle_config: ParticleConfig,
    pub active: bool,
    pub one_shot: bool,
    pub has_spawned: bool,
}

impl ParticleEmitter {
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

    pub fn one_shot(mut self) -> Self {
        self.one_shot = true;
        self
    }
}

#[derive(Clone)]
pub struct ParticleConfig {
    pub lifetime: f32,
    pub lifetime_variance: f32,
    pub speed: f32,
    pub speed_variance: f32,
    pub direction: Vec3,
    pub direction_variance: f32,
    pub scale: f32,
    pub scale_variance: f32,
    pub color: Color,
    pub angular_velocity: f32,
    pub angular_velocity_variance: f32,
    pub acceleration: Vec3,
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

#[derive(Clone)]
pub enum EmissionShape {
    Point,
    Circle { radius: f32 },
    Cone { angle: f32 },
}
