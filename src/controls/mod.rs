pub mod toggle_gravity;

use bevy::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_controls)
            .add_systems(Update, toggle_gravity::toggle_gravity);
    }
}

fn init_controls(mut commands: Commands) {
    commands.insert_resource(toggle_gravity::GravityToggled(true));
}
