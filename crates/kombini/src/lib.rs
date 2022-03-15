//! An iterator to enumerate every combinaison of
//! a given array.

#![deny(missing_docs)]

/// The iterator to enumerate every combinaison.
///
/// ## Exemple
/// ```
/// # use kombini::Kombini;
/// let komb = Kombini::from([1, 2, 3]);
/// let expected = [
///     [1, 2, 3],
///     [2, 1, 3],
///     [2, 3, 1],
///     [3, 2, 1],
///     [3, 1, 2],
///     [1, 3, 2],
/// ];
///
/// // Iterates over all the possible combinasion of the
/// // array [1, 2, 3]
/// komb.enumerate().for_each(|(index, combinaison)| {
///     assert_eq!(combinaison, expected[index])
/// })
/// ```
pub struct Kombini<T: Clone, const N: usize> {
    values: [T; N],
    n_iter: usize,
    max_iter: usize,
    cursor: usize,
}

impl<T: Clone, const N: usize> From<[T; N]> for Kombini<T, N> {
    fn from(values: [T; N]) -> Self {
        Self {
            values,
            n_iter: 0,
            max_iter: N * (N - 1),
            cursor: 0,
        }
    }
}

impl<T: Clone, const N: usize> TryFrom<Vec<T>> for Kombini<T, N> {
    type Error = Vec<T>;
    fn try_from(values: Vec<T>) -> Result<Self, Self::Error> {
        let values = values.try_into()?;

        Ok(Self {
            values,
            n_iter: 0,
            max_iter: N * (N - 1),
            cursor: 0,
        })
    }
}

impl<T: Clone, const N: usize> Iterator for Kombini<T, N> {
    type Item = [T; N];
    fn next(&mut self) -> Option<Self::Item> {
        if self.n_iter == self.max_iter {
            return None
        }

        let ret = Some(self.values.clone());
        self.values.swap(self.cursor, self.cursor + 1);

        self.n_iter += 1;
        self.cursor = if self.cursor == N - 2 {
            0
        } else {
            self.cursor + 1
        };

        ret
    }
}

impl<T: Clone, const N: usize> ExactSizeIterator for Kombini<T, N> {
    fn len(&self) -> usize {
        N * (N - 1)
    }
}
