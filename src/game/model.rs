use bevy::prelude::*;

#[derive(Component)]
pub struct SimpleCamera;

#[derive(States, Clone, PartialEq, Eq, Debug, Hash, Default)]
pub enum GameState {
    Paused,
    Shell,
    #[default]
    Movement,
}
