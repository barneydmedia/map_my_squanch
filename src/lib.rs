#![feature(trace_macros)]
#![feature(cell_update)]
#![feature(test)]
pub struct MapGen;

extern crate bencher;

mod map_option;
mod map_collection;

use bencher::Bencher;

use map_collection::{MapCollection2D, map2d::terrain_map2d::TerrainMap2D};
use map_option::{MapOption};

pub fn new(x_size:usize, y_size:usize) -> MapCollection2D {
  return MapCollection2D::new(x_size, y_size);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_collection() {
      new(10, 10);
    }
}
