use crate::utils;
use crate::Area;

/* ---------- */

/// A GridDimension defines a grid size. It's basically a wrapper
/// around a (usize, usize) tuple to ease readability and provides
/// some usefull methods for index <-> coords conversions.
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

    /// Rectify an Area to make sure that an iterator based on that Area
    /// wont go out of Grid's bound.
    ///
    /// An Area can be rectified if it is partially contained in the GridDimension
    ///
    /// ### Return
    /// Returns the area modified wrapped in a Some variant if the area could
    /// be rectified, None otherwise.
    ///
    /// ### Examples
    /// ```ignore
    /// # use grid::grid_dimension::GridDimension;
    /// # use grid::Area;
    /// let dim = GridDimension::new(10, 10);
    ///
    /// let area = Area::new(5, 5, 10, 10);
    /// let rectified = dim.rectify(area);
    /// assert_eq!(rectified, Some(Area::new(5, 5, 9, 9)));
    ///
    /// let area = Area::new(10, 10, 20, 20);
    /// let rectified = dim.rectify(area);
    /// assert_eq!(rectified, None);
    /// ```
    #[doc(hidden)]
    #[inline]
    pub(crate) const fn rectify(&self, mut area: Area) -> Option<Area> {
        if area.left >= self.0 || area.top >= self.1 {
            return None;
        }

        if area.right >= self.0 {
            area.right = self.0 - 1
        }

        if area.bottom >= self.1 {
            area.bottom = self.1 - 1
        }

        Some(area)
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

/* ---------- */

#[cfg(test)]
mod tests {
    use super::GridDimension;
    use crate::Area;

    #[test]
    fn rectify_test() {
        let dim = GridDimension::new(10, 10);

        let area = Area::new(5, 5, 10, 10);
        let rectified = dim.rectify(area);
        assert_eq!(rectified, Some(Area::new(5, 5, 9, 9)));

        let area = Area::new(5, 6, 8, 8);
        let rectified = dim.rectify(area);
        assert_eq!(rectified, Some(Area::new(5, 6, 8, 8)));

        let area = Area::new(0, 0, 9, 9);
        let rectified = dim.rectify(area);
        assert_eq!(rectified, Some(Area::new(0, 0, 9, 9)));

        let area = Area::new(5, 15, 20, 20);
        let rectified = dim.rectify(area);
        assert_eq!(rectified, None);

        let area = Area::new(15, 5, 20, 20);
        let rectified = dim.rectify(area);
        assert_eq!(rectified, None);

        let area = Area::new(10, 10, 20, 20);
        let rectified = dim.rectify(area);
        assert_eq!(rectified, None);
    }
}
