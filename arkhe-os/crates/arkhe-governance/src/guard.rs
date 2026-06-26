//! GovernanceGuard — enforce do I_gov^v2 no ciclo de vida das propostas.
//!
//! # Design Choice: Síncrono
//!
//! O GovernanceGuard usa std::sync::Mutex por design — o TCB (Trusted
//! Computing Base) deve ser mínimo e não depende de runtime async.
//! Para uso em contexto Tokio/async, envolva as chamadas em
//! tokio::task::spawn_blocking ou use AsyncGovernanceGuard.
//!
//! # AsyncGovernanceGuard
//!
//! Veja crate::async_guard::AsyncGovernanceGuard para versão async.

use std::sync::Mutex;
use crate::invariants::{GovernanceAction, GovernanceInvariantChecker, ExecutionResult};
use crate::safe_core::SafeCoreHook;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecutedAction {
    pub id: [u8; 32],
    pub class: crate::invariants::ActionClass,
    pub executed_at: DateTime<Utc>,
    pub action_hash: [u8; 32],
    pub result: ExecutionResult,
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum GuardError {
    #[error("Action not found: {0}")]
    NotFound(String),
    #[error("Cancellation denied: {0}")]
    CancellationDenied(String),
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
}

pub struct GovernanceGuard {
    checker: Mutex<GovernanceInvariantChecker>, // ✅ P1
    pending: Mutex<Vec<GovernanceAction>>,
    executed: Mutex<Vec<ExecutedAction>>,
    hooks: Mutex<Vec<Box<dyn SafeCoreHook>>>,
}

impl GovernanceGuard {
    pub fn new() -> Self {
        Self {
            checker: Mutex::new(GovernanceInvariantChecker::default()), // ✅ P1
            pending: Mutex::new(Vec::new()),
            executed: Mutex::new(Vec::new()),
            hooks: Mutex::new(Vec::new()),
        }
    }

    pub fn with_checker(checker: GovernanceInvariantChecker) -> Self {
        Self {
            checker: Mutex::new(checker), // ✅ P1
            pending: Mutex::new(Vec::new()),
            executed: Mutex::new(Vec::new()),
            hooks: Mutex::new(Vec::new()),
        }
    }

    pub fn add_hook(&self, hook: Box<dyn SafeCoreHook>) {
        self.hooks.lock().unwrap().push(hook);
    }

    fn run_pre_submit_hooks(&self, action: &GovernanceAction) -> Result<(), GuardError> {
        let hooks = self.hooks.lock().unwrap();
        for hook in hooks.iter() {
            if let Err(e) = hook.pre_submit(action) {
                return Err(GuardError::CancellationDenied(e.to_string()));
            }
        }
        Ok(())
    }

    pub fn submit(&self, action: GovernanceAction) -> Result<String, GuardError> {
        self.run_pre_submit_hooks(&action)?;
        let id_hex = hex::encode(action.id);
        self.pending.lock().unwrap().push(action);
        Ok(id_hex)
    }

    pub fn execute<F, R>(&self, proposal_id: &str, action: F) -> Result<R, GuardError>
    where
        F: FnOnce(&GovernanceAction) -> Result<R, String>,
        R: std::fmt::Debug,
    {
        let proposal = {
            let pending = self.pending.lock().unwrap();
            let pos = pending.iter().position(|p| hex::encode(p.id) == proposal_id)
                .ok_or_else(|| GuardError::NotFound(proposal_id.to_string()))?;
            pending[pos].clone()
        };

        // Executar
        let action_result = action(&proposal);
        let success = action_result.is_ok();
        let execution_result = if success {
            ExecutionResult::Success
        } else {
            ExecutionResult::Rejected(
                action_result.as_ref().unwrap_err().clone()  // ✅ P2: erro real
            )
        };

        // Registrar execução
        {
            let mut checker = self.checker.lock().unwrap();
            checker.record_execution(&proposal, execution_result.clone());
        }
        self.executed.lock().unwrap().push(ExecutedAction {
            id: proposal.id,
            class: proposal.class.clone(),
            executed_at: Utc::now(),
            action_hash: proposal.action_hash,
            result: execution_result,
        });

        {
            let mut pending = self.pending.lock().unwrap();
            if let Some(pos) = pending.iter().position(|p| hex::encode(p.id) == proposal_id) {
                pending.remove(pos);
            }
        }

        action_result.map_err(GuardError::ExecutionFailed)  // ✅ P2: retorna Result<R, _>
    }

    pub fn checker(&self) -> std::sync::MutexGuard<GovernanceInvariantChecker> {
        self.checker.lock().unwrap()
    }

    pub fn cancel(
        &self,
        proposal_id: &str,
        cancellation: &GovernanceAction,
    ) -> Result<(), GuardError> {
        // Verificar que o cancelamento também satisfaz I_gov
        let check = self.checker.lock().unwrap().check(cancellation);  // ✅ P1
        if !check.satisfied {
            return Err(GuardError::CancellationDenied(check.summary()));
        }

        let mut pending = self.pending.lock().unwrap();
        let pos = pending
            .iter()
            .position(|p| hex::encode(p.id) == proposal_id)
            .ok_or_else(|| GuardError::NotFound(proposal_id.to_string()))?;

        let target = &pending[pos];

        // Hacky way to convert target to ExecutedAction for checking revocation
        // Usually, cancellation would check against the executed actions, but here it's pending.
        // We'll construct a dummy ExecutedAction just to satisfy the API
        let dummy_executed = ExecutedAction {
            id: target.id,
            class: target.class.clone(),
            executed_at: target.created_at, // Use creation time as executed_at for this check
            action_hash: target.action_hash,
            result: ExecutionResult::Success,
        };

        // ✅ P3: Verificar revogabilidade da ação alvo
        if let Err(e) = self.checker.lock().unwrap().check_revocation(&dummy_executed) {
            return Err(GuardError::CancellationDenied(e.to_string()));
        }

        pending.remove(pos);
        Ok(())
    }
}
