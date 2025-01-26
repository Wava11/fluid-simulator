use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec2);

pub fn move_entities(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) -> () {
    for (Velocity(velocity), mut transform) in query.iter_mut() {
        transform.translation +=
            Vec3::from((*velocity, 0.)) * time.delta().as_secs_f32() * PIXELS_PER_METER;
    }
}

const PIXELS_PER_METER: f32 = 40.;
