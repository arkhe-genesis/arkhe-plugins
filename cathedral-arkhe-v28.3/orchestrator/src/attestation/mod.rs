pub mod manager;

pub use manager::{AttestationManager, PolicyDescriptor};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionAttestation {
    pub id: String,
    pub policy_compliance: bool,
    pub policy_attestation_id: Option<String>,
}

impl ExecutionAttestation {
    pub fn is_policy_compliant(&self) -> bool {
        self.policy_compliance
    }

    pub fn policy_attestation_id(&self) -> Option<String> {
        self.policy_attestation_id.clone()
    }
}

pub trait AttestationProvider: Send + Sync {
    fn run_authorized(
        &self,
        workload: &str,
        cost_cap: Option<f64>,
        identity: &crate::identity_attestation::IdentityAttestation,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ExecutionAttestation, String>> + Send + '_>>;
}

pub trait AttestationVerifier: Send + Sync {
    fn verify(&self, attestation: &crate::identity_attestation::IdentityAttestation) -> Result<bool, String>;
}

pub struct CathedralComputeProvider {}

impl CathedralComputeProvider {
    pub fn new() -> Self {
        CathedralComputeProvider {}
    }
}

impl AttestationProvider for CathedralComputeProvider {
    fn run_authorized(
        &self,
        _workload: &str,
        _cost_cap: Option<f64>,
        _identity: &crate::identity_attestation::IdentityAttestation,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ExecutionAttestation, String>> + Send + '_>> {
        Box::pin(async {
            Ok(ExecutionAttestation {
                id: "dummy".to_string(),
                policy_compliance: true,
                policy_attestation_id: None,
            })
        })
    }
}
