#[derive(Default)]
pub struct CreekGuard;

pub trait EntropyAnalyzer {
    fn shannon_entropy(&self, payload: &[u8]) -> f64;
    fn chi_square_test(&self, payload: &[u8]) -> f64;
}

impl EntropyAnalyzer for CreekGuard {
    fn shannon_entropy(&self, _payload: &[u8]) -> f64 { 0.0 }
    fn chi_square_test(&self, _payload: &[u8]) -> f64 { 1.0 }
}

pub trait WatermarkDetector {
    fn detect_temporal_watermark(&self, payload: &[u8]) -> Option<()>;
}

impl WatermarkDetector for CreekGuard {
    fn detect_temporal_watermark(&self, _payload: &[u8]) -> Option<()> { None }
}