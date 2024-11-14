// MIT License

Copyright (c) 2024 Based Labs

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::models::types::Coordinates;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct ExtendedNeighborhood {
    pub neighbors: HashMap<Uuid, (f64, f64)>, // (distance, influence_weight)
    pub radius: f64,
    pub max_neighbors: usize,
}

impl ExtendedNeighborhood {
    pub fn new(radius: f64, max_neighbors: usize) -> Self {
        Self {
            neighbors: HashMap::new(),
            radius,
            max_neighbors,
        }
    }

    pub fn calculate_influence(&self, distance: f64, source_phase: f64, target_phase: f64) -> f64 {
        let spatial_influence = (1.0 + (-4.0 * (self.radius - distance) / self.radius).exp()).recip();
        let phase_sync = 0.5 * (1.0 + (source_phase - target_phase).cos());
        spatial_influence * (0.7 + 0.3 * phase_sync)
    }

    pub fn update_neighbors(&mut self, cell_position: &Coordinates, other_cells: &[(Uuid, Coordinates)]) {
        let mut neighbor_distances: Vec<(Uuid, f64)> = other_cells
            .iter()
            .map(|(id, pos)| {
                let distance = calculate_3d_distance(cell_position, pos);
                (*id, distance)
            })
            .filter(|(_, distance)| *distance <= self.radius && *distance > 0.0)
            .collect();

        neighbor_distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        neighbor_distances.truncate(self.max_neighbors);

        self.neighbors.clear();
        for (id, distance) in neighbor_distances {
            let influence = self.calculate_influence(distance, 0.0, 0.0); // Default phase values for now
            self.neighbors.insert(id, (distance, influence));
        }
    }
}

#[derive(Clone)]
pub struct EnhancedCellState {
    pub energy: f64,
    pub activity_level: f64,
    pub stability: f64,
    pub phase: f64,
    pub phase_velocity: f64,
    pub coupling_strength: f64,
    pub adaptation_rate: f64,
}

impl EnhancedCellState {
    pub fn calculate_phase_coupling(&mut self, neighbor_phases: &[f64], weights: &[f64]) {
        let mut phase_diff_sum = 0.0;
        let mut total_weight = 0.0;
        
        for (&phase, &weight) in neighbor_phases.iter().zip(weights.iter()) {
            phase_diff_sum += weight * (phase - self.phase).sin();
            total_weight += weight;
        }
        
        if total_weight > 0.0 {
            self.phase_velocity += self.coupling_strength * phase_diff_sum / total_weight;
            self.phase += self.phase_velocity * self.adaptation_rate;
            self.phase = self.phase % (2.0 * std::f64::consts::PI);
            
            // Damping
            self.phase_velocity *= 0.9;
        }
    }
}

impl EnhancedCellState {
    pub fn new() -> Self {
        Self {
            energy: 100.0,
            activity_level: 0.0,
            stability: 1.0,
            phase: 0.0,
            phase_velocity: 0.0,
            coupling_strength: 0.5,
            adaptation_rate: 0.1,
        }
    }

    pub fn update(&mut self, neighborhood: &ExtendedNeighborhood, neighbor_states: &HashMap<Uuid, EnhancedCellState>) {
        let mut weighted_energy = 0.0;
        let mut weighted_activity = 0.0;
        let mut total_weight = 0.0;

        for (id, (_, weight)) in &neighborhood.neighbors {
            if let Some(neighbor) = neighbor_states.get(id) {
                weighted_energy += neighbor.energy * weight;
                weighted_activity += neighbor.activity_level * weight;
                total_weight += weight;
            }
        }

        if total_weight > 0.0 {
            self.energy = self.energy * 0.8 + (weighted_energy / total_weight) * 0.2;
            self.activity_level = self.activity_level * 0.7 + (weighted_activity / total_weight) * 0.3;
            self.stability = self.calculate_stability(neighborhood, neighbor_states);
            self.phase = (self.phase + 0.1) % (2.0 * std::f64::consts::PI);
        }
    }

    fn calculate_stability(&self, neighborhood: &ExtendedNeighborhood, neighbor_states: &HashMap<Uuid, EnhancedCellState>) -> f64 {
        let mut variance = 0.0;
        let mut count = 0;

        for (id, (_, weight)) in &neighborhood.neighbors {
            if let Some(neighbor) = neighbor_states.get(id) {
                let energy_diff = (self.energy - neighbor.energy).abs();
                variance += energy_diff * energy_diff * weight;
                count += 1;
            }
        }

        if count > 0 {
            1.0 / (1.0 + (variance / count as f64).sqrt())
        } else {
            1.0
        }
    }
}

#[derive(Debug)]
pub enum InteractionEffect {
    EnergyBoost(f64),
    SynchronizationBonus(f64),
    SpawnConditionsMet,
}

pub fn calculate_3d_distance(a: &Coordinates, b: &Coordinates) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}
