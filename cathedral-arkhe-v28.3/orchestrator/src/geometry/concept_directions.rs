use ndarray::{Array1, ArrayView1};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::debug;

use super::causal_inner_product::CovarianceMatrix;

#[derive(Debug, Clone)]
pub struct ConceptDirection {
    pub name: String,
    pub vector: Array1<f32>,
    pub confidence: f32,
    pub sample_count: usize,
}

#[derive(Clone)]
pub struct ConceptCatalog {
    concepts: HashMap<String, ConceptDirection>,
    cov: Arc<CovarianceMatrix>,
}

impl ConceptCatalog {
    pub fn new(cov: Arc<CovarianceMatrix>) -> Self {
        Self {
            concepts: HashMap::new(),
            cov,
        }
    }

    pub fn register_concept(
        &mut self,
        name: &str,
        positive_examples: &[Array1<f32>],
        negative_examples: &[Array1<f32>],
    ) -> Result<(), String> {
        if positive_examples.is_empty() || negative_examples.is_empty() {
            return Err("Exemplos insuficientes".into());
        }

        let mut pos_mean: Array1<f32> = Array1::zeros(self.cov.dimension);
        for ex in positive_examples { pos_mean = pos_mean + ex; }
        pos_mean = pos_mean / (positive_examples.len() as f32);

        let mut neg_mean: Array1<f32> = Array1::zeros(self.cov.dimension);
        for ex in negative_examples { neg_mean = neg_mean + ex; }
        neg_mean = neg_mean / (negative_examples.len() as f32);

        let direction: Array1<f32> = pos_mean - &neg_mean;

        let norm = self.cov.causal_norm(&direction.view());
        if norm < 1e-9 {
            return Err("Direção do conceito tem norma zero".into());
        }
        let normalized = direction / norm;

        self.concepts.insert(
            name.to_string(),
            ConceptDirection {
                name: name.to_string(),
                vector: normalized,
                confidence: 1.0,
                sample_count: positive_examples.len(),
            },
        );

        debug!("Conceito '{}' registrado com {} amostras", name, positive_examples.len());
        Ok(())
    }

    pub fn get_direction(&self, name: &str) -> Option<Array1<f32>> {
        self.concepts.get(name).map(|c| c.vector.clone())
    }

    pub fn orthogonality(&self, concept_a: &str, concept_b: &str) -> Option<f32> {
        let dir_a = self.get_direction(concept_a)?;
        let dir_b = self.get_direction(concept_b)?;
        Some(self.cov.causal_orthogonality(&dir_a.view(), &dir_b.view()))
    }
}
