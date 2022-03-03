use super::cursor::Cursor;
use crate::Area;

/* ---------- */

/// Immutable Grid iterator
pub struct Iter<'a, T> {
    inner: &'a [T],
    cursor: Cursor,
}

impl<'a, T> Iter<'a, T> {
    /// Construct an iterator over a Grid.
    #[inline]
    pub(crate) fn new(inner: &'a [T], grid_width: usize, area: Area) -> Self {
        Self {
            inner,
            cursor: Cursor::new(grid_width, area),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = match self.cursor.next() {
            Some(index) if index < self.inner.len() => index,
            _ => return None,
        };

        Some(&self.inner[index])
    }
}

/* ---------- */

#[cfg(test)]
mod tests {
    use super::Iter;
    use crate::Area;

    const GRID_SIDE_LEN: usize = 10;
    const ARRAY_LEN: usize = GRID_SIDE_LEN * GRID_SIDE_LEN;

    fn build_array() -> Vec<usize> {
        let mut array = Vec::new();

        for idx in 0..ARRAY_LEN {
            array.push(idx);
        }

        array
    }

    #[test]
    fn iter() {
        let array = build_array();
        let area = Area::new(0, 0, 9, 9);

        let it = Iter::new(&array, GRID_SIDE_LEN, area);
        assert_eq!(it.count(), ARRAY_LEN);

        let it = Iter::new(&array, GRID_SIDE_LEN, area);
        it.enumerate().for_each(|(idx, value)| {
            assert_eq!(*value, idx);
        });
    }

    #[test]
    fn iter_over() {
        let array = build_array();
        let area = Area::new(0, 0, 1, 1);

        let it = Iter::new(&array, GRID_SIDE_LEN, area);
        assert_eq!(it.count(), 4);

        let it = Iter::new(&array, GRID_SIDE_LEN, area);
        let expected = [0, 1, 10, 11];
        it.enumerate().for_each(|(idx, value)| {
            assert_eq!(*value, expected[idx]);
        });

        let area = Area::new(8, 8, 9, 9);
        let it = Iter::new(&array, GRID_SIDE_LEN, area);
        let expected = [88, 89, 98, 99];
        it.enumerate().for_each(|(idx, value)| {
            assert_eq!(*value, expected[idx]);
        });
    }
}
