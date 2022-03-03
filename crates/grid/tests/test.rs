use std::fmt::Debug;

use grid::{Area, Grid};

/* ---------- */

#[derive(Debug, Default, Clone, Copy)]
struct Witness {
    id: usize,
    status: bool,
}

impl Witness {
    fn set_status(&mut self) {
        self.status = true;
    }

    fn unset_status(&mut self) {
        self.status = false;
    }
}

impl PartialEq for Witness {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status
    }
}

/* ---------- */

const fn coord_from_idx(idx: usize, width: usize) -> (usize, usize) {
    (idx % width, idx / width)
}

fn test_value<T: Default + PartialEq + Debug>(grid: Grid<T>, val: T, width: usize, height: usize) {
    for idx in 0..width * height {
        let (x, y) = coord_from_idx(idx, width);

        let val_idx = grid.get_at_index(idx);
        assert_eq!(val_idx, Some(&val));

        let val_coord = grid.get(x, y);
        assert_eq!(val_coord, Some(&val));
    }
}

const fn is_contained_in_area((x, y): (usize, usize), area: Area) -> bool {
    x >= area.left && x <= area.right && y >= area.top && y <= area.bottom
}

/* ---------- */

#[test]
fn test_get() {
    let width = 10;
    let height = 5;

    test_value(Grid::new_filled(width, height, 1), 1, width, height);
    test_value(
        Grid::new_with_default(width, height),
        i32::default(),
        width,
        height,
    )
}

/* ---------- */

#[test]
fn test_get_mut() {
    let width = 10;
    let height = 5;
    let mut test = Grid::new_filled(width, height, 1);

    for idx in 0..width * height {
        if let Some(val) = test.get_at_index_mut(idx) {
            *val = 2
        }
    }

    test_value(test, 2, width, height)
}

/* ---------- */

#[test]
fn test_iter() {
    let grid = Grid::new_square_filled(10, 1);

    assert_eq!(grid.iter().count(), 100);
    grid.iter().for_each(|val| assert_eq!(val, &1));
    assert_eq!(grid.iter().sum::<i32>(), 100);
}

/* ---------- */

#[test]
fn test_iter_over() {
    let grid = Grid::new_square_filled(10, 0);
    assert_eq!(grid.iter_over(Area::new(0, 0, 0, 0)).count(), 1);
    assert_eq!(grid.iter_over(Area::new(0, 0, 1, 1)).count(), 4);
    assert_eq!(grid.iter_over(Area::new(1, 1, 2, 2)).count(), 4);
    assert_eq!(grid.iter_over(Area::new(0, 0, 9, 9)).count(), 100);
    assert_eq!(grid.iter_over(Area::new(10, 10, 11, 11)).count(), 0);
    assert_eq!(grid.iter_over(Area::new(9, 9, 9, 9)).count(), 1);
    assert_eq!(grid.iter_over(Area::new(5, 6, 6, 6)).count(), 2);
    assert_eq!(grid.iter_over(Area::new(9, 9, 10, 9)).count(), 1);
}

/* ---------- */

#[test]
fn test_iter_mut() {
    let mut grid = Grid::new_square_filled(10, 0);

    assert_eq!(grid.iter_mut().count(), 100);
    assert_eq!(grid.iter().sum::<i32>(), 0);

    grid.iter_mut().for_each(|val| *val = 10);
    assert_eq!(grid.iter().sum::<i32>(), 1000);

    grid.iter_mut()
        .enumerate()
        .for_each(|(index, val)| *val = index as i32);
    for index in 0..10 * 10 {
        assert_eq!(grid.get_at_index(index), Some(&(index as i32)))
    }
}

/* ---------- */

#[test]
fn test_iter_mut_over() {
    let mut grid = Grid::new_square_filled(10, 0);

    assert_eq!(grid.iter_mut().count(), 100);
    assert_eq!(grid.iter().sum::<i32>(), 0);
    grid.iter_over_mut(Area::new(0, 0, 10, 10))
        .for_each(|val| *val = 10);
    assert_eq!(grid.iter().sum::<i32>(), 1000);
    assert_eq!(grid.iter_over_mut(Area::new(10, 10, 20, 20)).count(), 0);

    let mut grid = Grid::new_square_filled(1000, Witness::default());
    assert_eq!(grid.iter_mut().count(), 1_000_000);
    assert_eq!(grid.iter_mut().filter(|item| item.status).count(), 0);

    grid.iter_mut()
        .enumerate()
        .for_each(|(index, val)| val.id = index);

    for index in 0..1000 * 1000 {
        let wit = grid.get_at_index(index).unwrap();
        assert!(!wit.status);
        assert_eq!(wit.id, index)
    }

    grid.iter_over_mut(Area::new(959, 489, 964, 759))
        .enumerate()
        .for_each(|(_, val)| val.set_status());
    for index in 0..1000 * 1000 {
        let coords = coord_from_idx(index, 1000);
        let val = grid.get(coords.0, coords.1).unwrap();

        if is_contained_in_area(coords, Area::new(959, 489, 964, 759)) {
            assert_eq!(
                val,
                &Witness {
                    id: index,
                    status: true
                }
            );
        } else {
            assert_eq!(
                val,
                &Witness {
                    id: index,
                    status: false
                }
            );
        }
    }

    grid.iter_mut()
        .enumerate()
        .for_each(|(_, val)| val.unset_status());
    grid.iter_over_mut(Area::new(0, 0, 0, 999))
        .enumerate()
        .for_each(|(_, val)| val.set_status());
    for index in 0..1000 * 1000 {
        let coords = coord_from_idx(index, 1000);
        let val = grid.get(coords.0, coords.1).unwrap();

        if is_contained_in_area(coords, Area::new(0, 0, 0, 999)) {
            assert_eq!(
                val,
                &Witness {
                    id: index,
                    status: true
                }
            );
        } else {
            assert_eq!(
                val,
                &Witness {
                    id: index,
                    status: false
                }
            );
        }
    }
}

/* ---------- */
