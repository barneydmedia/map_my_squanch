#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TerrainMap2D {
  values: Vec<i32>,
  width: usize,
  height: usize,
}

impl TerrainMap2D {
  pub fn new(width:usize, height: usize) -> Self {
    let vec = vec![0; height * width];

    TerrainMap2D {
      values: vec,
      width: width,
      height: height,
    }
  }

  pub fn get_values(&self) -> Vec<i32> {
    return self.values.clone();
  }

  pub fn set_value(&mut self, x:usize, y:usize, value: i32) {
    self.values[x + (y * &self.width)] = value;
  }

  pub fn get_value(&self, x:usize, y:usize) -> i32 {
    return self.values[x + (y * self.width)];
  }

  pub fn get_size(&self) -> (usize, usize) {
    return (self.width, self.height);
  }
}


