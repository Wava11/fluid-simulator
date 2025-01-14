use crate::bounds::enforce_bounds;

use super::{forces::apply_forces, velocity::Velocity};
use bevy::prelude::*;

pub struct AccelerationPlugin;

impl Plugin for AccelerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, accelerate_entities.after(enforce_bounds).after(apply_forces));
    }
}

#[derive(Component)]
pub struct Acceleration(pub Vec2);

pub fn accelerate_entities(time: Res<Time>, mut query: Query<(&Acceleration, &mut Velocity)>) -> () {
    for (Acceleration(acceleration), mut velocity) in query.iter_mut() {
        velocity.0 += acceleration * time.delta().as_secs_f32();
    }
}
