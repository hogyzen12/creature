// MIT License

Copyright (c) 2024 Based Labs

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;
use crate::models::types::{Plan, Thought, DimensionalPosition};

#[derive(Serialize, Deserialize)]
pub struct CellState {
    pub id: Uuid,
    pub energy: f64,
    pub thoughts: Vec<Thought>,
    pub current_plan: Option<Plan>,
    pub dimensional_position: DimensionalPosition,
    pub dopamine: f64,
    pub stability: f64,
    pub phase: f64,
    pub context_alignment_score: f64,
    pub mission_alignment_score: f64,
    pub lenia_state: f64,
    pub lenia_influence: f64,
    pub x: f64,
    pub y: f64, 
    pub z: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ColonyState {
    pub timestamp: DateTime<Utc>,
    pub cells: HashMap<Uuid, CellState>,
    pub total_cycles: u32,
    pub mission: String,
    pub lenia_world: Option<LeniaWorldState>,
    pub energy_grid: EnergyGridState,
}

#[derive(Serialize, Deserialize)]
pub struct EnergyGridState {
    pub size: usize,
    pub grid: Vec<f64>,  // Flattened 3D array of energy values
    pub cell_positions: HashMap<Uuid, (usize, usize, usize)>, // Maps cell IDs to grid coordinates
}

#[derive(Serialize, Deserialize)]
pub struct LeniaWorldState {
    pub grid: Vec<f64>,
    pub size: usize,
    pub growth_mu: f64,
    pub growth_sigma: f64,
}

impl ColonyState {
    pub fn save_to_file(&self, path: &Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> std::io::Result<Self> {
        let json = fs::read_to_string(path)?;
        let state = serde_json::from_str(&json)?;
        Ok(state)
    }
}
