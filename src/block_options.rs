#[derive(Debug, Clone, Copy)]
pub struct BlockOption {
  block_width: usize,
  block_height: usize,
}

impl BlockOption {
  pub fn new(block_width:usize, block_height:usize) -> BlockOption {
    BlockOption {
      block_width: block_width,
      block_height: block_height,
    }
  }

  pub fn get_height(self) -> usize {
    self.block_height.clone()
  }

  pub fn get_width(self) -> usize {
    self.block_width.clone()
  }
}
