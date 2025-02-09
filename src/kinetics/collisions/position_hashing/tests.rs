#[cfg(test)]
mod PositionHashMapTests {

    use super::super::*;
    const CELL_SIZE: usize = 10;
    const MIN_X: f32 = -200.;
    const MAX_X: f32 = 200.;
    const MIN_Y: f32 = -200.;
    const MAX_Y: f32 = 200.;
    #[test]
    fn particle_is_only_in_the_cell_of_its_center() {
        let mut map = PositionHashMap::new(CELL_SIZE, MIN_X, MAX_X, MIN_Y, MAX_Y);
        let center = Vec2::new(5., 5.);
        let radius = 4.;
        map.insert(center, radius, Entity::from_raw(0));
        assert_eq!(map.cells_idxs_of(center, radius).len(), 1);
        assert_eq!(map.cells_idxs_of(center, radius)[0], (20, 20));
    }

    #[test]
    fn particle_intersects_left_and_upper_and_left_upper_cells() {
        let mut map = PositionHashMap::new(CELL_SIZE, MIN_X, MAX_X, MIN_Y, MAX_Y);
        let center = Vec2::new(2., 9.);
        let radius = 4.;
        map.insert(center, radius, Entity::from_raw(0));
        assert_eq!(map.cells_idxs_of(center, radius).len(), 4);
        assert!(map.cells_idxs_of(center, radius).contains(&(20, 20)));
        assert!(map.cells_idxs_of(center, radius).contains(&(20, 21)));
        assert!(map.cells_idxs_of(center, radius).contains(&(19, 20)));
        assert!(map.cells_idxs_of(center, radius).contains(&(19, 21)));
    }
}
