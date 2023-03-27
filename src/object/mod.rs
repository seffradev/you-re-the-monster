use crate::{
    input::model::{HasInput, MovementDirection},
    physics::model::{Collider, Speed}, game::{WINDOW_WIDTH, WINDOW_HEIGHT},
};
use bevy::prelude::*;

pub mod controller;
pub mod model;
pub mod view;

pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("sprites/player.png"),
            transform: Transform::from_xyz(160., 0., 0.),
            ..default()
        },
        ..default()
    });

    let mut x = 0.;
    while x <= WINDOW_WIDTH {
        commands.spawn(WallBundle::new(
            &asset_server,
            snap_number(x - WINDOW_WIDTH / 2.),
            snap_number(WINDOW_HEIGHT / 2. + 16.),
            0.,
        ));
        commands.spawn(WallBundle::new(
            &asset_server,
            snap_number(x - WINDOW_WIDTH / 2.),
            snap_number(-WINDOW_HEIGHT / 2.),
            0.,
        ));
        x += 16.;
    }

    let mut y = 0.;
    while y <= WINDOW_WIDTH {
        commands.spawn(WallBundle::new(
            &asset_server,
            snap_number(-WINDOW_WIDTH / 2.),
            snap_number(WINDOW_HEIGHT / 2. - y),
            0.,
        ));
        commands.spawn(WallBundle::new(
            &asset_server,
            snap_number(WINDOW_WIDTH / 2.),
            snap_number(WINDOW_HEIGHT / 2. - y),
            0.,
        ));
        y += 16.;
    }

    commands.spawn(WallBundle::new(&asset_server, 64., 32., 0.));
    commands.spawn(WallBundle::new(&asset_server, 80., 32., 0.));
}

#[derive(Bundle, Default)]
struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    speed: Speed,
    collider: Collider,
    direction: MovementDirection,
    has_input: HasInput,
}

#[derive(Bundle, Default)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    fn new(asset_server: &Res<AssetServer>, x: f32, y: f32, z: f32) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("sprites/wall.png"),
                transform: Transform::from_xyz(x, y, z),
                ..default()
            },
            ..default()
        }
    }
}

fn snap_number(number: f32) -> f32 {
    (number / 16.).floor() * 16.
}
