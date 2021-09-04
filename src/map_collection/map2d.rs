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
    
    pub fn get(&mut self, x: usize, y: usize) -> Option<&mut TerrainMap2D> {
        if x < 0 || y < 0 || x > self.block_x_size || y > self.block_y_size {
            let x_size = &self.block_x_size;

            return Some(&mut self.blocks[((y * x_size) + x)]);
        } else {
            return None;
        }
    }

    pub fn add_open_simplex_noise(&mut self) {
        (0 .. self.blocks.len()).for_each(|i| {
            self.blocks[i].add_open_simplex_noise();
        });
    }

    pub fn add_fbm_noise(&mut self) {
        (0 .. self.blocks.len()).for_each(|i| {
            self.blocks[i].add_fbm_noise();
        });
    }
}