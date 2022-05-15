pub mod terrain_map2d;

use chrono::prelude::*;
use plotters::prelude::*;
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
                    let new_value = closure(value);

                    self.map2d[map_offset].set_by_index(iterator_offset, new_value);
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

    pub fn get_resolution(&self) -> (usize, usize) {
        let (x, y) = self.map2d[0].size();
        (self.x_size * x, self.y_size * y)
    }

    pub fn make_colors(&self, size: i32) -> Vec<(u8, u8, u8)> {
        let mut colors = vec!();
        let mut rng = rand::thread_rng();

        (0 .. size).for_each(|_| {
            let r = rng.gen_range(0..254) as u8;
            let g = rng.gen_range(0..254) as u8;
            let b = rng.gen_range(0..254) as u8;

            colors.push( (r, g, b) );
        });

        colors
    }

    // renders an image to disk from TerrainMap values
    pub fn render_image(&self, number_of_colors: i32) {
        let (resolution_x, resolution_y) = self.get_resolution();
        let mut img = image::RgbImage::new((self.x_size * resolution_x) as u32, (self.y_size * resolution_y) as u32);
        let values = self.get_values();
        let mut sorted = values.clone();
        sorted.sort_unstable_by(|a,b| a.partial_cmp(b).unwrap());
        
        // get basic stats
        let min = *sorted.first().unwrap();
        let max = *sorted.last().unwrap();
        let colors = self.make_colors(number_of_colors);
        let value_range = max - min.abs();
        let color_constant = number_of_colors as f64 / value_range;
        let mut level_adjustment = 0.0;

        if min < 0.0 {
            level_adjustment = min.abs();
        }
        if max < 0.0 {
            panic!("Map must contain at least one positive value. max: {}, min: {}", max, min);
        }

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let value_index = (y as usize * self.x_size) + x as usize;
            let mut value = *values.last().unwrap();

            if value_index < values.len() {
                value = values[value_index];
            }

            let adjusted_value = value + level_adjustment;
            let color_position = ((adjusted_value * color_constant).floor() - 1.0) as usize;
            let (r,g,b) = colors[color_position];
            *pixel = image::Rgb([r,g,b]);
        }

        let datetime = Local::now();
        let time = datetime.to_rfc3339_opts(SecondsFormat::Secs, true);
        img.save(format!("test_images/{}-{}.png", time, uuid::Uuid::new_v4())).unwrap();
    }

    pub fn render_graph(&self) {
        let datetime = Local::now();
        let time = datetime.to_rfc3339_opts(SecondsFormat::Secs, true);
        let filename = &format!("test_images/{}-{}-graph.png", time, uuid::Uuid::new_v4());
        let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
        let values = self.get_values();
        let mut sorted = values.clone();
        sorted.sort_unstable_by(|a,b| a.partial_cmp(b).unwrap());let values = self.get_values();
        let min = *sorted.first().unwrap();
        let max = *sorted.last().unwrap();
        let value_range = max - min.abs();
        let mut data_points = vec!();

        root.fill(&WHITE).unwrap();
        let random_points: &Vec<(f64, f64)> = {
            let values = self.get_values();

            values.iter().enumerate().for_each(|(iter, val)| {
                data_points.push((iter as f64, *val));
            });

            &data_points
        };

        let areas = root.split_by_breakpoints([944 as i32], [80 as i32]);
        let mut x_hist_ctx = ChartBuilder::on(&areas[0])
            .y_label_area_size(40 as i32)
            .build_cartesian_2d((0.0..values.len() as f64).step((values.len() / 10) as f64).use_round().into_segmented(), 0.0..values.len() as f64).unwrap();
        let mut y_hist_ctx = ChartBuilder::on(&areas[3])
            .x_label_area_size(40 as i32)
            .build_cartesian_2d(0..250 as i32, (0.0..max).step(value_range / 10.0).use_round()).unwrap();
        let mut scatter_ctx = ChartBuilder::on(&areas[2])
            .x_label_area_size(40 as i32)
            .y_label_area_size(40 as i32)
            .build_cartesian_2d(0f64..1f64, 0f64..1f64).unwrap();
        scatter_ctx
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()
            .unwrap();
        scatter_ctx.draw_series(
            random_points
                .iter()
                .map(|(x, y)| Circle::new((*x, *y), 2 as i32, GREEN.filled())),
        ).unwrap();
        let y_value = 1.0;
        let x_hist = Histogram::vertical(&x_hist_ctx)
            .style(GREEN.filled())
            .margin(0)
            .data(random_points.iter().map(|(x, _)| {(*x, y_value)}));
        let y_hist = Histogram::horizontal(&y_hist_ctx)
            .style(GREEN.filled())
            .margin(0)
            .data(random_points.iter().map(|(_, y)| (*y, 1)));
        x_hist_ctx.draw_series(x_hist).unwrap();
        y_hist_ctx.draw_series(y_hist).unwrap();

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    }
}
