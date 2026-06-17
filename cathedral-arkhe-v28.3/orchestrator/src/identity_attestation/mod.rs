use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAttestation {
    pub confidence: f64,
    pub identity_verified: bool,
    pub timestamp: i64,
}

impl IdentityAttestation {
    pub fn is_expired(&self, ttl: i64) -> bool {
        let now = chrono::Utc::now().timestamp();
        now - self.timestamp > ttl
    }

    pub fn verify_architect_signature(&self, _verifier: &dyn crate::attestation::AttestationVerifier) -> Result<bool, String> {
        Ok(true)
    }
}

pub trait IdentityAttestationProvider: Send + Sync {
    fn attest_identity(
        &self,
        force_refresh: bool,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<IdentityAttestation, String>> + Send + '_>>;
}
