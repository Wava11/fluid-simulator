
use std::time::Duration;

use crate::{
    fluids::particle::FluidParticle,
    kinetics::{acceleration::Acceleration, forces::Forces, mass::Mass, velocity::Velocity},
};
use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::{rngs::StdRng, Rng, SeedableRng};

pub struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, draw_circle)
        .insert_resource(SpawnTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
        // .add_systems(Update, continuously_spawn.run_if(on_timer(Duration::from_millis(10))));
    }
}

fn draw_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = StdRng::seed_from_u64(40);

    let p1 = FluidParticle {
        radius: 4.,
        restitution_coeff: 0.97,
    };
    for _ in 1..1500 {
        spawn_random_particle(&mut commands, &mut meshes, &mut materials, &mut rng, p1);
    }

    // commands.spawn((
    //     p1,
    //     Mesh2d(meshes.add(p1)),
    //     MeshMaterial2d(materials.add(Color::hsl(rng.gen_range(0.0..360.), 0.95, 0.7))),
    //     Transform::from_xyz(
    //         -150.,0.,
    //         0.,
    //     ),
    //     Velocity(Vec2::new(6.,0.)),
    //     Acceleration(Vec2::new(0., 0.)),
    //     Mass(1.),
    //     Forces(vec![]),
    // ));
    // commands.spawn((
    //     p1,
    //     Mesh2d(meshes.add(p1)),
    //     MeshMaterial2d(materials.add(Color::hsl(rng.gen_range(0.0..360.), 0.95, 0.7))),
    //     Transform::from_xyz(
    //         150.,0.,
    //         0.,
    //     ),
    //     Velocity(Vec2::new(-6.,0.)),
    //     Acceleration(Vec2::new(0., 0.)),
    //     Mass(1.),
    //     Forces(vec![]),
    // ));
}

fn spawn_random_particle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    rng: &mut StdRng,
    p1: FluidParticle,
) {
    commands.spawn((
        p1,
        Mesh2d(meshes.add(p1)),
        MeshMaterial2d(materials.add(Color::hsl(rng.gen_range(0.0..360.), 0.95, 0.7))),
        Transform::from_xyz(
            rng.gen_range(-300.0..300.0),
            rng.gen_range(-150.0..150.0),
            0 as f32,
        ),
        Velocity(Vec2::new(rng.gen_range(-5.0..5.), rng.gen_range(-5.0..5.))),
        Acceleration(Vec2::new(0., 0.)),
        Mass(1.),
        Forces(vec![]),
    ));
}

fn continuously_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = StdRng::seed_from_u64(40);
    let p1 = FluidParticle {
        radius: 4.,
        restitution_coeff: 0.95,
    };
    spawn_random_particle(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut rng,
        p1,
    );
}

#[derive(Resource)]
struct SpawnTimer(Timer);
