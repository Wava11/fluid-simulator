use bevy::prelude::*;

use super::bounds::{MAX_X, MAX_Y, MIN_X, MIN_Y};

#[derive(Component, Clone)]
pub struct Velocity(pub Vec2);

pub fn move_entities(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) -> () {
    query
        .par_iter_mut()
        .for_each(|(Velocity(velocity), mut transform)| {
            transform.translation = (transform.translation
                + Vec3::from((*velocity, 0.)) * time.delta().as_secs_f32() * PIXELS_PER_METER)
                .clamp(
                    Vec3::new(MIN_X, MIN_Y, f32::MIN),
                    Vec3::new(MAX_X-1., MAX_Y-1., f32::MAX),
                );
        });
}

const PIXELS_PER_METER: f32 = 40.;
