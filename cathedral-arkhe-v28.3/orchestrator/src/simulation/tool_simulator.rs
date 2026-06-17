use std::sync::Arc;
use anyhow::Result;
use ndarray::Array1;

use crate::geometry::CausalGeometryService;
use crate::stubs::{LlmClient, ToolResponse};

pub struct ToolSimulator {
    geometry: Arc<CausalGeometryService>,
    llm_client: Arc<dyn LlmClient + Send + Sync>,
}

impl ToolSimulator {
    pub fn new(geometry: Arc<CausalGeometryService>, llm_client: Arc<dyn LlmClient + Send + Sync>) -> Self {
        Self { geometry, llm_client }
    }

    pub async fn simulate_tool_call(
        &self,
        tool_name: &str,
        parameters: &serde_json::Value,
        context_embedding: &Array1<f32>,
        previous_tool_responses: &[ToolResponse],
    ) -> Result<ToolResponse> {
        let _causal_context = self.geometry.project_causal(context_embedding);

        let prompt = format!(
            "Given the following context embedding (causal projection), simulate the response for tool '{}' with parameters: {}. Previous responses: {:?}",
            tool_name, parameters, previous_tool_responses
        );

        let response = self.llm_client.generate(&prompt).await?;

        let response_emb = self.geometry.embed(&response);
        let orthogonality = self.geometry.causal_orthogonality(&context_embedding.view(), &response_emb.view());
        if orthogonality < 0.6 {
            // Regenerate if too close to context
        }

        Ok(ToolResponse { tool_name: tool_name.to_string(), response })
    }

    pub async fn simulate_tool_calls(
        &self,
        _candidate_response: &crate::stubs::CandidateResponse,
        _context_embedding: &Array1<f32>,
    ) -> Result<Vec<ToolResponse>> {
        Ok(vec![])
    }
}
