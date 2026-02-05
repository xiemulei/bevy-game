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

pub fn resolve_entity_collisions(mut query: Query<(Entity, &Transform, &mut Velocity, &Collider)>) {
    let entities: Vec<_> = query
        .iter()
        .map(|(e, t, _, c)| (e, c.world_position(t), c.radius))
        .collect();

    for (entity, transform, mut velocity, collider) in query.iter_mut() {
        if !velocity.is_moving() {
            continue;
        }

        let pos = collider.world_position(transform);
        let radius = collider.radius;

        for &(other_entity, other_pos, other_radius) in &entities {
            if entity == other_entity {
                continue;
            }

            let delta = other_pos - pos;
            let distance = delta.length();
            let min_distance = radius + other_radius;

            if distance < min_distance * 1.1 {
                if distance > 0.01 {
                    let direction = delta / distance;
                    let velocity_toward = velocity.0.dot(direction);
                    if velocity_toward > 0.0 {
                        velocity.0 -= direction * velocity_toward;
                    }
                }
            }
        }
    }
}
