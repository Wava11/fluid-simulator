mod tests;

use std::time::{Duration, Instant};

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::{
    fluids::particle::FluidParticle,
    kinetics::bounds::{MAX_X, MAX_Y, MIN_X, MIN_Y},
};

pub struct PositionHashingPlugin;

impl Plugin for PositionHashingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_maps)
            .add_systems(FixedUpdate, update_position_map);
    }
}

fn init_maps(mut commands: Commands) {
    commands.insert_resource(EntityPreviousPositionMap {
        map: HashMap::new(),
    });
    commands.insert_resource(PositionHashMap::new(
        6,
        MIN_X as f32,
        MAX_X as f32,
        MIN_Y as f32,
        MAX_Y as f32,
    ));
}

fn update_position_map(
    mut positions_map: ResMut<PositionHashMap>,
    mut entity_previous_position_map: ResMut<EntityPreviousPositionMap>,
    particles_q: Query<(Entity, &Transform, &FluidParticle)>,
) {
    let start = Instant::now();
    positions_map.duration = Duration::from_secs(0);
    particles_q
        .iter()
        .for_each(|(entity, transform, particle)| {
            let prev_position = entity_previous_position_map.map.get(&entity);
            let curr_position = transform.translation.xy();

            if let Some(prev_position) = prev_position {
                positions_map.update(*prev_position, curr_position, particle.radius, entity);
            } else {
                positions_map.insert(curr_position, particle.radius, entity);
            }
            entity_previous_position_map
                .map
                .insert(entity, curr_position);
        });
    println!(
        "total: {:?}, get cells: {:?}",
        start.elapsed(),
        positions_map.duration
    );
}

#[derive(Resource)]
struct EntityPreviousPositionMap {
    map: HashMap<Entity, Vec2>,
}

#[derive(Resource)]
pub struct PositionHashMap {
    pub map: Vec<Vec<Vec<Entity>>>,
    cell_side_size: usize,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    duration: Duration,
}

impl PositionHashMap {
    fn new(
        cell_side_size: usize,
        min_x: f32,
        max_x: f32,
        min_y: f32,
        max_y: f32,
    ) -> PositionHashMap {
        let amount_of_x_cells = (max_x - min_x) as usize / cell_side_size as usize;
        let amount_of_y_cells = (max_y - min_y) as usize / cell_side_size as usize;
        PositionHashMap {
            map: vec![vec![Vec::with_capacity(8); amount_of_y_cells]; amount_of_x_cells],
            cell_side_size: cell_side_size,
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
            duration: Duration::from_secs(0),
        }
    }

    fn update(&mut self, prev_position: Vec2, curr_position: Vec2, radius: f32, entity: Entity) {
        self.cells_idxs_of(prev_position, radius)
            .iter()
            .for_each(|(prev_cell_x, prev_cell_y)| {
                self.map[*prev_cell_x][*prev_cell_y].retain(|e| *e != entity);
            });

        self.cells_idxs_of(curr_position, radius)
            .iter()
            .for_each(|(curr_cell_x, curr_cell_y)| {
                self.map[*curr_cell_x][*curr_cell_y].push(entity);
            });
    }
    fn insert(&mut self, position: Vec2, radius: f32, entity: Entity) {
        self.cells_idxs_of(position, radius)
            .iter()
            .for_each(|(prev_cell_x, prev_cell_y)| {
                self.map[*prev_cell_x][*prev_cell_y].push(entity);
            });
    }

    fn cell_idxs_of(&self, position: Vec2) -> (usize, usize) {
        (
            ((position.x - self.min_x) as usize) / self.cell_side_size,
            ((position.y - self.min_y) as usize) / self.cell_side_size,
        )
    }
    fn cells_idxs_of(&mut self, position: Vec2, radius: f32) -> Vec<(usize, usize)> {
        let start = Instant::now();
        let cell_of_center = (
            ((position.x - self.min_x) as usize) / self.cell_side_size,
            ((position.y - self.min_y) as usize) / self.cell_side_size,
        );
        let mut result: Vec<(usize, usize)> = Vec::with_capacity(9);
        result.push(cell_of_center);

        let cell_border = self.cell_idxs_to_borders(cell_of_center.0, cell_of_center.1);

        let intersects_left = position.x - radius <= cell_border.left;
        let intersects_upper = position.y + radius >= cell_border.up;
        let intersects_right = position.x + radius >= cell_border.right;
        let intersects_lower = position.y - radius <= cell_border.down;

        if intersects_left {
            result.push((cell_of_center.0 - 1, cell_of_center.1));
            if intersects_upper {
                result.push((cell_of_center.0 - 1, cell_of_center.1 + 1));
            }
        }
        if intersects_upper {
            result.push((cell_of_center.0, cell_of_center.1 + 1));
            if intersects_right {
                result.push((cell_of_center.0 + 1, cell_of_center.1 + 1));
            }
        }
        if intersects_right {
            result.push((cell_of_center.0 + 1, cell_of_center.1));
            if intersects_lower {
                result.push((cell_of_center.0 + 1, cell_of_center.1 - 1));
            }
        }
        if intersects_lower {
            result.push((cell_of_center.0, cell_of_center.1 - 1));
            if intersects_left {
                result.push((cell_of_center.0 - 1, cell_of_center.1 - 1));
            }
        }
        let amount_of_x_cells = (self.max_x - self.min_x) as usize / self.cell_side_size as usize;
        let amount_of_y_cells = (self.max_y - self.min_y) as usize / self.cell_side_size as usize;



        let result = result
            .iter()
            .filter(|(x, y)| *x <= amount_of_x_cells - 1 && *y <= amount_of_y_cells - 1)
            .map(|x| *x)
            .collect();
        self.duration += start.elapsed();
        result
    }

    fn cell_idxs_to_borders(&self, cell_x: usize, cell_y: usize) -> Borders {
        Borders {
            left: (cell_x * self.cell_side_size) as f32 + self.min_x,
            up: ((cell_y + 1) * self.cell_side_size) as f32 + self.min_y,
            right: ((cell_x + 1) * self.cell_side_size) as f32 + self.min_x,
            down: (cell_y * self.cell_side_size) as f32 + self.min_y,
        }
    }
}

#[derive(Debug)]
struct Borders {
    left: f32,
    up: f32,
    right: f32,
    down: f32,
}
