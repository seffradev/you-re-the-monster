use bevy::prelude::*;

use super::model::{Selected, ShellData};

#[derive(Bundle, Default)]
pub struct ShellView {
    pub node_bundle: NodeBundle,
    pub shell_screen: ShellScreen,
}

#[derive(Component, Default)]
pub struct ShellScreen;

pub fn hide(mut shell_screens: Query<&mut Visibility, With<ShellScreen>>) {
    for mut shell_screen in &mut shell_screens {
        *shell_screen = Visibility::Hidden;
    }
}

pub fn show(mut shell_screens: Query<&mut Visibility, With<ShellScreen>>) {
    for mut shell_screen in &mut shell_screens {
        *shell_screen = Visibility::Inherited;
    }
}

pub fn show_shell(
    mut texts: Query<&mut Text, With<ShellScreen>>,
    shell_data: Query<(&ShellData, Option<&Selected>)>,
) {
    for (shell_data, selected) in shell_data.iter() {
        if let None = selected {
            continue;
        }

        for mut text in texts.iter_mut() {
            for (idx, row) in shell_data.0.iter().enumerate() {
                text.sections[idx].value = format!("{}\n", row.iter().collect::<String>());
            }
        }
    }
}
