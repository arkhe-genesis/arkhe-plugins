use crate::evolution::sepl::AutogenesisOperator;
use crate::evolution::sandbox::WasiPreview2Sandbox;
pub struct EvolutionPipeline {}
impl EvolutionPipeline {
    pub fn new(_operator: Box<AutogenesisOperator>, _sandbox: WasiPreview2Sandbox, _version_manager: (), _max_retries: usize) -> Self { Self {} }
}
