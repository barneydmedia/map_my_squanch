use noise::{NoiseFn, OpenSimplex};

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

  pub fn get(&self) -> Vec<i32> {
    return self.values.clone();
  }

  pub fn set(&mut self, x:usize, y:usize, value: i32) {
    self.values[x + (y * &self.x_size)] = value;
  }

  pub fn value(&self, x:usize, y:usize) -> i32 {
    return self.values[x + (y * self.x_size)];
  }

  pub fn size(&self) -> (usize, usize) {
    return (self.x_size, self.y_size);
  }

  // pub fn addOpenSimplexNoise(&self) -> () {
  //   let open_simplex = OpenSimplex::new();
  //   let rand = rand::thread_rng().gen();

  //   let (map_x, map_y) = render.size();
  //   let map_size = (map_x * map_y) as i32;

  //   (0..map_size).into_par_iter().for_each(|i| {
  //     let x = i as usize % options.x_size();
  //     let y = (i as usize - x)/options.x_size();
  //     let val = (render.get_value(x,y) * 100 as f64).abs() as i32;
  //     let mut block = block_cell.lock().unwrap();
  //     block.set_value(x,y,val);
  //   });

  //   return block_cell.lock().unwrap().clone();
  // }
}
