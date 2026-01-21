use bevy::prelude::States;

#[derive(States, Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    Paused,
}
