pub struct Partition<const N: usize> {
    values: [usize; N],
}

impl<const N: usize> Partition<N> {
    pub fn new(to_partition: usize) -> Self {
        assert!(N > 0);

        let mut values = [0; N];

        values[0] = to_partition - N;
        values.iter_mut().take(N).skip(1).for_each(|val| *val = 1);

        Self { values }
    }
}

impl<const N: usize> Iterator for Partition<N> {
    type Item = &'static [usize];

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/* ---------- */

#[cfg(test)]
mod tests {

}
