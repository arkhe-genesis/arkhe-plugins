use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDescriptor {
    pub name: String,
    pub description: String,
    pub blocking: bool,
}

pub struct AttestationManager {}

impl AttestationManager {
    pub fn new() -> Self {
        AttestationManager {}
    }

    pub async fn list_active_policies(&self) -> Result<Vec<PolicyDescriptor>, String> {
        // Em produção: consultar o GeometricPolicyEngine.
        // Por enquanto, retorna políticas padrão.
        Ok(vec![
            PolicyDescriptor {
                name: "pii_prohibition".to_string(),
                description: "Proíbe a saída de PII em respostas".to_string(),
                blocking: true,
            },
            PolicyDescriptor {
                name: "steering_safety".to_string(),
                description: "Garante que steering vectors não afetem segurança".to_string(),
                blocking: true,
            },
            PolicyDescriptor {
                name: "no_representation_collapse".to_string(),
                description: "Evita colapso de conceitos em embeddings".to_string(),
                blocking: false,
            },
        ])
    }

    pub async fn store_execution(&self, attestation: &crate::attestation::ExecutionAttestation, provenance: &str) -> Result<(), String> {
        Ok(())
    }

    pub async fn get_execution(&self, id: &str) -> Result<Option<crate::attestation::ExecutionAttestation>, String> {
        Ok(None)
    }

    pub async fn validate_execution(&self, attestation: &crate::attestation::ExecutionAttestation) -> Result<bool, String> {
        Ok(true)
    }
}
