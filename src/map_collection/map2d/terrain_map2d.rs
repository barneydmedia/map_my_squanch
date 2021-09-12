use noise::{Fbm, OpenSimplex};
use noise::utils::{PlaneMapBuilder, NoiseMapBuilder};
use std::sync::Arc;
use std::sync::Mutex;
use rayon::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct TerrainMap2D {
  values: Vec<i32>,
  x_size: usize,
  y_size: usize,
}

impl TerrainMap2D {
  pub fn new(x_size:usize, y_size: usize) -> Self {
    let vec = vec![0; y_size * x_size];

    TerrainMap2D {
      values: vec,
      x_size: x_size,
      y_size: y_size,
    }
  }

  pub fn set(&mut self, x:usize, y:usize, value: i32) {
    self.values[x + (y * &self.x_size)] = value;
  }

  pub fn get(&self, x:usize, y:usize) -> i32 {
    return self.values[x + (y * self.x_size)];
  }

  pub fn rasterize(&self) -> Vec<i32> {
    self.values.clone()
  }

  pub fn size(&self) -> (usize, usize) {
    return (self.x_size.clone(), self.y_size.clone());
  }

  pub fn add_open_simplex_noise(&mut self) -> () {
    let open_simplex = OpenSimplex::new();
    let (map_x, map_y) = (self.x_size, self.y_size);
    let map_size = map_x * map_y;
    let render = PlaneMapBuilder::new(&open_simplex)
      .set_size(self.x_size, self.y_size)
      .build();

    (0..map_size).for_each(|i| {
      let x = i as usize % self.x_size;
      let y = (i as usize - x)/self.x_size;
      self.values[x + (y * &self.x_size)] = (render.get_value(x,y) * 100 as f64).abs() as i32;
    });
  }

  pub fn add_fbm_noise(&mut self) -> () {
    let open_simplex = Fbm::new();
    let (map_x, map_y) = (self.x_size, self.y_size);
    let map_size = map_x * map_y;
    let render = PlaneMapBuilder::new(&open_simplex)
      .set_size(self.x_size, self.y_size)
      .build();

    (0..map_size).for_each(|i| {
      let x = i as usize % self.x_size;
      let y = (i as usize - x)/self.x_size;
      self.values[x + (y * &self.x_size)] = (render.get_value(x,y) * 100 as f64).abs() as i32;
    });
  }
}