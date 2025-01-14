use core::f32;

use bevy::{
    math::{NormedVectorSpace, VectorSpace},
    prelude::*,
};

use crate::{
    fluids::particle::FluidParticle,
    kinetics::{
        acceleration::{self, Acceleration},
        forces::{apply_forces, Forces},
        mass::Mass,
        velocity::{self, Velocity},
    },
};

pub struct BoundsPlugin;

impl Plugin for BoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, enforce_bounds.before(apply_forces));
    }
}

pub fn enforce_bounds(
    time: Res<Time>,
    mut q_particles: Query<(
        &FluidParticle,
        &mut Transform,
        &Mass,
        &Velocity,
        &mut Forces,
    )>,
) -> () {
    for (particle, mut transform, mass, velocity, mut forces) in q_particles.iter_mut() {
        let particle_center = transform.translation.xy();

        let collision_force =
            calculate_collision_force(particle_center, &particle, mass, velocity, &time);

        forces.0.push(collision_force);

        if collision_force.x != 0. || collision_force.y != 0. {
            transform.translation = transform.translation.clamp(
                Vec3::new(
                    MIN_X + 2.*particle.radius,
                    MIN_Y + 2.*particle.radius,
                    f32::MIN,
                ),
                Vec3::new(
                    MAX_X - 2.*particle.radius,
                    MAX_Y - 2.*particle.radius,
                    f32::MAX,
                ),
            );
        }
    }
}

fn calculate_collision_force(
    particle_center: Vec2,
    particle: &FluidParticle,
    Mass(mass): &Mass,
    Velocity(velocity): &Velocity,
    time: &Res<Time>,
) -> Vec2 {
    if time.delta().as_secs_f32() == 0. {
        return Vec2::ZERO;
    }
    let particle_left = particle_center.x - particle.radius;
    let particle_right = particle_center.x + particle.radius;
    let particle_down = particle_center.y - particle.radius;
    let particle_up = particle_center.y + particle.radius;

    let mut new_velocity = *velocity;
    if (particle_left < MIN_X) || (particle_right>MAX_X){
        new_velocity.x = -velocity.x;
    }
    if (particle_down < MIN_Y) || (particle_up>MAX_Y) {
        new_velocity.y = -velocity.y;
    }

    let impulse = mass * (new_velocity - velocity);

    impulse*particle.restitution_coeff / time.delta().as_secs_f32()
}


const MIN_X: f32 = -400.;
const MAX_X: f32 = 400.;
const MIN_Y: f32 = -200.;
const MAX_Y: f32 = 200.;
