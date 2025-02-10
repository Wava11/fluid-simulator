use std::{hash::Hash, ops::Add, sync::{Arc, Mutex}, time::Instant};

use super::{forces::Forces, mass::Mass, velocity::Velocity};
use crate::{fluids::particle::FluidParticle, performance_monitor};
use bevy::{
    ecs::query,
    math::VectorSpace,
    prelude::*,
    tasks::{ComputeTaskPool, ParallelSlice},
    utils::HashSet,
};

pub mod position_hashing;

pub fn apply_collisions(
    mut collision_detection_monitor: ResMut<performance_monitor::CollisionDetectionMonitor>,
    position_hash_map: Res<position_hashing::PositionHashMap>,
    time: Res<Time>,
    mut query: Query<(
        &FluidParticle,
        &mut Transform,
        &Mass,
        &Velocity,
        &mut Forces,
    )>,
) {
    let start = Instant::now();
    
    let resolutions = position_hash_map.map.par_splat_map(ComputeTaskPool::get(),None, |_,slice| {
        let mut amount_of_checked_pairs=0;
        let mut amount_of_colliding_pairs=0;

        let mut checked_pairs = HashSet::<UnorderedEntitiesPair>::new() ;
        let mut resolutions:Vec<CollisionResolution> = vec![];
        for row_sets in slice {
            for cell_set in row_sets.iter() {
                for entity1 in cell_set {
                    for &entity2 in cell_set.iter() {
                        amount_of_checked_pairs += 1;
                        let unordered_entities_pair = UnorderedEntitiesPair::new(*entity1, entity2);
                        if *entity1 == entity2 || checked_pairs.contains(&unordered_entities_pair)  {
                            continue;
                        }
                        amount_of_colliding_pairs += 1;
                        let query_result = query.get_many([*entity1, entity2]);
                        if let Ok(
                            [(particle1, transform1, mass1, velocity1, forces1), (particle2,  transform2, mass2, velocity2, forces2)],
                        ) = query_result
                        {
                            let collidable_p1 = CollidableParticle {
                                mass: mass1,
                                particle: particle1,
                                particle_center: transform1.translation.xy(),
                                velocity: velocity1,
                            };
                            let collidable_p2 = CollidableParticle {
                                mass: mass2,
                                particle: particle2,
                                particle_center: transform2.translation.xy(),
                                velocity: velocity2,
                            };
                            if collidable_p1.is_colliding(&collidable_p2) {
                                let (force1, force2) =
                                    calculate_collision_forces_of_intersecting_particles(
                                        &time,
                                        collidable_p1,
                                        collidable_p2,
                                    );
    
                                let (t1, t2) = calculate_new_centers_for_intersecting_particles(
                                    collidable_p1,
                                    collidable_p2,
                                );

                                resolutions.push(CollisionResolution::new(*entity1,entity2,force1,t1));
                                resolutions.push(CollisionResolution::new(entity2,*entity1,force2,t2));

                            }
                        }
                        checked_pairs.insert(unordered_entities_pair);
                    }
                }
            }
        }
        (resolutions, amount_of_checked_pairs,amount_of_colliding_pairs)
    });
    let amount_of_checked_pairs = resolutions.iter().map(|(_,amount_of_checked_pairs,_)|amount_of_checked_pairs).sum();
    let amount_of_colliding_pairs = resolutions.iter().map(|(_,_,amount_of_colliding_pairs)|amount_of_colliding_pairs).sum();
    let resolutions: HashSet<CollisionResolution> = resolutions.iter().map(|(resolutions,_,_)|resolutions).flatten().map(|x|*x).collect();
    for resolution in resolutions {
        if let Ok((_,mut transform,_,_,mut forces )) = query.get_mut(resolution.entity) {
            if resolution.new_force != Vec2::ZERO {
                forces.0.push(resolution.new_force);
            }
            transform.translation = resolution.new_position.extend(transform.translation.z);
        }
    }

    

    collision_detection_monitor.checked_pairs = amount_of_checked_pairs;
    collision_detection_monitor.colliding_pairs = amount_of_colliding_pairs;
    collision_detection_monitor.duration = start.elapsed();
}

