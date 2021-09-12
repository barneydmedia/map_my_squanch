pub mod map2d;

use map2d::Map2D;
use rayon::prelude::*;
use chrono::prelude::*;

use self::map2d::terrain_map2d::TerrainMap2D;

#[derive()]
pub struct MapCollection2D {
    size: usize,
    x_size: usize,
    y_size: usize,
    map2d: Vec<Map2D>,
    x_resolution: usize, 
    y_resolution: usize,
}

impl MapCollection2D {
    fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
        nested.into_iter().flatten().collect()
    }

    pub fn new(x_size: usize, y_size:usize, x_resolution:usize, y_resolution:usize) -> MapCollection2D {
        let mut map_vec = Vec::new();
        let size = x_size * y_size;

        for i in 0..size {
            map_vec.push(Map2D::new(x_resolution, y_resolution));
        }
        
        MapCollection2D {
            size: x_size * y_size,
            x_size: x_size,
            y_size: y_size,
            map2d: map_vec,
            x_resolution: x_resolution,
            y_resolution: y_resolution,
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

    pub fn render(&mut self, closures: &Vec<fn(&MapCollection2D)>) -> () {
        (0..self.map2d.len()).into_par_iter().for_each(|map_offset| {
            (0..self.size()).into_par_iter().for_each(|pixel_offset| {
                (0..closures.len()).for_each(|closure_offset| {
                    closures[closure_offset](&self);
                });
            });
        });
    }

    pub fn get(&mut self, x: usize, y: usize) -> Option<&mut Map2D> {
        let position = (y * self.x_size) + x;
        
        if position >= 0 && position < self.size {
            return Some(&mut self.map2d[(y * self.x_size) + x]);
        } else {
            return None;
        }
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

    pub fn rasterize(&self) -> Vec<i32> {
        let mut raster = vec!();
        
        for map in &self.map2d {
            for value in map.rasterize() {
                raster.push(value);
            }
        }

        return raster;
    }


    pub fn render_image(&self) {
        let mut img = image::RgbImage::new(self.x_size as u32, self.y_size as u32);
        let values = self.rasterize();

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let block_val = values[(x as usize + (y as usize * self.x_size))] as f32;
            let r = (5. * block_val).abs().ceil() as u8;
            let g = (2.2 * block_val).abs().ceil() as u8;
            let b = (2.2 * block_val).abs().ceil() as u8;
            *pixel = image::Rgb([r, g, b]);
        }
        let datetime = Local::now();
        let time = datetime.to_rfc3339_opts(SecondsFormat::Secs, true);
        img.save(format!("test_images/{}-{}.png", time, uuid::Uuid::new_v4())).unwrap();
    }
}
