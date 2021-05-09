pub mod terrain_map2d;

use terrain_map2d::TerrainMap2D;
#[derive(Debug, Clone)]
pub struct Map2D {
  blocks: Vec<TerrainMap2D>,
  block_x_size: usize,
  block_y_size: usize,
}

impl Map2D {
    pub fn new(x_size: usize, y_size: usize) -> Map2D {
        Map2D {
            blocks: vec!(TerrainMap2D::new(x_size, y_size)),
            block_x_size: x_size,
            block_y_size: y_size,
        }
    }
}