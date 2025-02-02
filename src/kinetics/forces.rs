use bevy::{math::VectorSpace, prelude::*};

use super::{acceleration::Acceleration, mass::Mass};

pub fn apply_forces(mut query: Query<(&mut Forces, &Mass, &mut Acceleration)>) {
    query
        .par_iter_mut()
        .for_each(|(mut forces, Mass(mass), mut acceleration)| {
            acceleration.0 = forces.0.iter().sum::<Vec2>() / mass;

            forces.0.clear();
        });
    // for (mut forces, Mass(mass), mut acceleration) in query.iter_mut() {
    //     acceleration.0 = forces.0.iter().sum::<Vec2>() / mass;

    //     forces.0.clear();
    // }
}

#[derive(Component, Clone)]
pub struct Forces(pub Vec<Vec2>);
