use crate::characters::physics::Velocity;
use crate::collision::CollisionMap;
use crate::config::player::COLLIDER_RADIUS;
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Collider {
    pub radius: f32,
    pub offset: Vec2,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            radius: COLLIDER_RADIUS,
            offset: Vec2::ZERO,
        }
    }
}
impl Collider {
    pub fn world_position(&self, transform: &Transform) -> Vec2 {
        transform.translation.truncate() + self.offset
    }
}

pub fn validate_movement(
    map: Option<Res<CollisionMap>>,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut Velocity, &Collider)>,
) {
    let Some(map) = map else {
        return;
    };

    for (transform, mut velocity, collider) in query.iter_mut() {
        if !velocity.is_moving() {
            continue;
        }

        let current_pos = collider.world_position(transform);

        let delta = velocity.0 * time.delta_secs();
        let desired_pos = current_pos + delta;

        let valid_pos = map.sweep_circle(current_pos, desired_pos, collider.radius);

        let actual_delta = valid_pos - current_pos;

        if (actual_delta - delta).length_squared() > 0.001 {
            let dt = time.delta_secs();
            if dt > 0.0 {
                velocity.0 = actual_delta / dt;
            }
        }
    }
}
