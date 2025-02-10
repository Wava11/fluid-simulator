use crate::{
    fluids::particle::FluidParticle,
    kinetics::{
        acceleration::Acceleration,
        bounds::{MAX_X, MAX_Y, MIN_X, MIN_Y},
        forces::Forces,
        mass::Mass,
        velocity::Velocity,
    },
};
use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

pub struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_circle)
            .insert_resource(SpawnTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
        // .add_systems(
        //     Update,
        //     continuously_spawn.run_if(on_timer(Duration::from_millis(10))),
        // );
    }
}

fn draw_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = StdRng::seed_from_u64(40);

    let p1 = FluidParticle {
        radius: 3.,
        restitution_coeff: 0.97,
    };
    for _ in 1..3000 {
        spawn_random_particle(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut rng,
            p1,
            Mass(1.),
        );
    }
}

fn spawn_random_particle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    rng: &mut StdRng,
    p1: FluidParticle,
    mass: Mass,
) {
    commands.spawn((
        p1,
        Mesh2d(meshes.add(p1)),
        MeshMaterial2d(materials.add(Color::hsl(rng.gen_range(0.0..360.), 0.95, 0.7))),
        Transform::from_xyz(
            rng.gen_range(MIN_X..MAX_X),
            rng.gen_range(MIN_Y..MAX_Y),
            0.,
        ),
        // Velocity(Vec2::ZERO),
        Velocity(Vec2::new(rng.gen_range(-5.0..5.), rng.gen_range(-5.0..5.))),
        Acceleration(Vec2::new(0., 0.)),
        mass,
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
        radius: 3.,
        restitution_coeff: 0.95,
    };
    spawn_random_particle(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut rng,
        p1,
        Mass(1.),
    );
}

#[derive(Resource)]
struct SpawnTimer(Timer);
