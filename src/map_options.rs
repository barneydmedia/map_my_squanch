#[derive(Debug, Clone, Copy)]
pub struct MapOption {
  block_width: usize,
  block_height: usize,
}

impl MapOption {
  pub fn new(block_width:usize, block_height:usize) -> MapOption{
    MapOption {
      block_width: block_width,
      block_height: block_height,
    }
  }

  pub fn get_height(self) -> usize {
    self.block_height
  }

  pub fn get_width(self) -> usize {
    self.block_width
  }
}
