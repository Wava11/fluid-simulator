pub mod acceleration;
pub mod bounds;
pub mod collisions;
pub mod forces;
pub mod gravity;
pub mod mass;
pub mod velocity;

use bevy::prelude::*;

pub struct KineticsPlugin;

impl Plugin for KineticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(collisions::position_hashing::PositionHashingPlugin)
            .add_systems(
                FixedUpdate,
                (
                    gravity::apply_gravity,
                    collisions::apply_collisions,
                    bounds::enforce_bounds,
                    forces::apply_forces,
                    acceleration::accelerate_entities,
                    velocity::move_entities,
                )
                    .chain(),
            );
    }
}
