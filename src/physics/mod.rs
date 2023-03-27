use self::{
    controller::{apply_movement, check_for_collisions},
    model::CollisionEvent,
};
use crate::game::model::GameState;
use bevy::prelude::*;

pub mod controller;
pub mod model;
pub mod view;

const TIMESTEP: f32 = 0.05;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(
                (
                    apply_movement.after(check_for_collisions),
                    check_for_collisions,
                )
                    .in_set(OnUpdate(GameState::Movement))
                    .in_schedule(CoreSchedule::FixedUpdate),
            )
            .insert_resource(FixedTime::new_from_secs(TIMESTEP));
    }
}
