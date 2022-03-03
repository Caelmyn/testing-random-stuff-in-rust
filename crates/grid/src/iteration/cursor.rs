use crate::utils;
use crate::Area;

/* ---------- */

/// Iterator that returns a set of 2D coord over a given Area.
pub(super) struct Cursor {
    grid_width: usize,
    area: Area,
    cursor: (usize, usize),
}

impl Cursor {
    /// Create a new iterator over an Area. To compute the 2D coord
    /// it needs to know the width of the grid.
    #[inline]
    pub(super) fn new(grid_width: usize, area: Area) -> Self {
        let cursor = (area.left, area.top);

        Self {
            grid_width,
            area,
            cursor,
        }
    }

    /// Compute the current Cursor array index. Returns None if the cursor
    /// is out of the grid's bound.
    #[inline]
    pub(super) fn index(&self) -> Option<usize> {
        (self.cursor.1 <= self.area.bottom).then(|| self.index_unchecked())
    }

    /// Compute the current Cursor array index.
    ///
    /// ## Safety
    /// The returned index isn't checked ; it might returns on index that is
    /// out of the grid's bounds
    #[inline]
    pub(super) fn index_unchecked(&self) -> usize {
        utils::index_from_coord(self.cursor.0, self.cursor.1, self.grid_width)
    }
}

impl Iterator for Cursor {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.1 > self.area.bottom {
            return None;
        }

        let index = self.index_unchecked();

        if self.cursor.0 >= self.area.right {
            self.cursor.0 = self.area.left;
            self.cursor.1 += 1
        } else {
            self.cursor.0 += 1;
        }

        Some(index)
    }
}

/* ---------- */

#[cfg(test)]
mod tests {
    use super::Cursor;
    use crate::Area;

    const GRID_SIZE: usize = 1000;

    #[test]
    fn new_cursor() {
        let area = Area::new(0, 0, 0, 0);
        let cursor = Cursor::new(GRID_SIZE, area);
        assert_eq!(cursor.cursor, (0, 0));

        let area = Area::new(0, 1, 0, 0);
        let cursor = Cursor::new(GRID_SIZE, area);
        assert_eq!(cursor.cursor, (0, 0));

        let area = Area::new(0, 1, 0, 1);
        let cursor = Cursor::new(GRID_SIZE, area);
        assert_eq!(cursor.cursor, (1, 0));

        let area = Area::new(959, 489, 964, 759);
        let cursor = Cursor::new(GRID_SIZE, area);
        assert_eq!(cursor.cursor, (489, 959));
    }

    #[test]
    fn iter() {
        let area = Area::new(0, 0, 0, 0);
        let mut cursor = Cursor::new(GRID_SIZE, area);
        assert_eq!(cursor.next(), Some(0));
        assert_eq!(cursor.next(), None);

        let area = Area::new(0, 0, 1, 1);
        let cursor = Cursor::new(GRID_SIZE, area);
        assert_eq!(cursor.count(), 4);

        let area = Area::new(1, 1, 2, 2);
        let cursor = Cursor::new(GRID_SIZE, area);
        let witness = [1001, 1002, 2001, 2002];
        cursor
            .enumerate()
            .for_each(|(idx, cursor)| assert_eq!(cursor, witness[idx]))
    }
}
