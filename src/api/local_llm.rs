use crate::models::types::{CellContext, RealTimeContext, Thought, Plan, DimensionalPosition};
use crate::api::model_client::ModelClient;
use async_trait::async_trait;
use chrono::Utc;
use reqwest;
use serde_json::json;
use std::error::Error;
use std::collections::HashMap;
use uuid::Uuid;

pub struct LocalLLMClient {
    client: reqwest::Client,
    base_url: String,
}

impl LocalLLMClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: "http://localhost:8000".to_string(),
        })
    }

    async fn generate_response(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        let response = self.client
            .post(&format!("{}/generate", self.base_url))
            .json(&json!({
                "prompt": prompt
            }))
            .send()
            .await?;

        // Print the raw response for debugging
        println!("Raw response status: {}", response.status());
        let response_text = response.text().await?;
        println!("Raw response body: {}", response_text);

        // Parse the response
        let parsed: serde_json::Value = serde_json::from_str(&response_text)?;
        
        Ok(parsed["response"].as_str()
            .ok_or("Invalid response format")?
            .to_string())
    }
}

#[async_trait]
impl ModelClient for LocalLLMClient {
    async fn generate_contextual_thought(
        &self,
        cell_context: &CellContext,
        real_time_context: &RealTimeContext,
        colony_mission: &str,
    ) -> Result<(String, f64, Vec<String>), Box<dyn Error>> {
        let prompt = format!(
            r#"Generate a philosophical thought about this mission and context.
Mission: {}
Current Focus: {}
Energy Level: {}

Format your response exactly as:
THOUGHT: [Your philosophical insight]
RELEVANCE: [0.0-1.0]
FACTORS: [Three factors, comma-separated]"#,
            colony_mission,
            cell_context.current_focus,
            cell_context.energy_level
        );

        let response = self.generate_response(&prompt).await?;
        println!("Model response: {}", response);  // Debug print

        // Default values
        let mut thought = String::new();
        let mut relevance = 0.5;
        let mut factors = Vec::new();

        // Parse the response
        for line in response.lines() {
            let line = line.trim();
            if line.starts_with("THOUGHT:") {
                thought = line.trim_start_matches("THOUGHT:").trim().to_string();
            } else if line.starts_with("RELEVANCE:") {
                relevance = line.trim_start_matches("RELEVANCE:")
                    .trim()
                    .parse()
                    .unwrap_or(0.5);
            } else if line.starts_with("FACTORS:") {
                factors = line.trim_start_matches("FACTORS:")
                    .trim()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
            }
        }

        // If thought is empty, use the entire response as the thought
        if thought.is_empty() {
            thought = response;
        }

        // Ensure we have at least 3 factors
        while factors.len() < 3 {
            factors.push("general insight".to_string());
        }

        Ok((thought, relevance, factors))
    }

    async fn create_plan(&self, thoughts: &[Thought]) -> Result<Plan, Box<dyn Error>> {
        let prompt = format!(
            r#"Create a plan based on these thoughts:
{}

Format response as:
SUMMARY: [Plan summary]
STEPS: [Numbered list of steps]
SCORE: [0.0-1.0]"#,
            thoughts.iter().map(|t| t.content.clone()).collect::<Vec<_>>().join("\n")
        );

        let response = self.client
            .post(&format!("{}/generate", self.base_url))
            .json(&json!({
                "prompt": prompt
            }))
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        let response_text = result["response"].as_str()
            .ok_or("Invalid response format")?;

        // Create a basic plan structure
        Ok(Plan {
            id: Uuid::new_v4(),
            thoughts: thoughts.to_vec(),
            nodes: vec![],
            summary: response_text.to_string(),
            score: 0.5,
            participating_cells: vec![],
            created_at: Utc::now(),
            status: crate::models::types::PlanStatus::Proposed,
        })
    }

    async fn evaluate_dimensional_state(
        &self,
        position: &DimensionalPosition,
        thoughts: &[Thought],
        plans: &[Plan],
    ) -> Result<(f64, f64), Box<dyn Error>> {
        // Return default values for now
        Ok((0.5, 0.5))
    }

    async fn compress_memories(&self, memories: &[String]) -> Result<String, Box<dyn Error>> {
        let prompt = format!(
            r#"Compress these memories into a concise summary:
{}

Format response as a single paragraph."#,
            memories.join("\n")
        );

        let response = self.client
            .post(&format!("{}/generate", self.base_url))
            .json(&json!({
                "prompt": prompt
            }))
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        Ok(result["response"].as_str()
            .ok_or("Invalid response format")?
            .to_string())
    }

    async fn gather_real_time_context(
        &self,
        cell_thoughts: Option<Vec<String>>,
    ) -> Result<RealTimeContext, Box<dyn Error>> {
        // Return a default context for now
        Ok(RealTimeContext::default())
    }

    async fn generate_contextual_thoughts_batch(
        &self,
        cell_contexts: &[(Uuid, &CellContext)],
        real_time_context: &RealTimeContext,
        colony_mission: &str,
        recent_thoughts: &[Thought],
    ) -> Result<HashMap<Uuid, Vec<(String, f64, Vec<String>)>>, Box<dyn Error>> {
        let mut results = HashMap::new();
        
        for (id, context) in cell_contexts {
            let (thought, relevance, factors) = self.generate_contextual_thought(
                context,
                real_time_context,
                colony_mission
            ).await?;
            
            results.insert(*id, vec![(thought, relevance, factors)]);
        }
        
        Ok(results)
    }

    async fn query_llm(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        let response = self.client
            .post(&format!("{}/generate", self.base_url))
            .json(&json!({
                "prompt": prompt
            }))
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        Ok(result["response"].as_str()
            .ok_or("Invalid response format")?
            .to_string())
    }
}