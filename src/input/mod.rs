use self::controller::handle_input;
use crate::game::model::GameState;
use bevy::prelude::*;

pub mod controller;
pub mod model;
pub mod view;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((handle_input,).in_set(OnUpdate(GameState::Movement)));
    }
}
