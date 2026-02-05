use crate::characters::state::CharacterState;
use crate::config::map::{GRID_Y, TILE_SIZE};
use crate::config::player::PLAYER_SCALE;
use bevy::prelude::*;

const NODE_SIZE_Z: f32 = 1.0;
const CHARACTER_BASE_Z: f32 = 4.0;
const CHARACTER_Z_OFFSET: f32 = 0.5;

pub fn update_character_depth(
    mut character_query: Query<&mut Transform, (With<CharacterState>, Changed<Transform>)>,
) {
    let map_height = TILE_SIZE * GRID_Y as f32;
    let map_y0 = -TILE_SIZE * GRID_Y as f32 / 2.0;

    let character_sprite_height = 64.0 * PLAYER_SCALE;

    for mut transform in character_query.iter_mut() {
        let character_center_y = transform.translation.y;
        let character_feet_y = character_center_y - character_sprite_height / 2.0;
        let t = ((character_feet_y - map_y0) / map_height).clamp(0.0, 1.0);
        let character_z = CHARACTER_BASE_Z + NODE_SIZE_Z * (1.0 - t) + CHARACTER_Z_OFFSET;
        transform.translation.z = character_z;
    }
}
