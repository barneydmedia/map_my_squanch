#[derive(Debug, Clone, Copy)]
pub struct MapOption {
  block_x_size: usize,
  block_y_size: usize,
}

impl MapOption {
  pub fn new(block_x_size:usize, block_y_size:usize) -> MapOption{
    MapOption {
      block_x_size: block_x_size,
      block_y_size: block_y_size,
    }
  }

  pub fn y_size(self) -> usize {
    self.block_y_size
  }

  pub fn x_size(self) -> usize {
    self.block_x_size
  }
}
