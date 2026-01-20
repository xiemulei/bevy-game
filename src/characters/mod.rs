use crate::characters::config::CharactersList;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

mod animation;
mod config;
mod movement;
mod spawn;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<CharactersList>::new(&["characters.ron"]))
            .init_resource::<spawn::CurrentCharacterIndex>()
            .add_systems(Startup, spawn::spawn_player)
            .add_systems(
                Update,
                (
                    spawn::initialize_player_character,
                    spawn::switch_character,
                    movement::move_player,
                    movement::update_jump_state,
                    animation::animate_characters,
                    animation::update_animation_flags,
                ),
            );
    }
}
