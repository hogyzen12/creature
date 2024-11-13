use crate::models::types::{Plan, PlanStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanAnalysis {
    pub cycle_id: String,
    pub timestamp: DateTime<Utc>,
    pub total_plans: usize,
    pub successful_plans: usize,
    pub failed_plans: usize,
    pub average_score: f64,
    pub best_plan_id: Option<Uuid>,
    pub best_plan_score: f64,
    pub best_plan_summary: String,
}

impl PlanAnalysis {
    pub fn analyze_plans(plans: &[Plan], cycle_id: &str) -> Self {
        let total = plans.len();
        let successful = plans.iter()
            .filter(|p| matches!(p.status, PlanStatus::Completed))
            .count();
        let failed = plans.iter()
            .filter(|p| matches!(p.status, PlanStatus::Failed))
            .count();
            
        let avg_score = if total > 0 {
            plans.iter().map(|p| p.score).sum::<f64>() / total as f64
        } else {
            0.0
        };

        let best_plan = plans.iter()
            .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

        Self {
            cycle_id: cycle_id.to_string(),
            timestamp: Utc::now(),
            total_plans: total,
            successful_plans: successful,
            failed_plans: failed,
            average_score: avg_score,
            best_plan_id: best_plan.map(|p| p.id),
            best_plan_score: best_plan.map(|p| p.score).unwrap_or(0.0),
            best_plan_summary: best_plan.map(|p| p.summary.clone()).unwrap_or_default(),
        }
    }

    pub fn save_to_file(&self, base_path: &Path) -> std::io::Result<()> {
        let analysis_dir = base_path.join("analysis");
        fs::create_dir_all(&analysis_dir)?;
        
        let filename = format!("analysis_{}.json", self.cycle_id);
        let file_path = analysis_dir.join(filename);
        
        let json = serde_json::to_string_pretty(self)?;
        fs::write(file_path, json)?;
        
        Ok(())
    }
}

pub fn save_plan_to_file(plan: &Plan, base_path: &Path, cycle_id: &str) -> std::io::Result<()> {
    let plans_dir = base_path.join("plans").join(cycle_id);
    fs::create_dir_all(&plans_dir)?;
    
    let filename = format!("plan_{}.json", plan.id);
    let file_path = plans_dir.join(filename);
    
    let json = serde_json::to_string_pretty(plan)?;
    fs::write(file_path, json)?;
    
    Ok(())
}
