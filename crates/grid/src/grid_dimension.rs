use crate::utils;

/* ---------- */

/// A GridDimension defines a grid size. It's basically a helper
/// to ease readability and provides some usefull methods for
/// index <-> coords conversions.
#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct GridDimension(usize, usize);

impl GridDimension {
    /// Construct a new GridDimension.
    #[doc(hidden)]
    #[inline]
    pub(crate) const fn new(width: usize, height: usize) -> Self {
        Self(width, height)
    }

    /// Convert some (x, y) coord into a index.
    #[doc(hidden)]
    #[inline]
    pub(crate) const fn index_from_coord(&self, x: usize, y: usize) -> usize {
        utils::index_from_coord(x, y, self.0)
    }

    /// Convert an index into the corrersonding coords in the grid.
    #[doc(hidden)]
    #[inline]
    pub(crate) const fn coords_from_index(&self, index: usize) -> (usize, usize) {
        utils::coords_from_index(index, self.0)
    }

    /// Return numbers of elements in the grid.
    #[doc(hidden)]
    #[inline]
    pub(crate) const fn area(&self) -> usize {
        self.0 * self.1
    }

    /// Return the width of the grid.
    #[doc(hidden)]
    #[inline]
    pub(crate) const fn width(&self) -> usize {
        self.0
    }

    /// Return the height of the grid.
    #[doc(hidden)]
    #[inline]
    pub(crate) const fn height(&self) -> usize {
        self.1
    }
}

impl From<(usize, usize)> for GridDimension {
    /// Construct a new GridDimension from some (x, y) tuple.
    #[doc(hidden)]
    #[inline]
    fn from((width, height): (usize, usize)) -> Self {
        Self::new(width, height)
    }
}
