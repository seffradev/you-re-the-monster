use bevy::prelude::*;

use super::model::{MovementDirection, HasInput};

pub fn handle_input(keys: Res<Input<KeyCode>>, mut direction: Query<&mut MovementDirection, With<HasInput>>) {
    trace!("handle_input");

    let movement = get_movement_direction(keys);
    trace!("{:?}", movement);

    for mut direction in direction.iter_mut() {
        trace!("{:?}", direction);

        *direction = movement;
    }
}

pub fn get_movement_direction(keys: Res<Input<KeyCode>>) -> MovementDirection {
    trace!("get_movement_direction");

    if keys.pressed(KeyCode::L) {
        return MovementDirection::Right;
    }

    if keys.pressed(KeyCode::H) {
        return MovementDirection::Left;
    }

    if keys.pressed(KeyCode::K) {
        return MovementDirection::Up;
    }

    if keys.pressed(KeyCode::J) {
        return MovementDirection::Down;
    }

    MovementDirection::None
}
