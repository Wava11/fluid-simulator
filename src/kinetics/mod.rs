pub mod acceleration;
pub mod forces;
pub mod mass;
pub mod velocity;
pub mod gravity;
pub mod collisions;

use bevy::prelude::*;

pub struct KineticsPlugin;

impl Plugin for KineticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            velocity::VelocityPlugin,
            acceleration::AccelerationPlugin,
            forces::ForcesPlugin,
            gravity::GravityPlugin
        ));
    }
}
