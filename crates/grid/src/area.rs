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
    #[inline]
    pub const fn new(mut top: usize, mut left: usize, mut bottom: usize, mut right: usize) -> Self {
        // Manual swap because std::mem::swap isn't stable in const context yet
        #[allow(clippy::manual_swap)]
        if top > bottom {
            let tmp = top;
            top = bottom;
            bottom = tmp;
        }

        // Manual swap because std::mem::swap isn't stable in const context yet
        #[allow(clippy::manual_swap)]
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

    /// Return the number of elements covered by the Area.
    #[inline]
    pub const fn area(&self) -> usize {
        (self.bottom - self.top + 1) * (self.right - self.left + 1)
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
