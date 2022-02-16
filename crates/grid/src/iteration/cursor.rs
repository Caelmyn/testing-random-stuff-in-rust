use crate::Area;
use crate::utils;

/// A Cursor is the heart of the iterator, it is this structure that define the current
/// index position and the next position according to the grid size and the area to
/// iterate over.
#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct Cursor {
    grid_width: usize,
    area: Area,
    cursor: usize,
}

impl Cursor {
    /// Construct a new cursor.
    #[doc(hidden)]
    #[inline]
    pub(super) const fn new(grid_width: usize, area: Area) -> Self {
        let cursor = utils::index_from_coord(area.left, area.top, grid_width);

        Self {
            grid_width,
            area,
            cursor
        }
    }

    /// Define the next position in the array based on the grid size and the area to iterate over.
    #[doc(hidden)]
    #[inline]
    pub(super) fn next(&mut self) {
        let (x, y) = utils::coords_from_index(self.cursor, self.grid_width);
        if x >= self.area.right {
            self.cursor = utils::index_from_coord(self.area.left, y + 1, self.grid_width);
        } else {
            self.cursor += 1;
        }
    }

    /// Return whether or not the cursor is contained in the area.
    #[doc(hidden)]
    #[inline]
    pub(super) const fn is_valid(&self) -> bool {
        let (x, y) = utils::coords_from_index(self.cursor, self.grid_width);
        x <= self.area.right && y <= self.area.bottom
    }

    /// Return the current position of the cursor in the array.
    #[doc(hidden)]
    #[inline]
    pub(super) const fn index(&self) -> usize {
        self.cursor
    }
}

#[cfg(test)]
mod tests {
    use crate::Area;
    use super::Cursor;

    const GRID_SIZE: usize = 1000;

    #[test]
    fn new_cursor() {
        let area = Area::new(0, 0, 0, 0);
        let cursor = Cursor::new(GRID_SIZE, area);
        assert_eq!(cursor.cursor, 0);

        let area = Area::new(0, 1, 0, 0);
        let cursor = Cursor::new(GRID_SIZE, area);
        assert_eq!(cursor.cursor, 0);

        let area = Area::new(0, 1, 0, 1);
        let cursor = Cursor::new(GRID_SIZE, area);
        assert_eq!(cursor.cursor, 1);

        let area = Area::new(959, 489, 964, 759);
        let cursor = Cursor::new(GRID_SIZE, area);
        assert_eq!(cursor.cursor, 959489);
    }
}
