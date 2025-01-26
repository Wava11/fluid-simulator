use crate::fluids::particle::FluidParticle;

use super::{forces::Forces, mass::Mass, velocity::Velocity};
use bevy::prelude::*;

pub fn apply_collisions(
    time: Res<Time>,
    mut query: Query<(
        &FluidParticle,
        &mut Transform,
        &Mass,
        &Velocity,
        &mut Forces,
    )>,
) {
    let mut pairs = query.iter_combinations_mut();
    while let Some(
        [(particle1, mut transform1, mass1, velocity1, mut forces1), (particle2, mut transform2, mass2, velocity2, mut forces2)],
    ) = pairs.fetch_next()
    {
        let p1_center = transform1.translation.xy();
        let p2_center = transform2.translation.xy();
        let collidable_p1 = CollidableParticle {
            mass: mass1,
            particle: particle1,
            particle_center: p1_center,
            velocity: velocity1,
        };
        let collidable_p2 = CollidableParticle {
            mass: mass2,
            particle: particle2,
            particle_center: p2_center,
            velocity: velocity2,
        };
        let (force1, force2) = calculate_collision_forces(collidable_p1, collidable_p2, &time);
        if force1 != Vec2::ZERO {
            forces1.0.push(force1);
        }
        if force2 != Vec2::ZERO {
            forces2.0.push(force2);
        }

        if collidable_p1.is_colliding(&collidable_p2) {
            translate_particles_to_not_intersect(
                *particle1,
                *particle2,
                &mut transform1,
                &mut transform2,
                *mass1,
                *mass2,
            );
        }
    }
}

fn translate_particles_to_not_intersect(
    particle1: FluidParticle,
    particle2: FluidParticle,
    transform1: &mut Transform,
    transform2: &mut Transform,
    mass1: Mass,
    mass2: Mass,
) -> () {
    let center1 = transform1.translation.xy();
    let center2 = transform2.translation.xy();
    let collision_line = (center2 - center1).normalize();
    let translation_vector =
        (particle1.radius + particle2.radius - center1.distance(center2)) * collision_line;

    transform1.translation +=
        (-(translation_vector * mass2.0) / (mass1.0 + mass2.0)).extend(transform1.translation.z);
    transform2.translation +=
        ((translation_vector * mass1.0) / (mass1.0 + mass2.0)).extend(transform2.translation.z);
}

fn calculate_collision_forces(
    p1: CollidableParticle,
    p2: CollidableParticle,
    time: &Res<Time>,
) -> (Vec2, Vec2) {
    if time.delta().as_secs_f32() == 0. {
        return (Vec2::ZERO, Vec2::ZERO);
    }

    if p1.is_colliding(&p2) {
        let (p1_final_velocity, p2_final_velocity) = p1.velocities_after_collision_with(&p2);

        let impulse1 = p1.mass.0 * (p1_final_velocity - p1.velocity.0);
        let impulse2 = p2.mass.0 * (p2_final_velocity - p2.velocity.0);

        return (
            impulse1 * p1.particle.restitution_coeff / time.delta().as_secs_f32(),
            impulse2 * p2.particle.restitution_coeff / time.delta().as_secs_f32(),
        );
    }

    (Vec2::ZERO, Vec2::ZERO)
}

#[derive(Clone, Copy)]
struct CollidableParticle<'a> {
    particle_center: Vec2,
    particle: &'a FluidParticle,
    mass: &'a Mass,
    velocity: &'a Velocity,
}

impl<'a> CollidableParticle<'a> {
    fn is_colliding(&self, other: &CollidableParticle) -> bool {
        self.particle_center.distance(other.particle_center)
            <= self.particle.radius + other.particle.radius
    }

    fn velocities_after_collision_with(&self, other: &CollidableParticle) -> (Vec2, Vec2) {
        let collision_line = (other.particle_center - self.particle_center).normalize();

        if self.is_moving_together_towards(other) {
            return (self.velocity.0, other.velocity.0);
        }

        let self_parallel_velocity = self.velocity.0.dot(collision_line) * collision_line;
        let self_perpendicular_velocity = self.velocity.0 - self_parallel_velocity;

        let other_parallel_velocity = other.velocity.0.dot(collision_line) * collision_line;
        let other_perpendicular_velocity = other.velocity.0 - other_parallel_velocity;

        let self_final_parallel_velocity = (self_parallel_velocity * (self.mass.0 - other.mass.0)
            + (1. + self.particle.restitution_coeff) * other.mass.0 * other_parallel_velocity)
            / (self.mass.0 + other.mass.0);
        let other_final_parallel_velocity = (other_parallel_velocity
            * (other.mass.0 - self.mass.0)
            + (1. + other.particle.restitution_coeff) * self.mass.0 * self_parallel_velocity)
            / (other.mass.0 + self.mass.0);

        (
            self_final_parallel_velocity + self_perpendicular_velocity,
            other_final_parallel_velocity + other_perpendicular_velocity,
        )
    }

    fn is_moving_together_towards(&self, other: &CollidableParticle) -> bool {
        let collision_line = (other.particle_center - self.particle_center).normalize();

        let relative_velocity = other.velocity.0 - self.velocity.0;
        let relative_velocity_projected_onto_collision_line = relative_velocity.dot(collision_line);
        relative_velocity_projected_onto_collision_line > 0.
    }
}
