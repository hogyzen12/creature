// MIT License

Copyright (c) 2024 Based Labs

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::models::types::Coordinates;
use ndarray::Array3;

#[derive(Clone, Debug)]
pub struct LeniaParams {
    pub kernel_radius: f64,
    pub kernel_sigma: f64,
    pub growth_mu: f64,
    pub growth_sigma: f64,
    pub time_step: f64,
    pub grid_size: usize,
    pub dt: f64,
}

impl Default for LeniaParams {
    fn default() -> Self {
        Self {
            kernel_radius: 10.0,
            kernel_sigma: 3.0,
            growth_mu: 0.15,
            growth_sigma: 0.015,
            time_step: 0.1,
            grid_size: 256,
            dt: 0.1,
        }
    }
}

#[derive(Clone)]
pub struct LeniaWorld {
    pub grid: Array3<f64>,
    pub params: LeniaParams,
    kernel: Array3<f64>,
}

impl LeniaWorld {
    pub fn new(params: LeniaParams) -> Self {
        let size = params.grid_size;
        let grid = Array3::zeros((size, size, size));
        let kernel = Self::create_kernel(&params);
        
        Self {
            grid,
            params,
            kernel,
        }
    }

    fn create_kernel(params: &LeniaParams) -> Array3<f64> {
        let size = params.grid_size;
        let mut kernel = Array3::zeros((size, size, size));
        let center = size / 2;
        
        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    let dx = (x as f64 - center as f64) / params.kernel_radius;
                    let dy = (y as f64 - center as f64) / params.kernel_radius;
                    let dz = (z as f64 - center as f64) / params.kernel_radius;
                    let distance = (dx * dx + dy * dy + dz * dz).sqrt();
                    
                    if distance < 1.0 {
                        kernel[[x, y, z]] = (-distance.powi(2) / 
                            (2.0 * params.kernel_sigma.powi(2))).exp();
                    }
                }
            }
        }
        
        // Normalize kernel
        let sum = kernel.sum();
        kernel.mapv_inplace(|x| x / sum);
        kernel
    }

    fn growth_function(&self, u: f64) -> f64 {
        let normalized_u = (u - self.params.growth_mu) / self.params.growth_sigma;
        2.0 * (-normalized_u.powi(2)).exp() - 1.0
    }

    pub fn step(&mut self) {
        let mut new_grid = self.grid.clone();
        
        // Compute convolution for each cell
        for x in 0..self.params.grid_size {
            for y in 0..self.params.grid_size {
                for z in 0..self.params.grid_size {
                    let mut sum = 0.0;
                    
                    // Apply kernel
                    for kx in 0..self.params.grid_size {
                        for ky in 0..self.params.grid_size {
                            for kz in 0..self.params.grid_size {
                                let gx = (x + kx) % self.params.grid_size;
                                let gy = (y + ky) % self.params.grid_size;
                                let gz = (z + kz) % self.params.grid_size;
                                
                                sum += self.grid[[gx, gy, gz]] * 
                                      self.kernel[[kx, ky, kz]];
                            }
                        }
                    }
                    
                    // Apply growth function and update
                    let growth = self.growth_function(sum);
                    new_grid[[x, y, z]] = (self.grid[[x, y, z]] + 
                        self.params.dt * growth).clamp(0.0, 1.0);
                }
            }
        }
        
        self.grid = new_grid;
    }

    pub fn add_pattern(&mut self, pattern: &Array3<f64>, position: &Coordinates) {
        let px = position.x.round() as usize % self.params.grid_size;
        let py = position.y.round() as usize % self.params.grid_size;
        let pz = position.z.round() as usize % self.params.grid_size;
        
        let pattern_size = pattern.shape()[0];
        for x in 0..pattern_size {
            for y in 0..pattern_size {
                for z in 0..pattern_size {
                    let gx = (px + x) % self.params.grid_size;
                    let gy = (py + y) % self.params.grid_size;
                    let gz = (pz + z) % self.params.grid_size;
                    self.grid[[gx, gy, gz]] = pattern[[x, y, z]];
                }
            }
        }
    }

    pub fn get_state_at(&self, position: &Coordinates) -> f64 {
        let x = position.x.round() as usize % self.params.grid_size;
        let y = position.y.round() as usize % self.params.grid_size;
        let z = position.z.round() as usize % self.params.grid_size;
        self.grid[[x, y, z]]
    }
}
