use bevy::prelude::Component;

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum CharacterState {
    #[default]
    Idle,
    Walking,
    Running,
    Jumping,
}

impl CharacterState {
    pub fn is_grounded(&self) -> bool {
        matches!(
            self,
            CharacterState::Idle | CharacterState::Walking | CharacterState::Running
        )
    }
}
