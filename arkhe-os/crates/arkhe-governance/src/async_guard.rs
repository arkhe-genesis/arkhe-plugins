//! AsyncGovernanceGuard — versão async do GovernanceGuard.
//!
//! ✅ P9: Fornece alternativa async para contextos Tokio.

use crate::guard::{GovernanceGuard, GuardError};
use crate::invariants::GovernanceAction;
use std::sync::Arc;

/// Versão async do GovernanceGuard.
pub struct AsyncGovernanceGuard {
    inner: Arc<tokio::sync::Mutex<GovernanceGuard>>,
}

impl AsyncGovernanceGuard {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(tokio::sync::Mutex::new(GovernanceGuard::new())),
        }
    }

    pub async fn submit(&self, action: GovernanceAction) -> Result<String, GuardError> {
        let guard = self.inner.lock().await;
        guard.submit(action)
    }

    pub async fn execute<F, R>(
        &self,
        proposal_id: &str,
        action: F,
    ) -> Result<R, GuardError>
    where
        F: FnOnce(&GovernanceAction) -> Result<R, String> + Send + 'static,
        R: Send + 'static + std::fmt::Debug,
    {
        let guard = self.inner.lock().await;
        // In a real implementation we would do something more sophisticated,
        // but here we just delegate.
        guard.execute(proposal_id, action)
    }
}
