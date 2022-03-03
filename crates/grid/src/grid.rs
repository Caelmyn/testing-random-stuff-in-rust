use crate::area::Area;
use crate::grid_dimension::GridDimension;
use crate::iteration::{Iter, IterMut};

/* ---------- */

/// A wrapper over an 1D array that act like a 2D array.
///
/// ### Structure
///
/// The axis of a grid are defined as follow:
/// * the 'horizontal' axis, also refered as the width, is accessed via the 'x' coordinate
/// * the 'vertical' axis, also refered as the height, is accessed via the 'y' coordinate
///
/// ### Example:
/// ```
/// use grid::Grid;
/// let grid = Grid::<u32>::new_with_default(5, 3);
///
/// // The grid constructed can be represented as follow
/// // +-------+-------+-------+-------+-------+
/// // | (0,0) | (0,1) | (0,2) | (0,3) | (0,4) |
/// // +-------+-------+-------+-------+-------+
/// // | (1,0) | (1,1) | (1,2) | (1,3) | (1,4) |
/// // +-------+-------+-------+-------+-------+
/// // | (2,0) | (2,1) | (2,2) | (2,3) | (2,4) |
/// // +-------+-------+-------+-------+-------+
/// ```
pub struct Grid<T> {
    inner: Vec<T>,
    dim: GridDimension,
}

impl<T: Default + Clone> Grid<T> {
    /// Construct a new grid with given width and height, filled with default values.
    #[inline]
    pub fn new_with_default(width: usize, height: usize) -> Self {
        // This line here forces T to be Default + Clone which is annoying.
        let inner = vec![T::default(); width * height];

        Self {
            inner,
            dim: GridDimension::new(width, height),
        }
    }

    /// Construct a new squared grid with given side length, filled with default values.
    #[inline]
    pub fn new_square_with_default(lenght: usize) -> Self {
        Self::new_with_default(lenght, lenght)
    }

    /// Construct a new grid with given width and height, filled with given value.
    #[inline]
    pub fn new_filled(width: usize, height: usize, val: T) -> Self {
        // This line here forces T to be Default + Clone which is annoying.
        let inner = vec![val; width * height];

        Self {
            inner,
            dim: GridDimension::new(width, height),
        }
    }

    /// Construct a new grid with given width and height, filled with given value.
    #[inline]
    pub fn new_square_filled(lenght: usize, val: T) -> Self {
        Self::new_filled(lenght, lenght, val)
    }
}

impl<T> Grid<T> {
    /// Creates a Grid from a Vec.
    ///
    /// Returns `Some(Grid)` if the vec can be perfectly converted to a Grid
    /// ### Example
    /// ```
    /// # use grid::Grid;
    /// assert!(Grid::try_from_vec(vec![0; 100], 10).is_some());
    /// assert!(Grid::try_from_vec(vec![0; 99], 10).is_none());
    /// ```
    #[inline]
    pub fn try_from_vec(array: Vec<T>, grid_width: usize) -> Option<Self> {
        let len = array.len();
        let width = len / grid_width;

        if len % grid_width != 0 {
            return None;
        }

        let height = len / grid_width;
        let ret = Self {
            inner: array,
            dim: GridDimension::new(width, height),
        };

        Some(ret)
    }

    /// Return the numbers of elements in the grid.
    #[inline]
    pub fn count(&self) -> usize {
        self.dim.area()
    }

    /// Convert some (x, y) coord into a index.
    #[inline]
    pub const fn index_from_coord(&self, x: usize, y: usize) -> usize {
        self.dim.index_from_coord(x, y)
    }

    /// Convert an index into the corrersonding coords in the grid.
    #[inline]
    pub const fn coords_from_index(&self, index: usize) -> (usize, usize) {
        self.dim.coords_from_index(index)
    }

    /// Return a shared reference to the item at the coords (x, y).<br>
    /// Return Option<&T> if the coords are contained in the grid, None otherwise.
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let index = self.dim.index_from_coord(x, y);
        self.inner.get(index)
    }

    /// Return a shared reference to the item at the index.<br>
    /// Return Option<&T> if the index is smaller than the grid's count, None otherwise.
    #[inline]
    pub fn get_at_index(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }

    /// Return a mutable reference to the item at the coords (x, y).<br>
    /// Return Option<&mut T> if the coords are contained in the grid, None otherwise.
    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let index = self.dim.index_from_coord(x, y);
        self.inner.get_mut(index)
    }

    /// Return a mutable reference to the item at the index.<br>
    /// Return Option<&mut T> if the index is smaller than the grid's count, None otherwise.
    #[inline]
    pub fn get_at_index_mut(&mut self, index: usize) -> Option<&mut T> {
        self.inner.get_mut(index)
    }

    /// Return an iterator over the whole grid.<br>
    /// It iterates 'rows by rows'.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }

    /// Return an iterator over a part of the grid specified by the given Area.<br>
    /// It iterates 'rows by rows'.
    #[inline]
    pub fn iter_over(&self, area: Area) -> impl Iterator<Item = &T> {
        Iter::new(&self.inner, self.dim.width(), area)
    }

    /// Return a mutable iterator over the whole grid.<br>
    /// It iterates 'rows by rows'.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.inner.iter_mut()
    }

    /// Return a mutable iterator over a part of the grid specified by the given Area.<br>
    /// It iterates 'rows by rows'.
    #[inline]
    pub fn iter_over_mut(&mut self, area: Area) -> impl Iterator<Item = &mut T> {
        match self.dim.rectify(area) {
            Some(area) => IterMut::new(&mut self.inner, self.dim.width(), area),
            None => IterMut::new(&mut [], self.dim.width(), area),
        }
    }
}

