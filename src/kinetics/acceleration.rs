use super::velocity::Velocity;
use bevy::prelude::*;


#[derive(Component)]
pub struct Acceleration(pub Vec2);

pub fn accelerate_entities(time: Res<Time>, mut query: Query<(&Acceleration, &mut Velocity)>) -> () {
    for (Acceleration(acceleration), mut velocity) in query.iter_mut() {
        velocity.0 += acceleration * time.delta().as_secs_f32();
    }
}
