#![feature(trace_macros)]
#![feature(cell_update)]
#![feature(test)]
pub struct MapGen;

extern crate bencher;

mod map_option;
mod map_collection;

use map_collection::MapCollection2D;

pub fn new(x_size:usize, y_size:usize, x_resolution:usize, y_resolution:usize) -> MapCollection2D {
  return MapCollection2D::new(x_size, y_size, x_resolution, y_resolution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_collection() {
      let collection = &mut new(3, 3, 200, 200);
      collection.add_open_simplex_noise();
      collection.add_fbm_noise();
      collection.render_image();
    }
}
