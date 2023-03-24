use crate::game::GameState;
use bevy::prelude::*;
use std::ops::Mul;

pub struct PlayerPlugin;

const FRICTION: f32 = 0.3;
const SPEED: f32 = 5.;

#[derive(Component, Default)]
struct Player {
    velocity: Vec3,
    acceleration: Vec3,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup)
            .add_systems((handle_input, movement, friction).in_set(OnUpdate(GameState::Movement)));
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/duck.png"),
            transform: Transform::from_xyz(100., 0., 0.).with_scale(Vec3::new(0.3, 0.3, 1.)),
            ..default()
        },
        Player::default(),
    ));
}

fn movement(mut position: Query<(&mut Player, &mut Transform)>) {
    let (mut player, mut transform) = position.single_mut();

    transform.translation += player.velocity;
    player.velocity = player.velocity + player.acceleration;
}

fn friction(mut player: Query<&mut Player>) {
    let mut player = player.single_mut();

    player.velocity = player.velocity - player.velocity.mul(FRICTION);

    if player.velocity.length() <= 0.01 {
        player.velocity = Vec3::new(0., 0., 0.);
    }
}

fn handle_input(keys: Res<Input<KeyCode>>, mut position: Query<&mut Player>) {
    let mut player = position.single_mut();

    let movement = get_movement_direction(keys);

    player.velocity += movement.mul(SPEED);
}

fn get_movement_direction(keys: Res<Input<KeyCode>>) -> Vec3 {
    let horizontal = f32::from(keys.pressed(KeyCode::D)) - f32::from(keys.pressed(KeyCode::A));
    let vertical = f32::from(keys.pressed(KeyCode::W)) - f32::from(keys.pressed(KeyCode::S));

    Vec3::new(horizontal, vertical, 0.)
}
