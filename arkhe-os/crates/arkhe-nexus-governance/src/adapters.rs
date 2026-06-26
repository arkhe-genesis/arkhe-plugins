//! Adaptadores para integrar GovernanceGuard no NEXUS/Safe Core.

use arkhe_governance::{
    GovernanceGuard, GovernanceAction, GuardError,
    ExecutionResult,
};
use std::sync::Arc;

/// Adaptador principal — envolve o NEXUS/Safe Core com governança.
///
/// Uso: Substituir o SafeCoreGuard existente por esta struct.
pub struct NexusGovernanceAdapter {
    guard: Arc<GovernanceGuard>
}

impl NexusGovernanceAdapter {
    /// Cria adaptador com GovernanceGuard padrão.
    pub fn new() -> Self {
        Self {
            guard: Arc::new(GovernanceGuard::new()),
        }
    }

    /// Cria adaptador com GovernanceGuard customizado.
    pub fn with_guard(guard: Arc<GovernanceGuard>) -> Self {
        Self { guard }
    }

    /// Executa ação administrativa com verificação de I_gov.
    ///
    /// Esta é a função principal de substituição para o NEXUS.
    /// Deve ser chamada em vez de executar ação diretamente.
    pub fn execute_admin_action<F>(
        &self,
        proposal: GovernanceAction,
        action: F,
    ) -> Result<ExecutionResult, NexusGovernanceError>
    where
        F: FnOnce(&GovernanceAction) -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
    {
        // Passo 1: Submit (verifica I_gov no momento da submissão)
        let id = self.guard.submit(proposal.clone())
            .map_err(NexusGovernanceError::GovernanceError)?;

        // Passo 2: Execute (re-verifica I_gov + timelock no momento da execução)
        let result = self.guard.execute(&id, |p| {
            action(p).map_err(|e| e.to_string())
        }).map_err(NexusGovernanceError::GovernanceError)?;

        // we returned () on success, which means execution succeeded
        Ok(ExecutionResult::Success)
    }

    /// Cancela ação administrativa pendente.
    ///
    /// Requer proposta de cancelamento que também satisfaça I_gov.
    pub fn cancel_admin_action(
        &self,
        proposal_id: &str,
        cancellation_proposal: &GovernanceAction,
    ) -> Result<(), NexusGovernanceError> {
        self.guard.cancel(proposal_id, cancellation_proposal)
            .map_err(NexusGovernanceError::GovernanceError)
    }

    /// Hash do audit trail (para anchoring no WormGraph).
    pub fn audit_hash(&self) -> [u8; 32] {
        [0u8; 32] // Placeholder for now, missing in guard api
    }

    /// Verifica se uma ação específica está no audit trail.
    pub fn is_action_audited(&self, _proposal_id: &str) -> bool {
        false // Placeholder for now
    }
}

/// Erros da bridge NEXUS <-> Governance.
#[derive(Debug, thiserror::Error)]
pub enum NexusGovernanceError {
    #[error("Governance operation failed: {0}")]
    GovernanceError(#[from] GuardError),

    #[error("NEXUS action failed: {0}")]
    NexusActionFailed(String),

    #[error("Action not found in audit trail: {0}")]
    ActionNotAudited(String),
}
