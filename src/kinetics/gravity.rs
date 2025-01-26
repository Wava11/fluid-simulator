use bevy::prelude::*;

use super::{forces::Forces, mass::Mass};


pub fn apply_gravity(mut query: Query<(&Mass, &mut Forces)>) -> () {
    for (Mass(mass), mut forces) in query.iter_mut() {
        forces
            .0
            .push(mass * GRAVITY_ACCELERATION * Vec2::new(0., -1.));
    }
}

const GRAVITY_ACCELERATION: f32 = 9.8;
