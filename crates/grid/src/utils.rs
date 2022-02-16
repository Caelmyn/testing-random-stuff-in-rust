//! This module contains utils functions.

/// A helper that return an index given some (x, y) coordinates and a grid width
#[doc(hidden)]
#[inline]
pub(crate) const fn index_from_coord(x: usize, y: usize, grid_width: usize) -> usize {
    (y * grid_width) + x
}

/* ---------- */

/// A helper that return some (x, y) coordinates given an index and a grid width
#[doc(hidden)]
#[inline]
pub(crate) const fn coords_from_index(index: usize, grid_width: usize) -> (usize, usize) {
    (index % grid_width, index / grid_width)
}

/* ---------- */

#[cfg(test)]
mod tests {
    use super::*;

    const GRID_SIZE: usize = 1000;

    #[test]
    fn conversion_index_to_coords() {
        assert_eq!(index_from_coord(0, 0, GRID_SIZE), 0);
        assert_eq!(index_from_coord(1, 0, GRID_SIZE), 1);
        assert_eq!(index_from_coord(0, 1, GRID_SIZE), 1000);
        assert_eq!(index_from_coord(999, 1, GRID_SIZE), 1999);
    }
}
