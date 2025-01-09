use bevy::{math::{NormedVectorSpace, VectorSpace}, prelude::*};

use crate::{fluids::particle::FluidParticle, kinetics::{acceleration::{self, Acceleration}, forces::Forces, velocity::{self, Velocity}}};

pub struct BoundsPlugin;

impl Plugin for BoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enforce_bounds);
    }
}

pub fn enforce_bounds(
    mut q_particles: Query<(&FluidParticle, &mut Transform, &mut Velocity)>,
    // q_camera: Query<(&Camera, &GlobalTransform)>,
) -> () {
    // let (camera,camera_transform) = q_camera.single();

    for (particle, mut transform, mut velocity) in q_particles.iter_mut() {
        let particle_center = transform.translation.xy();

        velocity.0 = new_velocity(particle_center, particle, &velocity);

        transform.translation = transform.translation.clamp(
            Vec3::new(MIN_X - particle.radius/4., MIN_Y - particle.radius/4., f32::MIN),
            Vec3::new(MAX_X + particle.radius/4., MAX_Y + particle.radius/4., f32::MAX),
        );
    }
}

fn new_velocity(particle_center: Vec2, particle: &FluidParticle, velocity: &Velocity) -> Vec2 {
    let particle_left = particle_center.x - particle.radius;
    let particle_right = particle_center.x + particle.radius;
    let particle_down = particle_center.y - particle.radius;
    let particle_up = particle_center.y + particle.radius;

    let mut force = Vec2::new(0., 0.);
    if particle_left < MIN_X {
        force.x += (MIN_X - particle_left).abs().clamp(0., 200.);
    }
    if particle_right > MAX_X {
        force.x -= (MAX_X - particle_right).abs().clamp(0., 200.);
    }
    if particle_down < MIN_Y {
        force.y += (MIN_Y - particle_down).abs().clamp(0., 200.);
    }
    if particle_up > MAX_Y {
        force.y -= (MAX_Y - particle_up).abs().clamp(0., 200.);
    }

    let mut new_velocity= velocity.0;

    if force.x > 0. {
        new_velocity.x = velocity.0.x.abs();
    }
    if force.x < 0. {
        new_velocity.x = -velocity.0.x.abs();
    }
    if force.y > 0. {
        new_velocity.y = velocity.0.y.abs();
    }
    if force.y < 0. {
        new_velocity.y = -velocity.0.y.abs();
    }
    new_velocity
    // new_sum_of_forces

    // if force.x != 0. || force.y != 0. {
    //     sum_of_forces + sum_of_forces.norm() * (force + Vec2::ONE)
    // } else {
    //     sum_of_forces
    // }
}

const MIN_X: f32 = -400.;
const MAX_X: f32 = 400.;
const MIN_Y: f32 = -200.;
const MAX_Y: f32 = 200.;
