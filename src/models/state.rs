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
}

#[derive(Serialize, Deserialize)]
pub struct ColonyState {
    pub timestamp: DateTime<Utc>,
    pub cells: HashMap<Uuid, CellState>,
    pub total_cycles: u32,
    pub mission: String,
    pub lenia_world: Option<LeniaWorldState>,
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
