use crate::Area;
use super::cursor::Cursor;

/* ---------- */

pub struct IterMut<'a, T: Default> {
    inner: &'a mut [T],
    cursor: Cursor
}

impl<'a, T: Default> IterMut<'a, T> {
    /// Construct a mutable iterator over a Grid.
    #[inline]
    pub(crate) fn new(inner: &'a mut [T], grid_width: usize, area: Area) -> Self {
        Self {
            inner,
            cursor: Cursor::new(grid_width, area)
        }
    }
}

impl<'a, T: Default> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() || !self.cursor.is_valid() {
            return None
        }

        let prev_index = self.cursor.index();
        self.cursor.next();
        let new_index = self.cursor.index();

        let array = std::mem::take(&mut self.inner);
        let (left, right) = array.split_at_mut(new_index - prev_index);

        self.inner = right;
        Some(&mut left[0])
    }
}
