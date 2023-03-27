use bevy::prelude::*;

use super::model::GameState;

pub fn unpause(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Movement);
    }
}

pub fn pause(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

pub fn exit_terminal(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.pressed(KeyCode::LShift) && keys.just_pressed(KeyCode::T) {
        next_state.set(GameState::Movement);
    }
}

pub fn enter_terminal(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.pressed(KeyCode::LShift) && keys.just_pressed(KeyCode::T) {
        next_state.set(GameState::Shell);
    }
}
