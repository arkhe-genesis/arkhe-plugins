use std::sync::Arc;
use anyhow::Result;

use super::trajectory_store::TrajectoryStore;
use super::tool_simulator::ToolSimulator;
use crate::stubs::{CathedralAgent, GeometricPolicyEngine, SimulationReport, SimulationResult, AgentRole};

pub struct DeploymentSimulationRunner {
    candidate_agent: Arc<dyn CathedralAgent + Send + Sync>,
    tool_simulator: Arc<ToolSimulator>,
    trajectory_store: Arc<TrajectoryStore>,
    policy_engine: Arc<GeometricPolicyEngine>,
}

impl DeploymentSimulationRunner {
    pub fn new(
        candidate_agent: Arc<dyn CathedralAgent + Send + Sync>,
        tool_simulator: Arc<ToolSimulator>,
        trajectory_store: Arc<TrajectoryStore>,
        policy_engine: Arc<GeometricPolicyEngine>,
    ) -> Self {
        Self { candidate_agent, tool_simulator, trajectory_store, policy_engine }
    }

    pub async fn run_simulation(
        &self,
        num_trajectories: usize,
    ) -> Result<SimulationReport> {
        let trajectories = self.trajectory_store.sample_trajectories(
            num_trajectories,
            chrono::Duration::days(30),
        ).await?;

        let mut results = Vec::new();
        for traj in trajectories {
            let candidate_response = self.candidate_agent.run(&traj.goal).await?;

            let simulated_tools = self.tool_simulator.simulate_tool_calls(
                &candidate_response,
                &traj.context_embedding,
            ).await?;

            let violation = self.policy_engine.authorize(
                AgentRole::Specialist,
                &traj.goal,
                &candidate_response.final_answer,
                None,
                None,
            ).await;

            results.push(SimulationResult {
                trajectory_id: traj.id,
                candidate_response,
                simulated_tools,
                violation: violation.err(),
            });
        }

        self.estimate_risk_rates(&results).await
    }

    async fn estimate_risk_rates(&self, results: &[SimulationResult]) -> Result<SimulationReport> {
        let total = results.len();
        if total == 0 {
            return Ok(SimulationReport {
                total_trajectories: 0,
                violation_rate: 0.0,
                violation_types: vec![],
                confidence_interval: (0.0, 0.0),
                causal_fidelity_score: 0.0,
            });
        }
        let violations: Vec<_> = results.iter()
            .filter(|r| r.violation.is_some())
            .collect();

        Ok(SimulationReport {
            total_trajectories: total,
            violation_rate: violations.len() as f32 / total as f32,
            violation_types: self.categorize_violations(violations),
            confidence_interval: (0.8, 1.2),
            causal_fidelity_score: 0.5,
        })
    }

    fn categorize_violations(&self, _violations: Vec<&SimulationResult>) -> Vec<String> {
        vec![]
    }
}
