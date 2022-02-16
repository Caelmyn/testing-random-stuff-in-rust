use crate::Area;
use super::cursor::Cursor;

/* ---------- */

pub struct IterMut<'a, T> {
    inner: &'a mut [T],
    cursor: Cursor
}

impl<'a, T> IterMut<'a, T> {
    /// Construct a mutable iterator over a Grid.
    #[inline]
    pub(crate) fn new(inner: &'a mut [T], grid_width: usize, area: Area) -> Self {
        let cursor = Cursor::new(grid_width, area);
        let inner = inner.split_at_mut(cursor.index()).1;

        Self {
            inner,
            cursor,
        }
    }
}

impl<'a, T: Default> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {

        if self.inner.is_empty() || !self.cursor.is_valid() {
            return None
        }

        let array = std::mem::take(&mut self.inner);
        let prev_index = self.cursor.index();

        self.cursor.next();
        let diff = self.cursor.index() - prev_index;

        if diff < array.len() {
            let (left, right) = array.split_at_mut(diff);
            self.inner = right;
            Some(&mut left[0])
        } else {
            Some(&mut array[0])
        }
    }
}
