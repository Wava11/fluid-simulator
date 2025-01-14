use crate::{bounds::enforce_bounds, fluids::particle::FluidParticle};

use super::{
    acceleration::accelerate_entities,
    forces::{apply_forces, Forces},
    mass::Mass,
    velocity::{self, Velocity},
};
use bevy::{prelude::*, transform};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, apply_collisions.before(accelerate_entities));
    }
}

fn apply_collisions(
    mut query: Query<(
        &FluidParticle,
        &mut Transform,
        &Mass,
        &Velocity,
        &mut Forces,
    )>,
) {
    for (particle, transform, Mass(mass), Velocity(velocity), mut forces) in query.iter() {
        
    }
}
