#![feature(trace_macros)]
#![feature(cell_update)]
#![feature(test)]
pub struct MapGen;

extern crate bencher;

mod map_collection;

use map_collection::MapCollection2D;
use std::sync::RwLock;

pub fn new(x_size:usize, y_size:usize, x_resolution:usize, y_resolution:usize) -> MapCollection2D {
  return MapCollection2D::new(x_size, y_size, x_resolution, y_resolution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_collection() {
      fn update_value(value: i32) -> (i32) {
        value * 2
      }

      let shaders = &mut vec![];
      shaders.push(RwLock::new(Box::new(update_value as fn(i32) -> i32)));
      
      let collection = &mut new(3, 3, 200, 200);
      collection.add_open_simplex_noise();
      collection.add_fbm_noise();
      collection.render(shaders);
      collection.render_image();
    }
}
