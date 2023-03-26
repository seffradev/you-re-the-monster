use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use std::ops::{AddAssign, Mul, MulAssign, SubAssign};

use crate::game::GameState;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>().add_systems(
            (
                handle_input,
                apply_acceleration,
                friction,
                apply_velocity,
                check_for_collisions,
                log_collisions,
            )
                .in_set(OnUpdate(GameState::Movement)),
        );
    }
}

#[derive(Component, Default, Clone, Copy)]
pub struct HasInput;

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Friction(f32);

impl Default for Friction {
    fn default() -> Self {
        Self(0.3)
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Self(2.)
    }
}

#[derive(Component, Default, Deref, DerefMut, Clone, Copy)]
pub struct Velocity(Vec3);

#[derive(Component, Default, Deref, DerefMut, Clone, Copy)]
pub struct Acceleration(Vec3);

#[derive(Component, Default)]
pub struct Collider;

#[derive(Default)]
struct CollisionEvent;

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.add_assign(**velocity);
    }
}

fn apply_acceleration(mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.add_assign(**acceleration);
    }
}

fn friction(mut velocities: Query<(&mut Velocity, &Friction)>) {
    for (mut velocity, &friction) in velocities.iter_mut() {
        let friction = velocity.mul(*friction);

        velocity.sub_assign(friction);

        if velocity.length() <= 0.01 {
            velocity.mul_assign(0.);
        }
    }
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut velocities: Query<(&mut Velocity, &Speed), With<HasInput>>,
) {
    let movement = get_movement_direction(keys);

    for (mut velocity, &speed) in velocities.iter_mut() {
        velocity.add_assign(movement.mul(*speed));
    }
}

fn get_movement_direction(keys: Res<Input<KeyCode>>) -> Vec3 {
    let horizontal = f32::from(keys.pressed(KeyCode::D)) - f32::from(keys.pressed(KeyCode::A));
    let vertical = f32::from(keys.pressed(KeyCode::W)) - f32::from(keys.pressed(KeyCode::S));

    Vec3::new(horizontal, vertical, 0.)
}

fn check_for_collisions(
    mut me: Query<(Entity, &Transform, &mut Velocity), With<Collider>>,
    you: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for (my_entity, my_transform, _my_velocity) in me.iter_mut() {
        for (your_entity, your_transform) in you.iter() {
            if my_entity == your_entity {
                continue;
            }

            let collision = collide(
                my_transform.translation,
                my_transform.scale.truncate(),
                your_transform.translation,
                your_transform.scale.truncate(),
            );

            if let Some(collision) = collision {
                println!("fff");

                collision_events.send_default();

                println!("ggg");

                match collision {
                    Collision::Left | Collision::Right => {}
                    Collision::Top | Collision::Bottom => {}
                    Collision::Inside => {}
                }
            }
        }
    }
}

fn log_collisions(mut collision_events: EventReader<CollisionEvent>) {
    if !collision_events.is_empty() {
        collision_events.clear();

        println!("haha you collided");
    }
}
