//! Hierarchical Router for 4096 experts with deduplication.

use crate::tensor::Tensor;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RoutingIndex {
    pub expert_id: usize,
    pub weight: f32,
}

pub struct HierarchicalRouter {
    pub num_groups: usize,
    pub experts_per_group: usize,
    pub top_k: usize,
    pub hidden_size: usize,
    pub group_weights: Tensor,
    pub expert_weights: Tensor,
}

impl HierarchicalRouter {
    pub fn new(num_experts: usize, top_k: usize, hidden_size: usize) -> Self {
        let num_groups = 64;
        let experts_per_group = if num_experts >= num_groups { num_experts / num_groups } else { 1 };
        let num_groups = if num_experts >= num_groups { num_groups } else { num_experts };

        let group_weights = Tensor::randn((num_groups, hidden_size));
        let expert_weights = Tensor::randn((num_groups * experts_per_group, hidden_size));

        Self {
            num_groups,
            experts_per_group,
            top_k,
            hidden_size,
            group_weights,
            expert_weights,
        }
    }

    pub fn route(&self, x: &Tensor) -> Vec<Vec<RoutingIndex>> {
        let batch_size = x.nrows();
        let mut routing = Vec::with_capacity(batch_size);

        for i in 0..batch_size {
            let token = x.slice_row(i);
            let entry = self.route_single(&token);
            routing.push(entry);
        }

        routing
    }

    fn route_single(&self, token: &Tensor) -> Vec<RoutingIndex> {
        let group_logits = token.matmul(&self.group_weights.t());
        let group_logits = group_logits.slice_row(0).to_vec();

        let top_groups = self.top_k_indices(&group_logits, 2);

        let mut expert_indices = Vec::with_capacity(self.top_k);
        let mut seen = HashSet::new();

        let experts_per_group = (self.top_k + 1) / 2;

        for &(group_idx, _) in &top_groups {
            let start = group_idx * self.experts_per_group;
            let end = start + self.experts_per_group;

            // Need a slice equivalent for Array2
            // Since we implemented basic tensor, we can use from for Array2 slices
            let group_expert_weights = Tensor::from(
                self.expert_weights.data.slice(ndarray::s![start..end, ..]).to_owned()
            );

            let expert_logits = token.matmul(&group_expert_weights.t());
            let expert_logits = expert_logits.slice_row(0).to_vec();

            let top_experts = self.top_k_indices(&expert_logits, experts_per_group);

            for (idx, weight) in top_experts {
                let expert_id = group_idx * self.experts_per_group + idx;
                if seen.insert(expert_id) {
                    expert_indices.push(RoutingIndex { expert_id, weight });
                }
            }
        }

        expert_indices.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));
        expert_indices.truncate(self.top_k);

        while expert_indices.len() < self.top_k {
            let random_expert = rand::random::<usize>() % (self.num_groups * self.experts_per_group);
            if seen.insert(random_expert) {
                expert_indices.push(RoutingIndex {
                    expert_id: random_expert,
                    weight: 0.01,
                });
            }
        }

        expert_indices
    }

    fn top_k_indices(&self, values: &[f32], k: usize) -> Vec<(usize, f32)> {
        let mut indexed: Vec<_> = values.iter().enumerate().map(|(i, &v)| (i, v)).collect();
        indexed.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        indexed
            .into_iter()
            .take(k)
            .collect()
    }

    pub fn num_parameters(&self) -> usize {
        self.num_groups * self.hidden_size
            + self.num_groups * self.experts_per_group * self.hidden_size
    }
}
