use crate::grid_dimension::GridDimension;

/* ---------- */

/// Allow iteration over a specific part of a Grid.
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Area {
    /// The area's top bound
    pub top: usize,

    /// The area's left bound
    pub left: usize,

    /// The area's bottom bound
    pub bottom: usize,

    /// The area's right bound
    pub right: usize,
}

impl Area {
    /// Create a new area.
    ///
    /// # Notes
    ///
    /// The Area created will always have its top-left coordinates
    /// stricly lower than its bottom-right ones. For example:
    ///
    /// ```
    /// use grid::Area;
    ///
    /// let area = Area::new(9, 9, 0, 0);
    ///
    /// assert_eq!(area.top, 0);
    /// assert_eq!(area.left, 0);
    /// assert_eq!(area.bottom, 9);
    /// assert_eq!(area.right, 9);
    /// ```
    #[inline(always)]
    pub const fn new(mut top: usize, mut left: usize, mut bottom: usize, mut right: usize) -> Self {
        // Manual swap because std::mem::swap isn't stable in const context yet
        #![allow(clippy::manual_swap)]

        if top > bottom {
            let tmp = top;
            top = bottom;
            bottom = tmp;
        }

        if left > right {
            let tmp = left;
            left = right;
            right = tmp;
        }

        Self {
            top,
            left,
            bottom,
            right,
        }
    }

    /// Return width of the Area.
    #[inline(always)]
    pub const fn width(&self) -> usize {
        self.right - self.left + 1
    }

    /// Return height of the Area.
    #[inline(always)]
    pub const fn height(&self) -> usize {
        self.bottom - self.top + 1
    }

    /// Return the number of elements covered by the Area.
    #[inline(always)]
    pub const fn area(&self) -> usize {
        self.height() * self.width()
    }
}

impl From<GridDimension> for Area {
    /// Generate an Area from a GridDimension resulting in an Area
    /// with a size of the grid
    #[doc(hidden)]
    fn from(dim: GridDimension) -> Self {
        Self {
            top: 0,
            left: 0,
            bottom: dim.height() - 1,
            right: dim.width() - 1,
        }
    }
}

/* ---------- */

#[cfg(test)]
mod tests {
    use super::Area;
    use super::GridDimension;

    #[test]
    fn test_area_elements() {
        assert_eq!(Area::new(0, 0, 0, 0).area(), 1);
        assert_eq!(Area::new(999, 999, 999, 999).area(), 1);
        assert_eq!(Area::new(0, 0, 999, 999).area(), 1_000_000);
        assert_eq!(Area::new(999, 999, 0, 0).area(), 1_000_000);
        assert_eq!(Area::new(959, 489, 964, 759).area(), 1626);
    }

    #[test]
    fn test_area_correctness() {
        assert_eq!(Area::new(0, 0, 0, 0), Area::new(0, 0, 0, 0));
        assert_eq!(Area::new(0, 0, 9, 9), Area::new(9, 9, 0, 0));

        let area = Area::new(9, 9, 0, 0);
        assert_eq!(area.top, 0);
        assert_eq!(area.left, 0);
        assert_eq!(area.bottom, 9);
        assert_eq!(area.right, 9);
    }

    #[test]
    fn area_from_grid_dim() {
        let area = Area::from(GridDimension::from((10, 10)));
        assert_eq!(area, Area::new(0, 0, 9, 9));
    }
}
