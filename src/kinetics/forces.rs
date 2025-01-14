use bevy::{ecs::query, prelude::*};

use crate::bounds::enforce_bounds;

use super::{acceleration::{self, Acceleration}, mass::Mass};


pub struct ForcesPlugin;

impl Plugin for ForcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPostUpdate, apply_forces);
    }
}

pub fn apply_forces(mut query: Query<(&mut Forces, &Mass, &mut Acceleration)>) {
    for (mut forces, Mass(mass), mut acceleration) in query.iter_mut(){
        // println!("forces: {:?}", forces.0);
        acceleration.0 = forces.0.iter().sum::<Vec2>() / mass;
        forces.0.clear();
    }

}

#[derive(Component)]
pub struct Forces(pub Vec<Vec2>);