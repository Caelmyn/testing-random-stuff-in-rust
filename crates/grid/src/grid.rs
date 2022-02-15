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
    /// Return the numbers of elements in the grid.
    #[inline]
    pub fn count(&self) -> usize {
        self.dim.area()
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
    pub fn iter(&self) -> Iter<T> {
        self.iter_over(Area::from(self.dim))
    }

    /// Return an iterator over a part of the grid specified by the given Area.<br>
    /// It iterates 'rows by rows'.
    #[inline]
    pub fn iter_over(&self, area: Area) -> Iter<T> {
        Iter::new(&self.inner, self.dim.width(), area)
    }

    /// Return a mutable iterator over the whole grid.<br>
    /// It iterates 'rows by rows'.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.iter_over_mut(Area::from(self.dim))
    }

    /// Return a mutable iterator over a part of the grid specified by the given Area.<br>
    /// It iterates 'rows by rows'.
    #[inline]
    pub fn iter_over_mut(&mut self, area: Area) -> IterMut<T> {
        IterMut::new(&mut self.inner, self.dim.width(), area)
    }
}
