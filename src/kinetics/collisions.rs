// use crate::bounds::enforce_bounds;

// use super::{acceleration::accelerate_entities, forces::apply_forces, velocity::Velocity};
// use bevy::prelude::*;

// pub struct CollisionsPlugin;

// impl Plugin for CollisionsPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Update, apply_collisions.before(accelerate_entities));
//     }
// }

// fn apply_collisions(mut query: Query<(&Acceleration, &mut Velocity)>) -> () {
    
// }