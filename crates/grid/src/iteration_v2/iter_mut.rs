use std::marker::PhantomData;
use std::ptr::NonNull;

use super::Cursor;
use crate::Area;

/* ---------- */

pub struct IterMut<'a, T: 'a> {
    ptr: NonNull<T>,
    end: *const T,
    cursor: Cursor,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: 'a> IterMut<'a, T> {
    /// Create a new iterator.
    ///
    /// ## Panics
    /// Panics if the given area or the grid width can make
    /// the iteraor go out of the array's bounds.
    #[inline]
    pub(crate) fn new(slice: &'a [T], grid_width: usize, area: Area) -> Self {
        let start_offset = area.top * grid_width + area.left;
        let end_offset = area.bottom * grid_width + area.right + 1;

        // SAFETY:
        // To be sure that we'll stay inbound at anytime.
        assert!(end_offset <= slice.len());

        let ptr = slice.as_ptr();

        // SAFETY:
        // * ptr is obtained from slice.as_ptr() where slice is
        // a valid ref. We are sure that it'll be non-null and is
        // safe to use.
        //
        // * end will never ever be dereferenced, only checked
        // against ptr to check if the iterator is done.
        unsafe {
            let (begin, end) = if std::mem::size_of::<T>() == 0 {
                (
                    (ptr as *const u8).wrapping_add(start_offset) as *const T,
                    (ptr as *const u8).wrapping_add(end_offset) as *const T,
                )
            } else {
                (ptr.add(start_offset), ptr.add(end_offset))
            };

            Self {
                ptr: NonNull::new_unchecked(begin as *mut T),
                end,
                cursor: Cursor::new(grid_width, area),
                _marker: PhantomData,
            }
        }
    }

    /// Move the internal pointer by a given offset.
    #[inline(always)]
    unsafe fn internal_next_unchecked(&mut self, offset: isize) -> *mut T {
        if std::mem::size_of::<T>() == 0 {
            self.end = (self.end as *const u8).wrapping_offset(-offset) as *mut T;
            self.ptr.as_ptr()
        } else {
            let old = self.ptr.as_ptr();

            // SAFETY:
            // the caller guarantees that offset doesn't exceed self.len(),
            // so this new pointer is inside self and thus guaranteed to be non-null.
            self.ptr = NonNull::new_unchecked(self.ptr.as_ptr().offset(offset));
            old
        }
    }

    /// Check if the iterator is empty.
    #[inline(always)]
    fn internal_is_empty(&self) -> bool {
        self.ptr.as_ptr() as *const T == self.end
    }

    /// Returns the remaining elements in the iterator.
    #[inline(always)]
    fn internal_len(&self) -> usize {
        self.cursor.remains()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.internal_is_empty() {
            return None;
        }

        // SAFETY:
        // Cursor::next_offset will always return a valid offset to apply to the pointer.
        let offset = self.cursor.next_offset();
        // println!("{:?}, offset = {offset}", self.cursor);
        unsafe { Some(&mut *self.internal_next_unchecked(offset as isize)) }
    }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.internal_len()
    }
}

/* ---------- */

#[cfg(test)]
mod tests {
    use crate::Area;
    use crate::iteration_v2::IterMut;

    const WIDTH: usize = 10;
    const AREA: usize = WIDTH * WIDTH;

    #[test]
    fn iter() {
        let vec = (0..AREA).collect::<Vec<usize>>();
        let area = Area::new(0, 0, 9, 9);

        let iter = IterMut::new(&vec, WIDTH, area);
        iter.for_each(|item| println!("{}", item));

        let iter = IterMut::new(&vec, WIDTH, Area::new(8, 8, 9, 9));
        iter.for_each(|item| println!("{}", item));
    }
}
