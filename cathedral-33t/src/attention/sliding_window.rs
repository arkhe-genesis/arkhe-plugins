//! Sliding Window Attention

use crate::tensor::Tensor;
use crate::config::AttentionConfig;

pub struct SlidingWindowAttention {
    pub window_size: usize,
}

impl SlidingWindowAttention {
    pub fn new(config: &AttentionConfig) -> Self {
        Self {
            window_size: config.sliding_window_size,
        }
    }

    pub fn forward(&self, x: &Tensor, _kv_cache: Option<&Tensor>) -> Tensor {
        let seq_len = x.nrows();
        if seq_len <= self.window_size {
            return x.clone();
        }

        // Simplified
        let start = seq_len - self.window_size;
        Tensor::from(x.data.slice(ndarray::s![start..seq_len, ..]).to_owned())
    }
}
