use crate::physics::{Acceleration, Friction, HasInput, Speed, Velocity, Collider};
use bevy::prelude::*;

pub struct ObjectsPlugin;

#[derive(Bundle, Default)]
struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    speed: Speed,
    has_input: HasInput,
    friction: Friction,
    velocity: Velocity,
    acceleration: Acceleration,
    collider: Collider,
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

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        sprite_bundle: SpriteBundle {
            texture: asset_server.load("sprites/player.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        ..default()
    });

    commands.spawn(WallBundle::new(&asset_server, 50., 30., 0.));
}
