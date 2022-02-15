#![warn(missing_docs)]

//! A Grid is a wrapper around an array built to ease
//! manipulation of grid-like arrays
//!
//! ### Code example
//!
//! ```
//! use grid::{Grid, Area};
//!
//! let mut grid = Grid::new_filled(10, 10, false);
//!
//! println!("There's {} items set to true", grid.iter().filter(|item| **item).count());
//! grid.iter_over_mut(Area::new(5, 5, 6, 6)).for_each(|item| *item = true);
//! println!("Now, there's {} items set to true", grid.iter().filter(|item| **item).count());
//!
//! ```

mod area;
mod grid;
mod grid_dimension;
mod iteration;
mod utils;

pub use crate::area::Area;
pub use crate::grid::Grid;
