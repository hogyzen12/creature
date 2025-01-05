use async_trait::async_trait;
use std::error::Error;
use crate::models::types::{CellContext, RealTimeContext, Thought, Plan, DimensionalPosition};
use uuid::Uuid;
use std::collections::HashMap;

#[async_trait]
pub trait ModelClient: Send + Sync {
    async fn generate_contextual_thought(
        &self,
        cell_context: &CellContext,
        real_time_context: &RealTimeContext,
        colony_mission: &str,
    ) -> Result<(String, f64, Vec<String>), Box<dyn Error>>;

    async fn create_plan(&self, thoughts: &[Thought]) -> Result<Plan, Box<dyn Error>>;

    async fn evaluate_dimensional_state(
        &self,
        position: &DimensionalPosition,
        thoughts: &[Thought],
        plans: &[Plan],
    ) -> Result<(f64, f64), Box<dyn Error>>;

    async fn compress_memories(&self, memories: &[String]) -> Result<String, Box<dyn Error>>;

    // Add these new required methods
    async fn gather_real_time_context(
        &self,
        cell_thoughts: Option<Vec<String>>,
    ) -> Result<RealTimeContext, Box<dyn Error>>;

    async fn generate_contextual_thoughts_batch(
        &self,
        cell_contexts: &[(Uuid, &CellContext)],
        real_time_context: &RealTimeContext,
        colony_mission: &str,
        recent_thoughts: &[Thought],
    ) -> Result<HashMap<Uuid, Vec<(String, f64, Vec<String>)>>, Box<dyn Error>>;

    async fn query_llm(&self, prompt: &str) -> Result<String, Box<dyn Error>>;
}