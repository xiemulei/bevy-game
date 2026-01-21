#[cfg(debug_assertions)]
mod debug;
mod map;
mod systems;
mod tile_type;

use crate::state::GameState;
use bevy::prelude::*;

pub use map::CollisionMap;
pub use systems::CollisionMapBuilt;
pub use tile_type::{TileMarker, TileType};

#[cfg(debug_assertions)]
pub use debug::DebugCollisionEnabled;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollisionMapBuilt>().add_systems(
            Update,
            systems::build_collision_map
                .run_if(resource_equals(CollisionMapBuilt(false)))
                .run_if(in_state(GameState::Playing)),
        );

        #[cfg(debug_assertions)]
        app.init_resource::<DebugCollisionEnabled>().add_systems(
            Update,
            (
                debug::toggle_debug_collision,
                debug::debug_draw_collision,
                debug::debug_player_position,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