/* ---------- */

#[cfg(test)]
mod tests {
    use crate::Area;
    use crate::Grid;

    const GRID_SIZE: usize = 10;
    const ARRAY_LEN: usize = GRID_SIZE * GRID_SIZE;

    #[test]
    fn getters() {
        let mut array = Vec::with_capacity(ARRAY_LEN);
        for value in 0..ARRAY_LEN {
            array.push(value)
        }

        let mut grid = Grid::try_from_vec(array, GRID_SIZE).unwrap();
        assert_eq!(grid.get(5, 6), Some(&65));
        assert_eq!(grid.get_at_index(65), Some(&65));
        assert_eq!(grid.get_mut(5, 7), Some(&mut 75));
        assert_eq!(grid.get_at_index_mut(75), Some(&mut 75));

        *grid.get_mut(9, 9).unwrap() = 0;
        assert_eq!(grid.get(9, 9), Some(&0))
    }

    #[test]
    fn test_iter() {
        let grid = Grid::new_square_filled(GRID_SIZE, 2);

        assert_eq!(grid.iter().count(), ARRAY_LEN);
        grid.iter().for_each(|val| assert_eq!(val, &2));
        assert_eq!(grid.iter().sum::<i32>(), 200);
    }

    #[test]
    fn test_iter_over() {
        let grid = Grid::new_square_filled(GRID_SIZE, 0);

        assert_eq!(grid.iter_over(Area::new(0, 0, 0, 0)).count(), 1);
        assert_eq!(grid.iter_over(Area::new(0, 0, 1, 1)).count(), 4);
        assert_eq!(grid.iter_over(Area::new(1, 1, 2, 2)).count(), 4);
        assert_eq!(grid.iter_over(Area::new(0, 0, 9, 9)).count(), 100);
        assert_eq!(grid.iter_over(Area::new(10, 10, 11, 11)).count(), 0);
        assert_eq!(grid.iter_over(Area::new(9, 9, 9, 9)).count(), 1);
        assert_eq!(grid.iter_over(Area::new(5, 6, 6, 6)).count(), 2);
        assert_eq!(grid.iter_over(Area::new(9, 9, 10, 9)).count(), 1);

        let mut array = Vec::with_capacity(ARRAY_LEN);
        for value in 0..ARRAY_LEN {
            array.push(value)
        }

        let grid = Grid::try_from_vec(array, GRID_SIZE).unwrap();
        let expected = [11, 12, 13, 21, 22, 23, 31, 32, 33];
        grid.iter_over(Area::new(1, 1, 3, 3))
            .enumerate()
            .for_each(|(index, value)| {
                assert_eq!(*value, expected[index]);
            })
    }

    #[test]
    fn test_iter_mut() {
        let mut grid = Grid::new_square_filled(GRID_SIZE, 0);

        assert_eq!(grid.iter_mut().count(), ARRAY_LEN);
        assert_eq!(grid.iter().sum::<i32>(), 0);

        grid.iter_mut().for_each(|val| *val = 10);
        assert_eq!(grid.iter().sum::<i32>(), 1000);

        grid.iter_mut()
            .enumerate()
            .for_each(|(index, val)| *val = index as i32);

        for index in 0..ARRAY_LEN {
            assert_eq!(grid.get_at_index(index), Some(&(index as i32)))
        }
    }

    #[test]
    fn test_iter_mut_over() {
        let mut grid = Grid::new_square_filled(GRID_SIZE, 0);

        assert_eq!(grid.iter_mut().count(), ARRAY_LEN);
        assert_eq!(grid.iter().sum::<i32>(), 0);

        grid.iter_over_mut(Area::new(0, 0, 10, 10))
            .for_each(|val| *val = 10);

        assert_eq!(grid.iter().sum::<i32>(), 1000);
        assert_eq!(grid.iter_over_mut(Area::new(10, 10, 20, 20)).count(), 0);

        let mut array = Vec::with_capacity(ARRAY_LEN);
        for value in 0..ARRAY_LEN {
            array.push(value)
        }

        let mut grid = Grid::try_from_vec(array, GRID_SIZE).unwrap();
        grid.iter_over_mut(Area::new(1, 1, 3, 3))
            .for_each(|value| *value = 0);

        let expected = [11, 12, 13, 21, 22, 23, 31, 32, 33];
        for index in 0..ARRAY_LEN {
            if expected.contains(&index) {
                assert_eq!(grid.get_at_index(index), Some(&0))
            } else {
                assert_eq!(grid.get_at_index(index), Some(&index))
            }
        }
    }
}
