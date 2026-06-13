#[derive(Debug, Clone)]
pub struct InferenceEngine;

impl InferenceEngine {
    pub fn supports_multimodal(&self) -> bool { true }
    pub fn capability_score(&self, _task: &Task) -> f64 { 1.0 }
    pub fn cost_per_million(&self) -> f64 { 1.0 }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub max_tokens: u64,
    pub latency_budget_us: u64,
}

pub struct EngineRouter;