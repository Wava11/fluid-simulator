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
        app.insert_resource(SpawnTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
            .add_systems(Startup, draw_circle);
            // .add_systems(Update, continuously_spawn.run_if(on_timer(Duration::from_millis(100))));
    }
}

fn draw_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = StdRng::seed_from_u64(40);

    let p1 = FluidParticle { radius: 4. };
    for i in 1..50000 {
        spawn_random_particle(&mut commands, &mut meshes, &mut materials, &mut rng, p1, i);
    }
}

fn spawn_random_particle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    rng: &mut StdRng,
    p1: FluidParticle,
    i: i32,
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
        Velocity(Vec2::new(0., 0.)),
        // Velocity(Vec2::new(10., 10.)),
        Acceleration(Vec2::new(0., 0.)),
        Mass(1.),
        // Mass(rng.gen_range(0.1..2.)),
        // Forces(vec![Vec2::new(rng.gen_range(-20.0..20.), rng.gen_range(0.0..30.))]),
        // Forces(vec![]),
    ));
}

fn continuously_spawn(
    // time: Res<Time>,
    // mut timer: SpawnTimer,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // if timer.0.tick(time.delta()).just_finished() {
        let mut rng = StdRng::seed_from_u64(40);
        let p1 = FluidParticle { radius: 4. };
        spawn_random_particle(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut rng,
            p1,
            0
            // time.elapsed().as_millis() as i32,
        );
    // }
}

#[derive(Resource)]
struct SpawnTimer(Timer);
