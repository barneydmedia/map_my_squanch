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

  pub fn values(&self) -> Vec<i32> {
    return self.values.clone();
  }

  pub fn set_value(&mut self, x:usize, y:usize, value: i32) {
    self.values[x + (y * &self.x_size)] = value;
  }

  pub fn value(&self, x:usize, y:usize) -> i32 {
    return self.values[x + (y * self.x_size)];
  }

  pub fn size(&self) -> (usize, usize) {
    return (self.x_size, self.y_size);
  }
}


