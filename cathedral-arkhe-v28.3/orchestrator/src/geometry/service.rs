use std::sync::Arc;
use tokio::sync::RwLock;
use ndarray::{Array1, ArrayView1};

use super::causal_inner_product::CovarianceMatrix;
use super::concept_directions::ConceptCatalog;
use super::steering_vectors::SteeringFactory;
use super::subspace_operations::SubspaceOperations;
use super::embedding_bridge::EmbeddingModel;

pub struct CausalGeometryService {
    cov: Arc<CovarianceMatrix>,
    catalog: Arc<RwLock<ConceptCatalog>>,
    steering_factory: Arc<RwLock<SteeringFactory>>,
    subspace_ops: Arc<SubspaceOperations>,
    embedder: Arc<dyn EmbeddingModel + Send + Sync>,
}

impl CausalGeometryService {
    pub fn new(embedder: Arc<dyn EmbeddingModel + Send + Sync>, embedding_dim: usize) -> Self {
        let cov = Arc::new(CovarianceMatrix::identity(embedding_dim));
        let catalog = Arc::new(RwLock::new(ConceptCatalog::new(cov.clone())));
        let steering_factory = Arc::new(RwLock::new(
            SteeringFactory::new((*cov).clone(), (*catalog.blocking_read()).clone())
        ));
        let subspace_ops = Arc::new(SubspaceOperations::new(cov.clone()));

        Self {
            cov,
            catalog,
            steering_factory,
            subspace_ops,
            embedder,
        }
    }

    pub fn embed(&self, text: &str) -> Array1<f32> {
        self.embedder.embed(text)
    }

    pub async fn register_concept(
        &self,
        name: &str,
        positive: &[Array1<f32>],
        negative: &[Array1<f32>],
    ) -> Result<(), String> {
        self.catalog.write().await.register_concept(name, positive, negative)
    }

    pub async fn get_concept_direction(&self, name: &str) -> Option<Array1<f32>> {
        self.catalog.read().await.get_direction(name)
    }

    pub async fn concept_orthogonality(&self, a: &str, b: &str) -> Option<f32> {
        self.catalog.read().await.orthogonality(a, b)
    }

    pub async fn get_steering_vector(&self, concept: &str, intensity: f32) -> Result<Array1<f32>, String> {
        self.steering_factory.write().await.get_steering_vector(concept, intensity)
    }

    pub async fn get_orthogonal_steering(
        &self,
        concept: &str,
        avoid: &[&str],
        intensity: f32,
    ) -> Result<Array1<f32>, String> {
        self.steering_factory.write().await.get_orthogonal_steering(concept, avoid, intensity)
    }

    pub fn causal_dot(&self, a: &ArrayView1<f32>, b: &ArrayView1<f32>) -> f32 {
        self.cov.causal_dot(a, b)
    }

    pub fn causal_similarity(&self, a: &ArrayView1<f32>, b: &ArrayView1<f32>) -> f32 {
        self.cov.causal_similarity(a, b)
    }

    pub fn causal_orthogonality(&self, a: &ArrayView1<f32>, b: &ArrayView1<f32>) -> f32 {
        self.cov.causal_orthogonality(a, b)
    }

    pub fn causal_project(&self, v: &ArrayView1<f32>, u: &ArrayView1<f32>) -> Array1<f32> {
        self.cov.causal_project(v, u)
    }

    pub fn causal_rank(&self, _v: &ArrayView1<f32>) -> usize {
        4
    }

    pub fn project_causal(&self, v: &Array1<f32>) -> Array1<f32> {
        self.subspace_ops.project_to_known_subspace(v)
    }

    pub fn causal_weight(&self, v: &Array1<f32>) -> f32 {
        self.subspace_ops.causal_weight(v)
    }
}
