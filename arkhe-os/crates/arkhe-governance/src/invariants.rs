use chrono::{DateTime, Utc, Duration};
use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionClass {
    Critical,
    Operational,
    Other,
}

impl ActionClass {
    pub fn name(&self) -> &'static str {
        match self {
            ActionClass::Critical => "Critical",
            ActionClass::Operational => "Operational",
            ActionClass::Other => "Other",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceAction {
    pub id: [u8; 32],
    pub class: ActionClass,
    pub description: String,
    pub proposer_did: String,
    pub created_at: DateTime<Utc>,
    pub requested_delay: std::time::Duration,
    pub votes_for: HashSet<String>,
    pub votes_against: HashSet<String>,
    pub action_hash: [u8; 32],
    pub revokes: Option<[u8; 32]>,
}

use std::sync::atomic::{AtomicU64, Ordering};

static ACTION_COUNTER: AtomicU64 = AtomicU64::new(0);

impl GovernanceAction {
    pub fn new(
        class: ActionClass,
        description: String,
        proposer_did: String,
        requested_delay: std::time::Duration,
        action_hash: [u8; 32],
    ) -> Self {
        let nonce = ACTION_COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut hasher = blake3::Hasher::new();
        hasher.update(&nonce.to_le_bytes());
        hasher.update(class.name().as_bytes());
        hasher.update(proposer_did.as_bytes());
        hasher.update(&action_hash);

        Self {
            id: *hasher.finalize().as_bytes(),  // ✅ P7: ID único por ação
            class,
            description,
            proposer_did,
            created_at: Utc::now(),
            requested_delay,
            votes_for: HashSet::new(),
            votes_against: HashSet::new(),
            action_hash,
            revokes: None,
        }
    }

    pub fn canonical_hash(&self) -> [u8; 32] {
        let bytes = serde_json::to_vec(self).expect("serialize");
        blake3::hash(&bytes).into()  // ✅ P5: consistente com resto do codebase
    }

    pub fn earliest_execution(&self) -> DateTime<Utc> {
        let delay_secs = self.requested_delay.as_secs() as i64;
        self.created_at + Duration::seconds(delay_secs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionResult {
    Success,
    Rejected(String),
}

pub struct CheckResult {
    pub satisfied: bool,
    pub reasons: Vec<String>,
}

impl CheckResult {
    pub fn summary(&self) -> String {
        self.reasons.join(", ")
    }
}

pub const REVOKE_WINDOW: std::time::Duration = std::time::Duration::from_secs(3600);

#[derive(Default)]
pub struct GovernanceInvariantChecker {
    // fields
}

impl GovernanceInvariantChecker {
    pub fn record_execution(&mut self, action: &GovernanceAction, result: ExecutionResult) {
        // record
    }

    pub fn check(&self, action: &GovernanceAction) -> CheckResult {
        CheckResult { satisfied: true, reasons: vec![] }
    }

    pub fn check_revocation(&self, target: &crate::guard::ExecutedAction) -> Result<(), String> {
        let elapsed = Utc::now()
            .signed_duration_since(target.executed_at).to_std().unwrap_or(std::time::Duration::ZERO);


        let revoke_window = REVOKE_WINDOW;

        if elapsed > revoke_window {
            return Err("Revocation window expired".into());
        }
        Ok(())
    }
}
