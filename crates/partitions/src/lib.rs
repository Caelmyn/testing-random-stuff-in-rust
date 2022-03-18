//! Iterates over the partitions of fixed addends.

#![deny(missing_docs)]

/// Iterates over the partitions of fixed addends
/// of a given positive integer.
///
/// ## Example
/// ```
/// # use partitions::Generator;
/// let gen = Generator::<5>::new(20);
///
/// // Iterate over all the partitions of 5
/// // numbers that adds up to 20
/// for part in gen {
///     // Some computation using that integer partition
/// }
/// ```
///
/// ## Notes
/// The algorithm will only generate one *kind* of partition.
/// ```
/// # use partitions::Generator;
/// let mut gen = Generator::<2>::new(10);
///
/// assert_eq!(gen.next(), Some([9, 1]));
/// assert_eq!(gen.next(), Some([8, 2]));
/// assert_eq!(gen.next(), Some([7, 3]));
/// assert_eq!(gen.next(), Some([6, 4]));
/// assert_eq!(gen.next(), Some([5, 5]));
///
/// // It will never generate Some(4, 6).
/// assert_eq!(gen.next(), None);
/// ```
pub struct Partitions<const N: usize> {
    values: [usize; N],
    state: State,
    n: usize,
}

impl<const N: usize> Partitions<N> {
    /// Creates a new iterator.
    ///
    /// ## Arguments
    /// * n: The number to decompose.
    /// * N: The number of addends that will decompose the given number.
    ///
    /// ## Panics
    /// Panics if:
    /// * N <= 1
    /// * n < N
    ///
    /// ## Example
    /// ```should_panic
    /// # use partitions::Generator;
    /// // OK
    /// Generator::<2>::new(3);
    /// Generator::<3>::new(3);
    ///
    /// // Failures
    /// Generator::<0>::new(3); // Panics: trying to decompose 3 in 0 addend
    /// Generator::<1>::new(3); // Panics: trying to decompose 3 in 1 one addend
    /// Generator::<5>::new(3); // Panics: trying to decompose 3 in 5 five addends
    /// ```
    pub fn new(n: usize) -> Self {
        assert!(N > 1);
        assert!(n >= N);

        let mut values = [n - N + 1; N];
        values[1..].iter_mut().for_each(|val| *val = 1);

        Self {
            values,
            state: State::default(),
            n,
        }
    }
}

impl<const N: usize> Iterator for Partitions<N> {
    type Item = [usize; N];

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::Default => (),
            State::Done => return None,
            State::Reset => {
                let index = match self
                    .values[2..]
                    .iter()
                    .position(|val| *val < self.values[0] - 1)
                {
                    Some(index) => index + 2,
                    None => {
                        self.state = State::Done;
                        return None;
                    }
                };

                let new_val = self.values[index] + 1;
                let mut sum = self.values[1..=index]
                    .iter_mut()
                    .map(|val| {
                        *val = new_val;
                        new_val
                    })
                    .sum::<usize>();

                sum += self.values[index + 1..].iter().sum::<usize>();
                self.values[0] = self.n - sum;
            }
        }

        let ret = Some(self.values);

        self.state = if self.values[0] > self.values[1] + 1 {
            self.values[0] -= 1;
            self.values[1] += 1;
            State::Default
        } else {
            State::Reset
        };

        ret
    }
}

/* ---------- */

#[derive(Debug)]
enum State {
    Default,
    Reset,
    Done,
}

impl Default for State {
    fn default() -> Self {
        Self::Default
    }
}

/* ---------- */

#[cfg(test)]
mod tests {
    use super::Partitions;

    #[test]
    fn ctor() {
        let parts = Partitions::<10>::new(10);
        assert!(parts.values.iter().all(|val| *val == 1));

        let parts = Partitions::<10>::new(20);
        let expected = [11, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(parts.values, expected);
        assert_eq!(parts.values.iter().sum::<usize>(), 20);
    }

    #[test]
    #[should_panic]
    fn ctor_failure() {
        Partitions::<0>::new(3);
        Partitions::<1>::new(3);
        Partitions::<5>::new(3);
    }
}
