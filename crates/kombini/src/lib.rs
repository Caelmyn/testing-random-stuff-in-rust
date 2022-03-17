//! An iterator to enumerate every combinaison of a given array.
//! Based on the [Heap's algorithm](https://en.wikipedia.org/wiki/Heap%27s_algorithm).

#![deny(missing_docs)]

/// The iterator to enumerate every combinaison of an array's item
///
/// One can create such an iterator in two ways, consuming the original array in the process:
/// ```
/// # use kombini::Kombini;
/// // From a fixed size array
/// let komb = Kombini::from([1, 2, 3]);
///
/// // From a Vec
/// let komb = Kombini::<i32, 3>::try_from(vec![1, 2, 3]);
/// ```
///
/// ### Notes
/// Creating a iterator from a [Vec] requires the array to
/// have the exact same size of the inner iterator.
/// On error, the original [Vec] will be returned.
/// ```
/// # use kombini::Kombini;
/// let komb = Kombini::<i32, 3>::try_from(vec![1, 2, 3]);
/// assert!(komb.is_ok());
///
/// let komb = Kombini::<i32, 5>::try_from(vec![1, 2, 3]);
/// assert!(komb.is_err());
/// ```
///
/// [Vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
pub struct Kombini<T: Clone, const N: usize> {
    values: [T; N],
    counter: [usize; N],
    n_iter: usize,
}

impl<T: Clone, const N: usize> From<[T; N]> for Kombini<T, N> {
    fn from(values: [T; N]) -> Self {
        Self {
            values,
            counter: [0; N],
            n_iter: 0,
        }
    }
}

impl<T: Clone, const N: usize> TryFrom<Vec<T>> for Kombini<T, N> {
    type Error = Vec<T>;
    fn try_from(values: Vec<T>) -> Result<Self, Self::Error> {
        let values = values.try_into()?;
        Ok(Self {
            values,
            counter: [0; N],
            n_iter: 0,
        })
    }
}

impl<T: Clone, const N: usize> Iterator for Kombini<T, N> {
    type Item = [T; N];
    fn next(&mut self) -> Option<Self::Item> {
        if self.n_iter > N {
            return None
        }

        while self.n_iter < N && self.counter[self.n_iter] >= self.n_iter {
            self.counter[self.n_iter] = 0;
            self.n_iter += 1;
        }

        if self.n_iter == N {
            self.n_iter += 1;
            return Some(self.values.clone());
        }

        let ret = Some(self.values.clone());

        let swap_index = if self.n_iter % 2 == 0 {
            0
        } else {
            self.counter[self.n_iter]
        };

        self.values.swap(swap_index, self.n_iter);
        self.counter[self.n_iter] += 1;
        self.n_iter = 0;

        ret
    }
}

impl<T: Clone, const N: usize> ExactSizeIterator for Kombini<T, N> {
    fn len(&self) -> usize {
        const_factorial::<N>()
    }
}

/* ---------- */

/// Compute the factoiral of given N. Use to return the len
/// of the iterator.
const fn const_factorial<const N: usize>() -> usize {
    let mut i = 1;
    let mut ret = 1;

    while i <= N {
        ret *= i;
        i += 1;
    }

    ret
}
