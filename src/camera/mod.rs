mod camera;

use crate::state::GameState;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera::setup_camera).add_systems(
            Update,
            camera::follow_camera.run_if(in_state(GameState::Playing)),
        );
    }
}
