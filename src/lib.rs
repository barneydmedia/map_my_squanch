#![feature(trace_macros)]
#![feature(cell_update)]
#![feature(test)]
pub struct MapGen;

extern crate bencher;

mod map_option;
mod block_options;
mod map_collection;

use noise::{Fbm, utils::NoiseMapBuilder, utils::*, OpenSimplex, Add, Seedable};
use noise::utils::PlaneMapBuilder;
use std::cell::{RefCell};
use std::rc::{Rc};
use std::sync::{RwLock, Arc, Mutex};
use map_collection::{MapCollection2D, map2d::terrain_map2d::TerrainMap2D};
use map_option::{MapOption};
use block_options::{BlockOption};
use rand::Rng;
use rayon::prelude::*;
use bencher::Bencher;



impl MapGen {

}

pub fn map_collection(x_size:usize, y_size:usize) -> MapCollection2D {
  return MapCollection2D::new(x_size, y_size);
}

pub fn map(options: MapOption) -> TerrainMap2D {
  return TerrainMap2D::new(options.x_size(), options.y_size());
}

pub fn block(options: &BlockOption) -> TerrainMap2D {
  let block_cell = Arc::new(Mutex::new(TerrainMap2D::new(options.x_size(), options.y_size())));

  let open_simplex = OpenSimplex::new();
  let rand = rand::thread_rng().gen();
  let fbm = Fbm::new().set_seed(rand);
  let noise = Add::new(&fbm, &open_simplex);
  let render = PlaneMapBuilder::new(&noise)
    .set_size(options.x_size(), options.y_size())
    .build();
  let (map_x, map_y) = render.size();
  let map_size = (map_x * map_y) as i32;

  (0..map_size).into_par_iter().for_each(|i| {
    let x = i as usize % options.x_size();
    let y = (i as usize - x)/options.x_size();
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
    fn test_block() {
      let opt = &BlockOption::new(5000, 1000);
      let block = block(opt).clone();
      let values = block.values();
      let (x_sizex_size, y_size) = block.size();

      assert_eq!(x_sizex_size, 5000);
      assert_eq!(y_size, 1000);
      let mut img = image::RgbImage::new(x_sizex_size as u32, y_size as u32);

      for (x, y, pixel) in img.enumerate_pixels_mut() {
        let block_val = values[(x as usize + (y as usize * x_sizex_size))] as f32;
        let r = (5. * block_val).abs().ceil() as u8;
        let g = (2.2 * block_val).abs().ceil() as u8;
        let b = (2.2 * block_val).abs().ceil() as u8;
        *pixel = image::Rgb([r, g, b]);
      }
      img.save(format!("test_images/{}.png", uuid::Uuid::new_v4())).unwrap();
    }

    #[test]
    fn test_map() {
      let map = map(MapOption::new(2500, 1000));
    }
}
