use self::{
    controller::{enter_terminal, exit_terminal, pause, unpause},
    model::{GameState, SimpleCamera},
};
use crate::{
    input::InputPlugin, object::ObjectsPlugin, physics::PhysicsPlugin, shell::ShellPlugin,
};
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowResolution,
};

pub mod controller;
pub mod model;
pub mod view;

pub const WINDOW_WIDTH: f32 = 960.;
pub const WINDOW_HEIGHT: f32 = 540.;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "You're the Monster".to_string(),
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_state::<GameState>()
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(InputPlugin)
        .add_plugin(ShellPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(ObjectsPlugin)
        .add_startup_system(setup)
        .add_system(pause.in_set(OnUpdate(GameState::Shell)))
        .add_system(pause.in_set(OnUpdate(GameState::Movement)))
        .add_system(unpause.in_set(OnUpdate(GameState::Paused)))
        .add_system(exit_terminal.in_set(OnUpdate(GameState::Shell)))
        .add_system(enter_terminal.in_set(OnUpdate(GameState::Movement)));
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), SimpleCamera));
}
