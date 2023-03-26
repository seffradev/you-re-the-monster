use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowResolution,
};

use crate::{object::ObjectsPlugin, terminal::TerminalPlugin, physics::PhysicsPlugin};

pub struct Game;

#[derive(Component)]
pub struct SimpleCamera;

#[derive(States, Clone, PartialEq, Eq, Debug, Hash, Default)]
pub enum GameState {
    Paused,
    Terminal,
    #[default]
    Movement,
}

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "You're the Monster".to_string(),
                        resolution: WindowResolution::new(960., 540.),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_state::<GameState>()
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(TerminalPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(ObjectsPlugin)
        .add_startup_system(setup)
        .add_system(pause.in_set(OnUpdate(GameState::Terminal)))
        .add_system(pause.in_set(OnUpdate(GameState::Movement)))
        .add_system(unpause.in_set(OnUpdate(GameState::Paused)))
        .add_system(exit_terminal.in_set(OnUpdate(GameState::Terminal)))
        .add_system(enter_terminal.in_set(OnUpdate(GameState::Movement)));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), SimpleCamera));
}

fn unpause(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Movement);
    }
}

fn pause(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn exit_terminal(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.pressed(KeyCode::LShift) && keys.just_pressed(KeyCode::T) {
        next_state.set(GameState::Movement);
    }
}

fn enter_terminal(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.pressed(KeyCode::LShift) && keys.just_pressed(KeyCode::T) {
        next_state.set(GameState::Terminal);
    }
}
