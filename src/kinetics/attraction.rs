use bevy::prelude::*;

use super::{forces::Forces, mass::Mass};


// pub fn apply_attraction(mut particles_query: Query<(&Mass, &Transform, &mut Forces), With<FluidParticle>) -> () {
//     for (Mass(mass), mut forces) in query.iter_mut() {
//         forces
//             .0
//             .push(mass * GRAVITY_ACCELERATION * Vec2::new(0., -1.));
//     }
// }