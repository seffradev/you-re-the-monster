use super::model::{Collider, CollisionEvent, Speed};
use crate::input::model::MovementDirection;
use bevy::{prelude::*, sprite::collide_aabb::collide};

pub fn apply_movement(mut query: Query<(&mut Transform, &MovementDirection, &Speed)>) {
    trace!("apply_movement");

    for (mut transform, direction, speed) in query.iter_mut() {
        trace!("{:?} {:?}", transform, direction);

        match direction {
            MovementDirection::Up => transform.translation += Vec3::new(0., **speed, 0.),
            MovementDirection::Down => transform.translation += Vec3::new(0., -**speed, 0.),
            MovementDirection::Left => transform.translation += Vec3::new(-**speed, 0., 0.),
            MovementDirection::Right => transform.translation += Vec3::new(**speed, 0., 0.),
            MovementDirection::None => { /* do nothing */ }
        }
    }
}

pub fn check_for_collisions(
    mut me: Query<(Entity, &Transform, &mut MovementDirection, &Speed), With<Collider>>,
    you: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for (my_entity, my_transform, mut my_direction, my_speed) in me.iter_mut() {
        for (your_entity, your_transform) in you.iter() {
            if my_entity == your_entity {
                continue;
            }

            let my_shifted_translation = match *my_direction {
                MovementDirection::Up => my_transform.translation + Vec3::new(0., **my_speed, 0.),
                MovementDirection::Down => {
                    my_transform.translation + Vec3::new(0., -**my_speed, 0.)
                }
                MovementDirection::Left => {
                    my_transform.translation + Vec3::new(-**my_speed, 0., 0.)
                }
                MovementDirection::Right => {
                    my_transform.translation + Vec3::new(**my_speed, 0., 0.)
                }
                MovementDirection::None => my_transform.translation,
            };

            let collision = collide(
                my_shifted_translation,
                my_transform.scale.truncate(),
                your_transform.translation,
                your_transform.scale.truncate(),
            );

            if let Some(_) = collision {
                collision_events.send_default();

                *my_direction = MovementDirection::None;
            }
        }
    }
}
