mod characters;
mod map;

use crate::map::generate::{map_pixel_dimensions, setup_generator};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_procedural_tilemaps::prelude::{Cartesian3D, ProcGenSimplePlugin};
fn main() {
    let map_size = map_pixel_dimensions();

    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "src/assets".to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(map_size.x as u32, map_size.y as u32),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(ProcGenSimplePlugin::<Cartesian3D, Sprite>::default())
        .add_plugins(characters::CharactersPlugin)
        .add_systems(Startup, (setup_camera, setup_generator))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
