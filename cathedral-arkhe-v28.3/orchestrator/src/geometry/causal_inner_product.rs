use ndarray::{Array1, Array2, ArrayView1};
use nalgebra as na;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct CovarianceMatrix {
    pub cov: Array2<f32>,
    pub cov_inv: Array2<f32>,
    pub dimension: usize,
}

impl CovarianceMatrix {
    pub fn from_vectors(vectors: &[Array1<f32>]) -> Self {
        let n = vectors.len();
        if n == 0 {
            return Self::identity(vectors.first().map(|v| v.len()).unwrap_or(768));
        }
        let d = vectors[0].len();

        let mut mean: Array1<f32> = Array1::zeros(d);
        for v in vectors { mean = mean + v; }
        mean = mean / (n as f32);

        let mut cov: Array2<f32> = Array2::zeros((d, d));
        for v in vectors {
            let centered: Array1<f32> = v - &mean;
            for i in 0..d {
                for j in 0..d {
                    cov[[i, j]] += centered[i] * centered[j];
                }
            }
        }
        cov = cov / ((n - 1) as f32);

        let lambda = 1e-6;
        for i in 0..d { cov[[i, i]] += lambda; }

        let cov_na = na::DMatrix::from_vec(d, d, cov.iter().cloned().collect());
        let cov_inv_na = cov_na.clone().try_inverse()
            .unwrap_or_else(|| {
                warn!("Matriz de covariância singular. Usando pseudo-inversa.");
                cov_na.pseudo_inverse(1e-6).unwrap()
            });

        let cov_inv = Array2::from_shape_vec((d, d), cov_inv_na.as_slice().to_vec())
            .unwrap();

        Self { cov, cov_inv, dimension: d }
    }

    pub fn identity(d: usize) -> Self {
        let mut cov: Array2<f32> = Array2::zeros((d, d));
        let mut cov_inv: Array2<f32> = Array2::zeros((d, d));
        for i in 0..d {
            cov[[i, i]] = 1.0;
            cov_inv[[i, i]] = 1.0;
        }
        Self { cov, cov_inv, dimension: d }
    }

    pub fn causal_dot(&self, a: &ArrayView1<f32>, b: &ArrayView1<f32>) -> f32 {
        let temp = self.cov_inv.dot(b);
        a.dot(&temp)
    }

    pub fn causal_norm(&self, v: &ArrayView1<f32>) -> f32 {
        self.causal_dot(v, v).sqrt()
    }

    pub fn causal_similarity(&self, a: &ArrayView1<f32>, b: &ArrayView1<f32>) -> f32 {
        let dot = self.causal_dot(a, b);
        let na = self.causal_norm(a);
        let nb = self.causal_norm(b);
        if na < 1e-9 || nb < 1e-9 { return 0.0; }
        dot / (na * nb)
    }

    pub fn causal_orthogonality(&self, a: &ArrayView1<f32>, b: &ArrayView1<f32>) -> f32 {
        1.0 - self.causal_similarity(a, b).abs()
    }

    pub fn causal_project(&self, v: &ArrayView1<f32>, u: &ArrayView1<f32>) -> Array1<f32> {
        let dot_vu = self.causal_dot(v, u);
        let dot_uu = self.causal_dot(u, u);
        if dot_uu < 1e-9 {
            return Array1::zeros(v.len());
        }
        u.to_owned() * (dot_vu / dot_uu)
    }

    pub fn causal_orthogonalize(&self, v: &ArrayView1<f32>, u: &ArrayView1<f32>) -> Array1<f32> {
        v.to_owned() - &self.causal_project(v, u)
    }
}
