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
    pub id: Uuid,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub relevance_score: f64,
    pub context_tags: Vec<String>,
    pub real_time_factors: Vec<String>,
    pub confidence_score: f64,
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
