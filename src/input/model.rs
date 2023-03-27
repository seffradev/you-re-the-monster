use bevy::prelude::*;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct HasInput;

#[derive(Component, Default, Debug, Clone, Copy)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
    #[default]
    None,
}
