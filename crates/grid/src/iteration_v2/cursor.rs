use crate::Area;

/* ---------- */

#[derive(Debug)]
pub(super) struct Cursor {
    index: usize,
    area_witdh: usize,
    total_area: usize,
    delta_offset: usize,
}

impl Cursor {
    #[inline(always)]
    pub(super) const fn new(grid_width: usize, area: Area) -> Self {
        let delta_offset = grid_width - area.width() + 1;

        Self {
            index: 0,
            area_witdh: area.width(),
            total_area: area.area(),
            delta_offset,
        }
    }

    #[inline(always)]
    pub(super) fn next_offset(&mut self) -> usize {
        self.index += 1;

        if self.index < self.total_area && self.index % self.area_witdh == 0 {
            self.delta_offset
        } else {
            1
        }
    }

    #[inline(always)]
    pub(super) fn remains(&self) -> usize {
        if self.total_area <= self.index {
            0
        } else {
            self.total_area - self.index
        }
    }
}
