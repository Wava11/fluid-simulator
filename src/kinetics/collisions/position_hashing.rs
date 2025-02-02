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
        40,
        MIN_X as f32,
        MAX_X as f32,
        MIN_Y as f32,
        MAX_Y as f32,
    ));
}

fn update_position_map(
    mut positions_map: ResMut<PositionHashMap>,
    mut entity_previous_position_map: ResMut<EntityPreviousPositionMap>,
    particles_q: Query<(Entity, &Transform), With<FluidParticle>>,
) {
    // let start = std::time::Instant::now();
    particles_q.iter().for_each(|(entity, transform)| {
        let prev_position = entity_previous_position_map.map.get(&entity);
        let curr_position = transform.translation.xy();
        if let Some(prev_position) = prev_position {
            positions_map.update(*prev_position, curr_position, entity);
        } else {
            positions_map.insert(curr_position, entity);
        }
        entity_previous_position_map
            .map
            .insert(entity, curr_position);
    });
    // let duration = start.elapsed();
    // println!("map update took {:?}", duration);
}

#[derive(Resource)]
struct EntityPreviousPositionMap {
    map: HashMap<Entity, Vec2>,
}

#[derive(Resource)]
pub struct PositionHashMap {
    pub map: Vec<Vec<HashSet<Entity>>>,
    cell_side_size: usize,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
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
            map: vec![vec![HashSet::new(); amount_of_y_cells]; amount_of_x_cells],
            cell_side_size: cell_side_size,
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
        }
    }

    fn update(&mut self, prev_position: Vec2, curr_position: Vec2, entity: Entity) {
        let (prev_cell_x, prev_cell_y) = self.cell_idxs_of(prev_position);
        let (curr_cell_x, curr_cell_y) = self.cell_idxs_of(curr_position);
        if curr_cell_y >= 10 {
            println!("{:?}", curr_position);
        }
        self.map[prev_cell_x][prev_cell_y].remove(&entity);
        self.map[curr_cell_x][curr_cell_y].insert(entity);
    }
    fn insert(&mut self, position: Vec2, entity: Entity) {
        let (cell_x, cell_y) = self.cell_idxs_of(position);
        self.map[cell_x][cell_y].remove(&entity);
    }

    fn cell_idxs_of(&self, position: Vec2) -> (usize, usize) {
        // println!("position: {:?}", position);
        (
            ((position.x - self.min_x) as usize) / self.cell_side_size,
            ((position.y - self.min_y) as usize) / self.cell_side_size,
        )
    }

    pub fn neighbouring_cells_particles(&self, x: usize, y: usize) -> HashSet<Entity> {
        vec![
            self.map.get(x - 1).map(|row| row.get(y - 1)).flatten(),
            self.map.get(x - 1).map(|row| row.get(y)).flatten(),
            self.map.get(x).map(|row| row.get(y - 1)).flatten(),
            self.map.get(x).map(|row| row.get(y)).flatten(),
            self.map.get(x + 1).map(|row| row.get(y + 1)).flatten(),
            self.map.get(x + 1).map(|row| row.get(y)).flatten(),
            self.map.get(x).map(|row| row.get(y + 1)).flatten(),
            self.map.get(x).map(|row| row.get(y)).flatten(),
        ]
        .into_iter()
        .flatten()
        .fold(HashSet::<Entity>::new(), |acc, curr| {
            acc.union(curr).map(|x| *x).collect::<HashSet<Entity>>()
        })
    }
}
