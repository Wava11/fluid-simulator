use bevy::ecs::schedule::ExecutorKind;
use bevy::prelude::*;
use kinetics::KineticsPlugin;

mod draw;
mod fluids;
mod performance_monitor;
mod kinetics;
mod particles_counter;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            KineticsPlugin,
            draw::DrawPlugin,
            performance_monitor::PerformanceMonitorPlugin,
            particles_counter::ParticlesCounterPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .insert_resource(Time::<Fixed>::from_hz(144.))
        .run();
}

fn spawn_camera(mut commands: Commands) -> () {
    commands.spawn(Camera2d);
}
