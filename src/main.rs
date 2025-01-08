use bevy::{prelude::*};
use kinetics::{KineticsPlugin};

mod draw;
mod kinetics;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            KineticsPlugin,
            draw::DrawPlugin
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) -> () {
    commands.spawn(Camera2d);
}
