use ndarray::{Array1, ArrayView1};
use std::collections::HashMap;

use super::causal_inner_product::CovarianceMatrix;
use super::concept_directions::ConceptCatalog;

#[derive(Debug, Clone)]
pub struct SteeringVector {
    pub concept: String,
    pub vector: Array1<f32>,
    pub intensity: f32,
}

pub struct SteeringFactory {
    cov: CovarianceMatrix,
    catalog: ConceptCatalog,
    cache: HashMap<String, Array1<f32>>,
}

impl SteeringFactory {
    pub fn new(cov: CovarianceMatrix, catalog: ConceptCatalog) -> Self {
        Self {
            cov,
            catalog,
            cache: HashMap::new(),
        }
    }

    pub fn get_steering_vector(
        &mut self,
        concept: &str,
        intensity: f32,
    ) -> Result<Array1<f32>, String> {
        if let Some(v) = self.cache.get(concept) {
            let mut result = v.clone();
            result = result * intensity;
            return Ok(result);
        }

        let dir = self.catalog.get_direction(concept)
            .ok_or_else(|| format!("Conceito '{}' não encontrado", concept))?;

        let steering = dir.clone();
        self.cache.insert(concept.to_string(), steering.clone());

        Ok(steering * intensity)
    }

    pub fn get_orthogonal_steering(
        &mut self,
        concept: &str,
        avoid_concepts: &[&str],
        intensity: f32,
    ) -> Result<Array1<f32>, String> {
        let mut steering = self.get_steering_vector(concept, 1.0)?;

        for avoid in avoid_concepts {
            if let Some(avoid_dir) = self.catalog.get_direction(avoid) {
                let projection = self.cov.causal_project(&steering.view(), &avoid_dir.view());
                steering = steering - &projection;
            }
        }

        let norm = self.cov.causal_norm(&steering.view());
        if norm > 1e-9 {
            steering = steering / norm * intensity;
        }

        Ok(steering)
    }
}
