use bevy::prelude::*;

pub fn toggle_gravity(mut gravity_toggled: ResMut<GravityToggled>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyG) {
        gravity_toggled.0 = !gravity_toggled.0;
    }
}

#[derive(Resource)]
pub struct GravityToggled(pub bool);
