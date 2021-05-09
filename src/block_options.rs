#[derive(Debug, Clone, Copy)]
pub struct BlockOption {
  block_x_size: usize,
  block_y_size: usize,
}

impl BlockOption {
  pub fn new(block_x_size:usize, block_y_size:usize) -> BlockOption {
    BlockOption {
      block_x_size: block_x_size,
      block_y_size: block_y_size,
    }
  }

  pub fn y_size(self) -> usize {
    self.block_y_size.clone()
  }

  pub fn x_size(self) -> usize {
    self.block_x_size.clone()
  }
}