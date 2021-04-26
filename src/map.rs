#[path = "terrain_map.rs"]
mod terrain_map;

use terrain_map::{TerrainMap2D};

#[derive(Debug, Clone)]
pub struct Map {
  blocks: Vec<TerrainMap2D>,
  block_width: usize,
  block_height: usize,
}
