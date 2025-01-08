use bevy::prelude::*;

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_entities);
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

fn move_entities(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) -> () {
    for (Velocity(velocity), mut transform) in query.iter_mut() {
        transform.translation += Vec3::from((*velocity, 0.)) * time.delta().as_secs_f32() * PIXELS_PER_METER;
    }
}

const PIXELS_PER_METER: f32 = 40.;