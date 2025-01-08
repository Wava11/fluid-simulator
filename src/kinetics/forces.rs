use bevy::{ecs::query, prelude::*};

use super::{acceleration::{self, Acceleration}, mass::Mass};


pub struct ForcesPlugin;

impl Plugin for ForcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_forces);
    }
}

fn apply_forces(mut query: Query<(&Forces, &Mass, &mut Acceleration)>) {
    for (Forces(forces), Mass(mass), mut acceleration) in query.iter_mut(){
        acceleration.0 = forces.iter().sum::<Vec2>() / mass;
    }
}

#[derive(Component)]
pub struct Forces(pub Vec<Vec2>);