pub fn apply_collisions_single_threaded(
    mut collision_detection_monitor: ResMut<performance_monitor::CollisionDetectionMonitor>,
    position_hash_map: Res<position_hashing::PositionHashMap>,
    time: Res<Time>,
    mut query: Query<(
        &FluidParticle,
        &mut Transform,
        &Mass,
        &Velocity,
        &mut Forces,
    )>,
) {
    let start = Instant::now();

    let mut total = 0;
    let mut checked_pairs = HashSet::<UnorderedEntitiesPair>::new();
    let mut amount_of_colliding_pairs = 0;

    for row_sets in position_hash_map.map.iter() {
        for cell_set in row_sets.iter() {
            for entity1 in cell_set {
                for &entity2 in cell_set.iter() {
                    total += 1;
                    let unordered_entities_pair = UnorderedEntitiesPair::new(*entity1, entity2);
                    if *entity1 == entity2 || checked_pairs.contains(&unordered_entities_pair) {
                        continue;
                    }
                    let query_result = query.get_many_mut([*entity1, entity2]);
                    if let Ok(
                        [(particle1, mut transform1, mass1, velocity1, mut forces1), (particle2, mut transform2, mass2, velocity2, mut forces2)],
                    ) = query_result
                    {
                        let collidable_p1 = CollidableParticle {
                            mass: mass1,
                            particle: particle1,
                            particle_center: transform1.translation.xy(),
                            velocity: velocity1,
                        };
                        let collidable_p2 = CollidableParticle {
                            mass: mass2,
                            particle: particle2,
                            particle_center: transform2.translation.xy(),
                            velocity: velocity2,
                        };
                        if collidable_p1.is_colliding(&collidable_p2) {
                            amount_of_colliding_pairs += 1;
                            let (force1, force2) =
                                calculate_collision_forces_of_intersecting_particles(
                                    &time,
                                    collidable_p1,
                                    collidable_p2,
                                );

                            if force1 != Vec2::ZERO {
                                forces1.0.push(force1);
                            }
                            if force2 != Vec2::ZERO {
                                forces2.0.push(force2);
                            }

                            let (t1, t2) = calculate_new_centers_for_intersecting_particles(
                                collidable_p1,
                                collidable_p2,
                            );
                            transform1.translation = t1.extend(transform1.translation.z);
                            transform2.translation = t2.extend(transform2.translation.z);
                        }
                    }
                    checked_pairs.insert(unordered_entities_pair);
                }
            }
        }
    }

    collision_detection_monitor.duration = start.elapsed();
    collision_detection_monitor.checked_pairs = checked_pairs.len();
    collision_detection_monitor.colliding_pairs = amount_of_colliding_pairs;
}

fn calculate_collision_forces_of_intersecting_particles(
    time: &Res<'_, Time>,
    collidable_p1: CollidableParticle,
    collidable_p2: CollidableParticle,
) -> (Vec2, Vec2) {
    if time.delta().as_secs_f32() == 0. {
        return (Vec2::ZERO, Vec2::ZERO);
    }

    let (p1_final_velocity, p2_final_velocity) =
        collidable_p1.velocities_after_collision_with(&collidable_p2);

    let impulse1 = collidable_p1.mass.0 * (p1_final_velocity - collidable_p1.velocity.0);
    let impulse2 = collidable_p2.mass.0 * (p2_final_velocity - collidable_p2.velocity.0);

    return (
        impulse1 * collidable_p1.particle.restitution_coeff / time.delta().as_secs_f32(),
        impulse2 * collidable_p2.particle.restitution_coeff / time.delta().as_secs_f32(),
    );
}

fn calculate_new_centers_for_intersecting_particles(
    p1: CollidableParticle,
    p2: CollidableParticle,
) -> (Vec2, Vec2) {
    let center1 = p1.particle_center;
    let center2 = p2.particle_center;
    let collision_line = (center2 - center1).normalize();
    let translation_vector =
        (p1.particle.radius + p2.particle.radius - center1.distance(center2)) * collision_line;

    (
        center1 + (-(translation_vector * p2.mass.0) / (p1.mass.0 + p2.mass.0)),
        center2 + ((translation_vector * p1.mass.0) / (p1.mass.0 + p2.mass.0)),
    )
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

#[derive(Eq, Hash, PartialEq)]
struct UnorderedEntitiesPair {
    entities: (Entity, Entity),
}

impl UnorderedEntitiesPair {
    fn new(e1: Entity, e2: Entity) -> UnorderedEntitiesPair {
        if e1 < e2 {
            UnorderedEntitiesPair { entities: (e1, e2) }
        } else {
            UnorderedEntitiesPair { entities: (e2, e1) }
        }
    }
}

#[derive(Copy,Clone)]
struct CollisionResolution {
    entity: Entity,
    new_force: Vec2,
    new_position: Vec2,
    pair: (Entity,Entity)
}
impl CollisionResolution{
    fn new (entity:Entity, other_entity:Entity, new_force: Vec2, new_position:Vec2) -> CollisionResolution {
        Self{entity,new_force,new_position,pair: (entity.min(other_entity), entity.max(other_entity))}
    }
}
impl Hash for CollisionResolution {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.entity.hash(state);
        self.pair.hash(state);
    }
}
impl Eq for CollisionResolution {
    
}
impl PartialEq for CollisionResolution {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity && self.pair == other.pair
    }
}
