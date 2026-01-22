mod inventory;
mod systems;

use crate::state::GameState;
use bevy::prelude::*;
pub use inventory::{Inventory, ItemKind, Pickable};
use systems::handle_pickups;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .add_systems(Update, handle_pickups.run_if(in_state(GameState::Playing)));
    }
}
