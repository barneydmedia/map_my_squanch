pub mod terrain_map2d;

use terrain_map2d::TerrainMap2D;

#[derive(Debug, Clone)]
pub struct Map2D {
  iterator_current: usize,
  iterator_next: usize,
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
            iterator_current: 0,
            iterator_next: 1,
        }
    }
    
    pub fn get(&mut self, x: usize, y: usize) -> Option<&mut TerrainMap2D> {
        if x > self.block_x_size || y > self.block_y_size {
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

    pub fn rasterize(&self) -> Vec<i32> {
        let mut raster  = vec![];
        (0 .. self.blocks.len()).for_each(|i| {
            let value_set = self.blocks[i].rasterize();

            for value in value_set {
                raster.push(value);
            }
        });

        return raster;
    }
}

impl Iterator for Map2D {
    type Item = TerrainMap2D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iterator_next >= self.blocks.len() {
            return None;
        }

        let result = &self.blocks[self.iterator_current];
        self.iterator_current = self.iterator_next;
        self.iterator_next = self.iterator_next + 1;

        return Some(result.clone());
    }
}