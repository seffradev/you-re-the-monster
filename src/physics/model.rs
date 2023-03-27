use bevy::prelude::*;

#[derive(Component, Debug, Deref, DerefMut, Clone, Copy)]
pub struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Self(16.)
    }
}

#[derive(Component, Default, Debug)]
pub struct Collider;

#[derive(Default, Debug)]
pub struct CollisionEvent;
