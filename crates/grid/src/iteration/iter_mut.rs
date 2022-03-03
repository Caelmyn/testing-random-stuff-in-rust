use super::cursor::Cursor;
use crate::Area;

/* ---------- */

/// Mutable Grid iterator
pub struct IterMut<'a, T> {
    inner: &'a mut [T],
    cursor: Cursor,
}

impl<'a, T> IterMut<'a, T> {
    /// Creates a new iterator over the grid.
    #[inline]
    pub(crate) fn new(inner: &'a mut [T], grid_width: usize, area: Area) -> Self {
        let cursor = Cursor::new(grid_width, area);

        let inner = match cursor.index() {
            Some(index) if !inner.is_empty() => inner.split_at_mut(index).1,
            _ => &mut [],
        };

        Self { inner, cursor }
    }
}

impl<'a, T: Default> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }

        let array = std::mem::take(&mut self.inner);
        let prev = match self.cursor.next() {
            Some(index) => index,
            None => return None,
        };

        let diff = self.cursor.index_unchecked() - prev;

        if diff < array.len() {
            let (left, right) = array.split_at_mut(diff);
            self.inner = right;
            Some(&mut left[0])
        } else {
            Some(&mut array[0])
        }
    }
}

/* ---------- */

#[cfg(test)]
mod tests {
    use super::IterMut;
    use crate::Area;

    const GRID_SIDE_LEN: usize = 10;
    const ARRAY_LEN: usize = GRID_SIDE_LEN * GRID_SIDE_LEN;

    #[test]
    fn iter_mut() {
        let mut array = vec![0; ARRAY_LEN];
        let area = Area::new(0, 0, 9, 9);

        let it = IterMut::new(&mut array, GRID_SIDE_LEN, area);
        assert_eq!(it.count(), ARRAY_LEN);

        let it = IterMut::new(&mut array, GRID_SIDE_LEN, area);
        it.enumerate().for_each(|(idx, value)| *value = idx);
        array.iter().enumerate().for_each(|(idx, value)| {
            assert_eq!(idx, *value);
        })
    }

    #[test]
    fn iter_over_mut() {
        let mut array = vec![0; ARRAY_LEN];
        let area = Area::new(5, 5, 6, 6);

        let it = IterMut::new(&mut array, GRID_SIDE_LEN, area);
        assert_eq!(it.count(), 4);

        let it = IterMut::new(&mut array, GRID_SIDE_LEN, area);
        it.for_each(|value| *value = 1);

        let expected = [55, 56, 65, 66];
        array.iter().enumerate().for_each(|(idx, value)| {
            if expected.contains(&idx) {
                assert_eq!(*value, 1)
            }
        })
    }
}
