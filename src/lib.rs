#![feature(trace_macros)]
#![feature(cell_update)]
#![feature(test)]
pub struct MapGen;

extern crate bencher;

mod terrain_map;
mod map_options;
mod block_options;
mod map;

use noise::{Fbm, utils::NoiseMapBuilder, utils::*, OpenSimplex, Add, Seedable};
use noise::utils::PlaneMapBuilder;
use std::cell::{RefCell};
use std::rc::{Rc};
use std::sync::{RwLock, Arc, Mutex};
use terrain_map::{TerrainMap2D};
use map_options::{MapOption};
use block_options::{BlockOption};
use rand::Rng;
use rayon::prelude::*;
use bencher::Bencher;



impl MapGen {

}

pub fn new_map(options: MapOption) -> Vec<TerrainMap2D> {
  let mut map = vec!();

  for _ in 0..5 {
    map.push(TerrainMap2D::new(options.get_width(), options.get_height()));
  }

  return map;
}

pub fn new_block(options: &BlockOption) -> TerrainMap2D {
  let block_cell = Arc::new(Mutex::new(TerrainMap2D::new(options.get_width(), options.get_height())));

  let open_simplex = OpenSimplex::new();
  let rand = rand::thread_rng().gen();
  let fbm = Fbm::new().set_seed(rand);
  let noise = Add::new(&fbm, &open_simplex);
  let render = PlaneMapBuilder::new(&noise)
    .set_size(options.get_width(), options.get_height())
    .build();
  let (map_x, map_y) = render.size();
  let map_size = (map_x * map_y) as i32;

  (0..map_size).into_par_iter().for_each(|i| {
    let x = i as usize % options.get_width();
    let y = (i as usize - x)/options.get_width();
    let val = (render.get_value(x,y) * 100 as f64).abs() as i32;
    let mut block = block_cell.lock().unwrap();
    block.set_value(x,y,val);
  });

  return block_cell.lock().unwrap().clone();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_block() {
      let opt = &BlockOption::new(5000, 1000);
      let block = new_block(opt).clone();
      let values = block.get_values();
      let (width, height) = block.get_size();

      assert_eq!(width, 5000);
      assert_eq!(height, 1000);
      let mut img = image::RgbImage::new(width as u32, height as u32);

      for (x, y, pixel) in img.enumerate_pixels_mut() {
        let block_val = values[(x as usize + (y as usize * width))] as f32;
        let r = (5. * block_val).abs().ceil() as u8;
        let g = (2.2 * block_val).abs().ceil() as u8;
        let b = (2.2 * block_val).abs().ceil() as u8;
        *pixel = image::Rgb([r, g, b]);
      }
      img.save(format!("test_images/{}.png", uuid::Uuid::new_v4())).unwrap();
    }

    #[test]
    fn test_new_map() {
      let map = new_map(MapOption::new(2500, 1000));
    }
}
