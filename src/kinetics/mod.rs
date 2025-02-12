pub mod acceleration;
pub mod attraction;
pub mod bounds;
pub mod collisions;
pub mod flow;
pub mod forces;
pub mod gravity;
pub mod mass;
pub mod utils;
pub mod velocity;

use bevy::prelude::*;

use crate::controls::toggle_gravity::GravityToggled;

pub struct KineticsPlugin;

impl Plugin for KineticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(collisions::position_hashing::PositionHashingPlugin)
            .add_systems(Startup, bounds::draw_bounds)
            .add_systems(
                FixedUpdate,
                (
                    gravity::apply_gravity
                        .run_if(|gravity_toggled: Res<GravityToggled>| gravity_toggled.0),
                    // attraction::apply_attraction,
                    collisions::apply_collisions,
                    // collisions::_apply_collisions_single_threaded,
                    // flow::apply_flow,
                    bounds::enforce_bounds,
                    forces::apply_forces,
                    acceleration::accelerate_entities,
                    velocity::move_entities,
                )
                    .chain(),
            );
    }
}
