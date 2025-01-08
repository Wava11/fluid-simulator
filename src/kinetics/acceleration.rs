use super::velocity::Velocity;
use bevy::prelude::*;

pub struct AccelerationPlugin;

impl Plugin for AccelerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, accelerate_entities);
    }
}

#[derive(Component)]
pub struct Acceleration(pub Vec2);

fn accelerate_entities(time: Res<Time>, mut query: Query<(&Acceleration, &mut Velocity)>) -> () {
    for (Acceleration(acceleration), mut velocity) in query.iter_mut() {
        velocity.0 += acceleration * time.delta().as_secs_f32();
    }
}
