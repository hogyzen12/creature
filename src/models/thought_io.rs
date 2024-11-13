use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInput {
    pub id: Uuid,
    pub event_type: String,
    pub description: String,
    pub probability: f64,
    pub timeframe: String,
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventOutput {
    pub id: Uuid,
    pub effect_type: String,
    pub description: String,
    pub impact_score: f64,
    pub dependencies: Vec<Uuid>,
    pub cascading_effects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtIO {
    pub inputs: Vec<EventInput>,
    pub outputs: Vec<EventOutput>,
    pub connection_graph: Vec<(Uuid, Uuid)>, // (input_id, output_id) pairs
}
