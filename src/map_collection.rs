pub mod map2d;

use map2d::Map2D;
use std::sync::{{RwLock, Arc}};

#[derive()]
pub struct MapCollection2D {
    size: usize,
    x_size: usize,
    y_size: usize,
    map: Vec<Map2D>,
}

impl MapCollection2D {
    pub fn new(x_size: usize, y_size:usize) -> MapCollection2D {
        let mut map_vec = Vec::new();
        let size = x_size * y_size;

        for i in 0..size {
            map_vec.push(Map2D::new(x_size, y_size));
        }
        
        MapCollection2D {
            size: x_size * y_size,
            x_size: x_size,
            y_size: y_size,
            map: map_vec,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn x_size(&self) -> usize {
        self.x_size
    }

    pub fn y_size(&self) -> usize {
        self.y_size
    }

    pub fn render(&self) -> &Self {
        &self
    }
}