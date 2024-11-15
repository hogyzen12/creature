// MIT License

/*Copyright (c) 2024 Based Labs

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.*/

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealTimeContext {
    pub timestamp: DateTime<Utc>,
    pub market_trends: Vec<String>,
    pub current_events: Vec<String>,
    pub technological_developments: Vec<String>,
    pub user_interactions: Vec<String>,
    pub environmental_data: HashMap<String, String>,
    pub mission_progress: Vec<String>,
}

impl Default for RealTimeContext {
    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            market_trends: Vec::new(),
            current_events: Vec::new(),
            technological_developments: Vec::new(),
            user_interactions: Vec::new(),
            environmental_data: HashMap::new(),
            mission_progress: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DimensionalPosition {
    pub emergence: f64,         // Dimension 1: -100 to 100 (Emergence vs Reduction)
    pub coherence: f64,         // Dimension 2: -100 to 100 (Coherence vs Chaos)
    pub resilience: f64,        // Dimension 3: -100 to 100 (Resilience vs Fragility)
    pub intelligence: f64,      // Dimension 4: -100 to 100 (Intelligence vs Instinct)
    pub efficiency: f64,        // Dimension 5: -100 to 100 (Efficiency vs Waste)
    pub integration: f64,       // Dimension 6: -100 to 100 (Integration vs Isolation)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CellContext {
    pub current_focus: String,
    pub active_research_topics: Vec<String>,
    pub recent_discoveries: Vec<String>,
    pub collaboration_history: Vec<String>,
    pub performance_metrics: HashMap<String, f64>,
    pub evolution_stage: u32,
    pub energy_level: f64,
    pub dimensional_position: DimensionalPosition,
    pub dopamine: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coordinates {
    // Spatial coordinates
    pub x: f64,
    pub y: f64,
    pub z: f64,
    
    // Heatmap values
    pub heat: f64,  // Overall activity/energy level (0.0 to 1.0)
    
    // Dimensional scores (-100.0 to 100.0)
    pub emergence_score: f64,
    pub coherence_score: f64,
    pub resilience_score: f64,
    pub intelligence_score: f64,
    pub efficiency_score: f64,
    pub integration_score: f64,
}

impl Default for Coordinates {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            heat: 0.0,
            emergence_score: 0.0,
            coherence_score: 0.0,
            resilience_score: 0.0,
            intelligence_score: 0.0,
            efficiency_score: 0.0,
            integration_score: 0.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Thought {
    pub id: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub relevance_score: f64,
    pub context_tags: Vec<String>,
    pub real_time_factors: Vec<String>,
    pub confidence_score: f64,
    pub ascii_visualization: Option<String>,
    pub referenced_thoughts: Vec<(Uuid, String)>, // (cell_id, thought_id)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanNode {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub dependencies: Vec<Uuid>,
    pub estimated_completion: f64, // 0.0 to 1.0
    pub status: PlanNodeStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlanNodeStatus {
    Pending,
    InProgress,
    Completed,
    Blocked,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plan {
    pub id: Uuid,
    pub thoughts: Vec<Thought>,
    pub nodes: Vec<PlanNode>,
    pub summary: String,
    pub score: f64,
    pub participating_cells: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub status: PlanStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlanStatus {
    Proposed,
    InProgress,
    Completed,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CellStatistics {
    pub thoughts_generated: u32,
    pub successful_plans: u32,
    pub failed_plans: u32,
    pub evolution_count: u32,
    pub total_energy_consumed: f64,
    pub highest_relevance_score: f64,
    pub average_confidence: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColonyStatistics {
    pub total_cells: u32,
    pub total_thoughts: u32,
    pub total_plans: u32,
    pub successful_plans: u32,
    pub failed_plans: u32,
    pub average_cell_energy: f64,
    pub highest_evolution_stage: u32,
    pub total_cycles: u32,
}
