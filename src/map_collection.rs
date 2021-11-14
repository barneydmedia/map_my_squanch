pub mod terrain_map2d;

use rayon::prelude::*;
use chrono::prelude::*;
use terrain_map2d::TerrainMap2D;
use std::sync::RwLock;
use rand::Rng;

#[derive()]
pub struct MapCollection2D {
    size: usize,
    x_size: usize,
    y_size: usize,
    map2d: Vec<TerrainMap2D>,
}

impl MapCollection2D {
    pub fn new(x_size: usize, y_size:usize, x_resolution:usize, y_resolution:usize) -> MapCollection2D {
        let mut map_vec = Vec::new();
        let size = x_size * y_size;

        for _ in 0..size {
            map_vec.push(TerrainMap2D::new(x_resolution, y_resolution));
        }
        
        MapCollection2D {
            size: x_size * y_size,
            x_size: x_size,
            y_size: y_size,
            map2d: map_vec,
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

    pub fn render(&mut self, closures: &Vec<RwLock<Box<fn(f64) -> f64>>>) -> () {
        (0..self.map2d.len()).for_each(|map_offset| {
            (0..self.size()).for_each(|iterator_offset| {
                (0..closures.len()).for_each(|closure_offset| {
                    let lock = &closures[closure_offset].read().unwrap();
                    let closure = &*lock;
                    let value = self.map2d[map_offset].get_by_index(iterator_offset);

                    // [iterator_offset][closure_offset]
                    self.map2d[map_offset].set_by_index(iterator_offset, closure(value));
                });
            });
        });
    }

    pub fn get(&mut self, x: usize, y: usize) -> Option<&mut TerrainMap2D> {
        let position = (y * self.x_size) + x;
        
        if position >= self.size {
            return None;
        }

        return Some(&mut self.map2d[(y * self.x_size) + x]);
    }

    pub fn add_fbm_noise(&mut self) {
        (0 .. self.map2d.len()).for_each(|i| {
            self.map2d[i].add_fbm_noise();
        });
    }

    pub fn add_open_simplex_noise(&mut self) {
        (0 .. self.map2d.len()).for_each(|i| {
            self.map2d[i].add_open_simplex_noise();
        });
    }

    pub fn get_values(&self) -> Vec<f64> {
        let mut raster = vec!();
        
        for map in &self.map2d {
            let value_map = map.get_values();
            for value in value_map {
                raster.push(value);
            }
        }

        return raster;
    }

    pub fn make_colors(&self, size: i32) -> Vec<(u8, u8, u8)> {
        let mut colors = vec!();
        let mut rng = rand::thread_rng();

        (0 .. size).for_each(|i| {
            let r = rng.gen_range(0..254) as u8;
            let g = rng.gen_range(0..254) as u8;
            let b = rng.gen_range(0..254) as u8;

            colors.push( (r, g, b) );
        });

        colors
    }

    // renders an image to disk from TerrainMap values
    pub fn render_image(&self, number_of_colors: i32) {
        let (resolution_x, resolution_y) = self.map2d[0].size();
        let mut img = image::RgbImage::new((self.x_size * resolution_x) as u32, (self.y_size * resolution_y) as u32);
        let values = self.get_values();
        let mut sorted = values.clone();
        sorted.sort_unstable_by(|a,b| a.partial_cmp(b).unwrap());
        
        // get basic stats
        let min = *sorted.first().unwrap();
        let max = *sorted.last().unwrap();
        let difference = max - min;
        let colors = self.make_colors(number_of_colors);
        let dynamic_range = (difference.abs() / number_of_colors as f64).round();
        let mut level_adjustment = 0.0;
        
        if min < 0.0 {
            level_adjustment = min.abs();
        }
        if max < 0.0 {
            panic!("Map must contain at least one positive value. max: {}, min: {}", max, min);
        }

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let value = values[(x as usize + (y as usize * self.x_size))] as f32;
            let adjusted_value: f64 = ((value as f64 + level_adjustment) / number_of_colors as f64).round();
            let color_position = (adjusted_value / number_of_colors as f64).round();
            let (r,g,b) = colors[color_position as usize];
            *pixel = image::Rgb([r,g,b]);
        }

        let datetime = Local::now();
        let time = datetime.to_rfc3339_opts(SecondsFormat::Secs, true);
        img.save(format!("test_images/{}-{}.png", time, uuid::Uuid::new_v4())).unwrap();
    }
}
