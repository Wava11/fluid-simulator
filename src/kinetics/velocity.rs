use bevy::prelude::*;

use crate::fluids::particle::FluidParticle;

use super::bounds::{MAX_X, MAX_Y, MIN_X, MIN_Y};

#[derive(Component, Clone)]
pub struct Velocity(pub Vec2);

pub fn move_entities(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform, &FluidParticle)>,
) -> () {
    query
        .par_iter_mut()
        .for_each(|(Velocity(velocity), mut transform, particle)| {
            transform.translation = (transform.translation
                + Vec3::from((*velocity, 0.)) * time.delta().as_secs_f32() * PIXELS_PER_METER)
                .clamp(
                    Vec3::new(MIN_X + particle.radius, MIN_Y + particle.radius, f32::MIN),
                    Vec3::new(MAX_X - particle.radius, MAX_Y - particle.radius, f32::MAX),
                );
        });
}

const PIXELS_PER_METER: f32 = 40.;
