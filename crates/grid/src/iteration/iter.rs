use crate::Area;
use super::cursor::Cursor;

/* ---------- */

pub struct Iter<'a, T> {
    inner: &'a [T],
    cursor: Cursor
}

impl<'a, T> Iter<'a, T> {
    /// Construct an iterator over a Grid.
    #[inline]
    pub(crate) fn new(inner: &'a [T], grid_width: usize, area: Area) -> Self {
        Self {
            inner,
            cursor: Cursor::new(grid_width, area)
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.cursor.index();

        if index >= self.inner.len() || !self.cursor.is_valid() {
            return None
        }

        let ret = Some(&self.inner[index]);
        self.cursor.next();
        ret
    }
}
