//! This module contains utils functions.

/// A helper that return an index given some (x, y) coordinates and a grid width
#[doc(hidden)]
#[inline]
pub(crate) const fn index_from_coord(x: usize, y: usize, grid_width: usize) -> usize {
    y * grid_width + x
}

/* ---------- */

/// A helper that return some (x, y) coordinates given an index and a grid width
#[doc(hidden)]
#[inline]
pub(crate) const fn coord_form_index(index: usize, grid_width: usize) -> (usize, usize) {
    (index % grid_width, index / grid_width)
}
