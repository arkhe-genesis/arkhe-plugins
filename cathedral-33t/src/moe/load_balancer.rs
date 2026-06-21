//! Load balancer para MoE

use crate::moe::router::RoutingIndex;

pub struct LoadBalancer {
    pub capacity_factor: f32,
    pub expert_loads: Vec<u32>,
    pub total_tokens: u32,
}

impl LoadBalancer {
    pub fn new(capacity_factor: f32, num_experts: usize) -> Self {
        Self {
            capacity_factor,
            expert_loads: vec![0; num_experts],
            total_tokens: 0,
        }
    }

    pub fn apply(&mut self, routing: &[Vec<RoutingIndex>]) -> Vec<(usize, usize, f32)> {
        let mut result = Vec::new();
        let max_capacity = self.compute_max_capacity(routing.len());

        self.expert_loads.fill(0);
        self.total_tokens = routing.len() as u32;

        for (token_idx, indices) in routing.iter().enumerate() {
            for idx in indices {
                let expert_id = idx.expert_id;
                if self.expert_loads[expert_id] < max_capacity {
                    self.expert_loads[expert_id] += 1;
                    result.push((token_idx, expert_id, idx.weight));
                }
            }
        }

        result
    }

    fn compute_max_capacity(&self, num_tokens: usize) -> u32 {
        let avg_load = num_tokens as f32 / self.expert_loads.len() as f32;
        (avg_load * self.capacity_factor).ceil() as u32
    }

    pub fn compute_aux_loss(&self, coef: f32) -> f32 {
        if self.total_tokens == 0 {
            return 0.0;
        }

        let num_experts = self.expert_loads.len();
        let avg_load = self.total_tokens as f32 / num_experts as f32;
        let mut loss = 0.0f32;

        for &load in &self.expert_loads {
            let diff = load as f32 - avg_load;
            loss += diff * diff;
        }

        coef * loss / num_experts as f32
    }


    pub fn stats(&self) -> LoadBalancerStats {
        let max_load = self.expert_loads.iter().copied().max().unwrap_or(0);
        let min_load = self.expert_loads.iter().copied().min().unwrap_or(0);
        let avg_load = self.total_tokens as f32 / self.expert_loads.len() as f32;

        LoadBalancerStats {
            max_load,
            min_load,
            avg_load,
            imbalance_ratio: if avg_load > 0.0 {
                (max_load as f32 - min_load as f32) / avg_load
            } else {
                0.0
            },
        }
    }

    pub fn reset(&mut self) {
        self.expert_loads.fill(0);
        self.total_tokens = 0;
    }
}

#[derive(Debug, Clone)]
pub struct LoadBalancerStats {
    pub max_load: u32,
    pub min_load: u32,
    pub avg_load: f32,
    pub imbalance_ratio: f32,
}
