use bevy::prelude::*;

use super::model::{AccessLevel, CurrentPosition, Hostname, Selected, ShellCommands, ShellData};

pub fn input(
    keys: Res<Input<KeyCode>>,
    mut event_reader: EventReader<ReceivedCharacter>,
    mut shells: Query<
        (
            &mut CurrentPosition,
            &mut ShellData,
            &mut AccessLevel,
            &Hostname,
        ),
        With<Selected>,
    >,
) {
    for (mut current_position, mut shell_data, mut access_level, hostname) in shells.iter_mut() {
        if keys.just_pressed(KeyCode::Return) {
            ShellCommands::process(
                &mut access_level,
                &mut shell_data,
                &mut current_position,
                hostname.clone(),
            );
        } else if keys.just_pressed(KeyCode::Back) {
            shell_data.pop(&mut current_position);
        } else {
            ShellData::insert(
                &mut event_reader,
                shell_data,
                current_position,
                access_level,
            );
        }
    }
}
