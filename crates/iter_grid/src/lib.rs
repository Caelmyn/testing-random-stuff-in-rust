pub trait IterGrid: Iterator {
    fn grid(self, width: usize) -> Grid<Self>
    where
        Self: Sized,
    {
        Grid::new(self, width)
    }
}

impl<T> IterGrid for T where T: Iterator {}

/* ---------- */

pub struct Grid<I: Iterator> {
    it: I,
    width: usize,
    x: usize,
    y: usize,
}

impl<I: Iterator> Grid<I> {
    pub fn new(it: I, width: usize) -> Self {
        Self {
            it,
            width,
            x: 0,
            y: 0,
        }
    }
}

impl<I: Iterator> Iterator for Grid<I> {
    type Item = ((usize, usize), I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|inner| {
            let ret = ((self.x, self.y), inner);

            if self.x == self.width - 1 {
                self.x = 0;
                self.y += 1;
            } else {
                self.x += 1;
            }

            ret
        })
    }
}

/* ---------- */

pub struct Area<I: Iterator> {
    it: I,
    width: usize,
    area: (usize, usize, usize, usize),
    x: usize,
    y: usize,
}

impl<I: Iterator> Area<I> {
    pub fn new(it: I, width: usize, area: (usize, usize, usize, usize)) -> Self {
        Self {
            it,
            width,
            area,
            x: 0,
            y: 0,
        }
    }
}

impl<I: Iterator> Iterator for Area<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
