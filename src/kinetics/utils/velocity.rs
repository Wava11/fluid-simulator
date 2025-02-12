use bevy::math::Vec2;

pub fn velocity_to_force(mass: f32, velocity: Vec2, dt_secs: f32) -> Vec2 {
    mass * velocity / dt_secs
}
