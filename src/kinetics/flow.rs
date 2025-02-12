use crate::fluids::particle::FluidParticle;

use super::{
    collisions::position_hashing,
    forces::Forces,
    mass::Mass,
    utils::{pairs::UnorderedEntitiesPair, velocity::velocity_to_force},
    velocity::Velocity,
};
use bevy::{math::VectorSpace, prelude::*, utils::HashSet};

pub fn apply_flow(
    position_hash_map: Res<position_hashing::PositionHashMap>,
    time: Res<Time>,
    mut query: Query<(&FluidParticle, &Transform, &Mass, &Velocity, &mut Forces)>,
) {
    for (x, col) in position_hash_map.map.iter().enumerate() {
        for (y, cell_entities) in col.iter().enumerate() {
            if cell_entities.len() < 6 {
                continue;
            }
            let neighbouring_cells = calculate_neighbouring_cells(&position_hash_map, x, y);
            let mut flow_direction = Vec2::ZERO;
            let neighbouring_cells_total_entities: f32 =
                neighbouring_cells.iter().map(|x| x.0.len() as f32).sum();
            for (neighbouring_cell_entities, direction) in neighbouring_cells.iter() {
                if neighbouring_cell_entities.len() == 0 {
                    continue;
                }
                let count_diff =
                    cell_entities.len() as f32 - neighbouring_cell_entities.len() as f32;
                if count_diff > 0. && neighbouring_cell_entities.len() > 0 {
                    flow_direction += (count_diff / neighbouring_cells_total_entities) * direction;
                }
            }

            for &entity in cell_entities {
                let (_, transform, mass, _, mut forces) = query.get_mut(entity).unwrap();
                if position_hash_map.cell_idxs_of(transform.translation.xy()) == (x, y) {
                    forces.0.push(
                        velocity_to_force(mass.0, flow_direction, time.delta_secs())
                            * FLOW_COEFFICIENT,
                    );
                }
            }
        }
    }
}

fn calculate_neighbouring_cells<'a>(
    position_hash_map: &'a Res<position_hashing::PositionHashMap>,
    x: usize,
    y: usize,
) -> Vec<(&'a Vec<Entity>, Vec2)> {
    let amount_of_x_cells = position_hash_map.get_amount_of_x_cells();
    let amount_of_y_cells = position_hash_map.get_amount_of_y_cells();

    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .map(|(i, j)| ((x as i32 + i, y as i32 + j), Vec2::new(i as f32, j as f32)))
    .iter()
    .filter(|((x, y), _)| {
        0 <= *x && *x < amount_of_x_cells as i32 && 0 <= *y && *y < amount_of_y_cells as i32
    })
    .map(|&((x, y), direction)| (&position_hash_map.map[x as usize][y as usize], direction))
    .collect()

    // if x < amount_of_x_cells-1 {
    //     result.push((&position_hash_map.map[x + 1][y], Vec2::new(1., 0.)));
    // }
    // if y < amount_of_y_cells-1 {
    //     result.push((&position_hash_map.map[x][y + 1], Vec2::new(0., 1.)));
    // }
    // if x < amount_of_x_cells-1 && y < amount_of_y_cells-1 {
    //     result.push((&position_hash_map.map[x + 1][y + 1], Vec2::new(1., 1.)));
    // }
    // result
}

const FLOW_COEFFICIENT: Vec2 = Vec2::new(1., 20.);
